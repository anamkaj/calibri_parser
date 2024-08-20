use dotenv::dotenv;
use reqwest::{header, Client, Method, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres};
use crate::{
    db::insert_client_info_table::{add_info_client, update_date_client, Site},
    utils::utils::timer,
};

use super::info_client_json::JsonNewMultiTracking;

#[derive(Default, Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct NewMultiTracking {
    #[serde(rename = "type")]
    pub site_id: i64,
    pub type_field: String,
    pub license_start: String,
    pub license_end: String,
    pub numbers: Vec<String>,
    pub not_enough_money: bool,
}

impl NewMultiTracking {
    //*Получение даты размещения и есть ли продление для активных клиентов */
    //*______________________________________________________________________ */
    pub async fn get_date(
        pool: Pool<Postgres>,
        client_list: &Vec<Site>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut arr_tracking: Vec<NewMultiTracking> = Vec::new();

        // let id_client: Vec<Site> = Site::get_id_client(pool.clone())
        //     .await
        //     .expect("Failed to retrieve list of active clients");

        for data in client_list {
            if data.active == "true" {
                println!("{} {}", data.site_id, data.sitename);
                let _ = timer(4).await;
                let array_firs_element: JsonNewMultiTracking =
                    request_tracking(data.site_id).await?;

                let result: NewMultiTracking = NewMultiTracking {
                    type_field: array_firs_element.type_field.trim().to_string(),
                    license_start: array_firs_element
                        .license_start
                        .clone()
                        .unwrap_or_default()
                        .trim()
                        .to_string(),
                    license_end: array_firs_element
                        .license_end
                        .clone()
                        .unwrap_or_default()
                        .trim()
                        .to_string(),
                    numbers: array_firs_element.numbers.clone().unwrap_or_default(),
                    not_enough_money: array_firs_element
                        .not_enough_money
                        .clone()
                        .unwrap_or_default(),

                    // site_id: data.site_id,
                    site_id: data.site_id,
                };

                arr_tracking.push(result);
            }
        }

        let _ = update_date_client(&arr_tracking, pool.clone()).await?;

        let _ = add_info_client(&arr_tracking, pool).await?;

        Ok(())
    }
}

async fn request_tracking(
    site_id: i64,
) -> Result<JsonNewMultiTracking, Box<dyn std::error::Error>> {
    dotenv().ok();
    let session: String = std::env::var("COOKIES_SESSION").unwrap();

    let url: String = format!(
        "https://in.callibri.ru/services/project/{}/my_services.json",
        site_id
    );
    let client: Client = reqwest::Client::builder().build()?;

    let mut headers: header::HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert("host", "in.callibri.ru".parse()?);
    headers.insert("Cookie", session.parse()?);

    let request: RequestBuilder = client.request(Method::GET, url.clone()).headers(headers);

    let response: Response = request
        .send()
        .await
        .expect("Ошибка получения дат размещения");
    let body: String = response.text().await?;

    let json_des: Vec<JsonNewMultiTracking> = serde_json::from_str(&body)
        .expect(&format!("Ошибка десериализации дат размещения {}", url));

    let array_firs_element: &JsonNewMultiTracking = &json_des[0];
    Ok(array_firs_element.clone())
}
