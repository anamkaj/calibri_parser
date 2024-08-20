use dotenv::dotenv;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use crate::db::insert_client_info_table::{update_client_table, Site};

//* */ Модель для парсинга JSON
//* */ ______________________________________________________________________
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonGetClientList {
    pub sites: Vec<JsonSite>,
    pub code: i64,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct JsonSite {
    pub site_id: i64,
    pub sitename: String,
    pub domains: Option<String>,
    pub active: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonNewMultiTracking {
    #[serde(rename = "type")]
    pub type_field: String,
    pub license_start: Option<String>,
    pub license_end: Option<String>,
    pub numbers: Option<Vec<String>>,
    pub not_enough_money: Option<bool>,
}

impl JsonNewMultiTracking {
    //*Получение через API информации о клиентах , (активные, домены сайта, ID)  после , добавление в таблицу клиентов */
    pub async fn get_client_list(
        pool: Pool<Postgres>,
    ) -> Result<Vec<Site>, Box<dyn std::error::Error>> {

        dotenv().ok();
        let token: String = std::env::var("TOKEN").unwrap();
        
        let url: String = format!(
            "https://api.callibri.ru/get_sites?user_email=blizko-context1@yandex.ru&user_token={}",
            token
        );
        let request: Response = reqwest::get(url)
            .await
            .expect("Ошибка запроса на получение списка пользователей");

        let body: String = request.text().await?;
        let json_list_client: JsonGetClientList =
            serde_json::from_str(&body).expect("Ошибка десериализации");

        let arr_verif: Vec<Site> =
            JsonNewMultiTracking::client_id_verif(json_list_client.clone()).await;

        //* Обнавление таблиы клиентов в базе */
        let data: Vec<Site> = update_client_table(arr_verif, pool)
            .await
            .expect("Ошибка добавление в базу ");

        Ok(data)
    }

    //*Прогон массива с клиентами , убрираем пробелы и подставляем default значения*/
    //*______________________________________________________________________ */
    pub async fn client_id_verif(json_list: JsonGetClientList) -> Vec<Site> {
        let client: Vec<JsonSite> = json_list.sites;
        let mut arr_verif_client: Vec<Site> = Vec::new();

        for cl in client {
            let update_client: Site = Site {
                domains: cl.domains.unwrap_or_default().trim().to_string(),
                site_id: cl.site_id,
                sitename: cl.sitename.trim().to_string(),
                active: cl.active.trim().to_string(),
            };
            arr_verif_client.push(update_client);
        }

        println!("Массив клиентов проверен");
        arr_verif_client
    }
}
