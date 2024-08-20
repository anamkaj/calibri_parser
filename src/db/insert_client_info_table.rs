use crate::{models::tracking::NewMultiTracking, utils::utils::date_transform};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres};

#[derive(Default, Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct Site {
    pub site_id: i64,
    pub sitename: String,
    pub domains: String,
    pub active: String,
}

//* Добовление новых клиентов и их обновление в таблице */
pub async fn update_client_table(
    data: Vec<Site>,
    pool: Pool<Postgres>,
) -> Result<Vec<Site>, Box<dyn std::error::Error>> {
    let mut result: Vec<Site> = Vec::new();

    for client in &data {
        println!("Обновление базы клиентов {}", client.sitename);
        let update_and_add_client_ = "
                INSERT INTO client_calibri (site_id, sitename, domains, active)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (site_id) 
                DO UPDATE SET
                sitename = EXCLUDED.sitename,
                domains = EXCLUDED.domains,
                active = EXCLUDED.active
                RETURNING site_id, sitename, domains, active;";

        let res: Site = sqlx::query_as(&update_and_add_client_)
            .bind(&client.site_id)
            .bind(&client.sitename.trim().to_string())
            .bind(&client.domains)
            .bind(&client.active)
            .fetch_one(&pool)
            .await?;

        result.push(res);
    }

    Ok(result)
}

//* Обновление дат и статуса продления  */
pub async fn update_date_client(
    data: &Vec<NewMultiTracking>,
    pool: Pool<Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let update_client = "
        UPDATE client_calibri
        SET 
        license_start = $1,
        license_end = $2,
        not_enough_money = $3
        WHERE site_id = $4
        AND (license_start <> $1 OR license_end <> $2 OR not_enough_money <> $3);";

    for client in data {
        let start_date: String = date_transform(client.license_start.clone()).await;

        let end_date: String = date_transform(client.license_end.clone()).await;

        sqlx::query(&update_client)
            .bind(&start_date)
            .bind(&end_date)
            .bind(client.not_enough_money)
            .bind(client.site_id)
            .execute(&pool)
            .await?;
    }

    Ok(())
}

pub async fn add_info_client(
    multi_tracking: &Vec<NewMultiTracking>,
    pool: Pool<Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    //* */ Добавление телефонов
    let insert_number_table: &str = "INSERT INTO phone (client_calibri_site_id_fk, number) 
        VALUES ($1, $2)
        ON CONFLICT (client_calibri_site_id_fk) 
        DO UPDATE SET 
        number = EXCLUDED.number
        WHERE phone.number <> EXCLUDED.number";

    for client in multi_tracking {
        sqlx::query(&insert_number_table)
            .bind(client.site_id)
            .bind(client.numbers.clone())
            .fetch_all(&pool)
            .await?;
    }

    Ok(())
}
