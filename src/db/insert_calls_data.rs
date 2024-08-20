use sqlx::{Pool, Postgres};

//* */ Финальная модель для залива в таблицы
//* */ ______________________________________________________________________

#[derive(Default, Debug, Clone)]
pub struct Statistic {
    pub calls: Vec<CallsArray>,
    pub emails: Vec<EmailArray>,
    pub site_id: i64,
}

#[derive(Default, Debug, Clone)]
pub struct CallsArray {
    pub id: i64,
    pub date: String,
    pub channel_id: i64,
    pub source: String,
    pub is_lid: bool,
    pub name_type: String,
    pub traffic_type: String,
    pub landing_page: String,
    pub conversations_number: i64,
    pub call_status: String,
}

#[derive(Default, Debug, Clone)]
pub struct EmailArray {
    pub id: i64,
    pub date: String,
    pub source: String,
    pub is_lid: bool,
    pub traffic_type: String,
    pub landing_page: String,
    pub lid_landing: String,
    pub conversations_number: i64,
}

//* Импорт звонков и писем в базу */
//*______________________________________________________________________ */
pub async fn insert_calls_and_email(
    array_calls: Vec<Statistic>,
    pool: Pool<Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Добавление новых звонков");

    //* Распределение по таблицам писем и звонков */
    for data in array_calls {
        if !data.calls.is_empty() {
            // Добавление новых звонков
            let _ = insert_calls(data.calls, data.site_id, pool.clone())
                .await
                .expect("Ошибка добавления звонков");
        }

        if !data.emails.is_empty() {
            // Добавление новых писем
            let _ = insert_email(data.emails, data.site_id, pool.clone())
                .await
                .expect("Ошибка добавления писем");
        }
    }

    Ok(())
}

//* Импорт массива звонков в таблицу */
//*______________________________________________________________________ */
pub async fn insert_calls(
    calls: Vec<CallsArray>,
    id_client: i64,
    pool: Pool<Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    for call in calls {
        let insert_call: &str = "INSERT INTO calls (
            client_calibri_site_id_fk,
            call_id,
            date,
            channel_id,
            is_lid,
            name_type,
            traffic_type,
            landing_page,
            conversations_number,
            call_status,
            source
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
            ON CONFLICT (call_id)
            DO NOTHING";

        sqlx::query(insert_call)
            .bind(id_client)
            .bind(call.id)
            .bind(call.date)
            .bind(call.channel_id)
            .bind(call.is_lid)
            .bind(call.name_type)
            .bind(call.traffic_type)
            .bind(call.landing_page)
            .bind(call.conversations_number)
            .bind(call.call_status)
            .bind(call.source)
            .execute(&pool)
            .await
            .expect("Ошибка добавление звонков клиента ! ");
    }

    Ok(())
}

//* Импорт писем в таблицу */
//*______________________________________________________________________ */
pub async fn insert_email(
    email: Vec<EmailArray>,
    id_client: i64,
    pool: Pool<Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    for em in email {
        let insert_call: &str = "INSERT INTO email (
            client_calibri_site_id_fk,
            email_id,
            date,
            is_lid,
            traffic_type,
            landing_page,
            lid_landing,
            conversations_number,
            source
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
            ON CONFLICT (email_id)
            DO NOTHING";

        sqlx::query(insert_call)
            .bind(id_client)
            .bind(em.id)
            .bind(em.date)
            .bind(em.is_lid)
            .bind(em.traffic_type)
            .bind(em.lid_landing)
            .bind(em.landing_page)
            .bind(em.conversations_number)
            .bind(em.source)
            .execute(&pool)
            .await
            .expect("Ошибка добавление писем клиента !");
    }

    Ok(())
}



