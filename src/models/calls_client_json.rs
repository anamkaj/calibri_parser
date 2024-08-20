use crate::{
    db::{
        insert_calls_data::{insert_calls_and_email, CallsArray, EmailArray, Statistic},
        insert_client_info_table::Site,
    },
    utils::utils::timer,
};
use chrono::{DateTime, Duration as Dur, Local};
use dotenv::dotenv;
use reqwest::Response;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::error::Error;

//* */ Модель для парсинга JSON
//* */ ______________________________________________________________________
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDataCalls {
    pub code: i64,
    #[serde(rename = "channels_statistics")]
    pub channels_statistics: Vec<ChannelsStatistic>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ChannelsStatistic {
    pub calls: Vec<Call>,
    pub emails: Vec<Email>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Call {
    pub id: i64,
    pub date: String,
    pub channel_id: i64,
    pub source: Option<String>,
    pub is_lid: bool,
    pub name_type: Option<String>,
    pub traffic_type: Option<String>,
    pub landing_page: Option<String>,
    pub conversations_number: i64,
    pub call_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Email {
    pub id: i64,
    pub date: String,
    pub source: Option<String>,
    #[serde(rename = "is_lid")]
    pub is_lid: bool,
    #[serde(rename = "traffic_type")]
    pub traffic_type: Option<String>,
    #[serde(rename = "landing_page")]
    pub landing_page: Option<String>,
    #[serde(rename = "lid_landing")]
    pub lid_landing: Option<String>,
    #[serde(rename = "conversations_number")]
    pub conversations_number: i64,
}

//* Получение статистика по звонкам и письмам*/
//*______________________________________________________________________ */
pub async fn get_calls_site(
    count: i64,
    pool: Pool<Postgres>,
    client_list: &Vec<Site>,
) -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let token: String = std::env::var("TOKEN").unwrap();

    let mut array_calls: Vec<Statistic> = Vec::new();

    // *!/ Собираем статистику за последние 3 дня. Максимум 6 дней  */
    let today: DateTime<Local> = Local::now();
    let yesterday: DateTime<Local> = today - Dur::days(count);

    let formatted_date_start: String = yesterday.format("%d.%m.%Y").to_string();
    let formatted_date_end: String = today.format("%d.%m.%Y").to_string();

    println!(
        "Collecting statistics from {:?} to {:?}",
        formatted_date_start, formatted_date_end
    );

    for client in client_list {
        if client.active == "true" {
            println!("Collecting statistics for client: {:?}", client.sitename);
            // Construct API request URL
            let full_url_param: String = format!(
                "https://api.callibri.ru/site_get_statistics?user_email=blizko-context1@yandex.ru&user_token={}&site_id={}&date1={}&date2={}",
                token,
                client.site_id,
                formatted_date_start,
                formatted_date_end
            );

            let request: Response = reqwest::get(full_url_param)
                .await
                .expect("Failed to send API request");

            let body: String = request.text().await?;

            // Deserialize response body into struct
            let json_dese: GetDataCalls =
                serde_json::from_str(&body).expect("Failed to deserialize response");

            if json_dese.code == 200 && !json_dese.channels_statistics.is_empty() {
                for elem in json_dese.channels_statistics {
                    // Retrieve filtered array of calls and emails
                    let calls_array_valid: Vec<CallsArray> = calls_array(elem.calls)
                        .await
                        .expect("Failed to validate calls array");
                    let email_array_valid: Vec<EmailArray> = email_array(elem.emails)
                        .await
                        .expect("Failed to validate emails array");

                    // Add client's call statistics to final array
                    array_calls.push(Statistic {
                        calls: calls_array_valid,
                        emails: email_array_valid,
                        site_id: client.site_id,
                    });
                }
            }

            // Add delay between API requests
            let _ = timer(4).await;
        }
    }

    let _ = insert_calls_and_email(array_calls.clone(), pool).await?;
    Ok(())
}

//* Обработка массива со звонками*/
//*______________________________________________________________________ */
async fn calls_array(calls: Vec<Call>) -> Result<Vec<CallsArray>, Box<dyn Error>> {
    let mut result: Vec<CallsArray> = Vec::new();

    for ca in calls {
        result.push(CallsArray {
            id: ca.id,
            date: ca.date.trim().to_string(),
            channel_id: ca.channel_id,
            source: ca.source.unwrap_or_default().trim().to_string(),
            is_lid: ca.is_lid,
            name_type: ca.name_type.unwrap_or_default().trim().to_string(),
            traffic_type: ca.traffic_type.unwrap_or_default().trim().to_string(),
            landing_page: ca.landing_page.unwrap_or_default().trim().to_string(),
            conversations_number: ca.conversations_number,
            call_status: ca.call_status.unwrap_or_default().trim().to_string(),
        });
    }
    Ok(result)
}

//* Обработка массива с письмами*/
//*______________________________________________________________________ */
async fn email_array(emails: Vec<Email>) -> Result<Vec<EmailArray>, Box<dyn Error>> {
    let mut result: Vec<EmailArray> = Vec::new();

    for em in emails {
        result.push(EmailArray {
            id: em.id,
            date: em.date.trim().to_string(),
            source: em.source.unwrap_or_default().trim().to_string(),
            is_lid: em.is_lid,
            traffic_type: em.traffic_type.unwrap_or_default().trim().to_string(),
            landing_page: em.landing_page.unwrap_or_default().trim().to_string(),
            lid_landing: em.lid_landing.unwrap_or_default().trim().to_string(),
            conversations_number: em.conversations_number,
        });
    }

    Ok(result)
}
