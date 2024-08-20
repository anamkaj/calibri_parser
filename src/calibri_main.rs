use crate::{
    db::insert_client_info_table::Site,
    models::{
        calls_client_json::get_calls_site, info_client_json::JsonNewMultiTracking,
        tracking::NewMultiTracking,
    },
};
use sqlx::{Pool, Postgres};

//* Парссер сервиса калибри , парсит статусы клиентов (активный или отключенный ) и собирает статистику по звонкаи и заявкам + номера тел и даты размещения */
pub async fn calibri_service_data(
    count: i64,
    pool: Pool<Postgres>,
) -> Result<String, Box<dyn std::error::Error>> {
    // //* Добовление новых клиентов и их обновление в таблице */
    let client_list: Vec<Site> = JsonNewMultiTracking::get_client_list(pool.clone()).await?;

    // //*Получение статистики звонков и писем  */
    let _ = get_calls_site(count, pool.clone(), &client_list).await?;

    // //* Добавление дат размешение и номеров подмены  */
    let _ = NewMultiTracking::get_date(pool.clone(), &client_list).await?;

    Ok("Сбор статистики завершен".to_string())
}
