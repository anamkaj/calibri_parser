use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub data: Vec<ClientCalibri>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCalibri {
    pub id: i32,
    pub site_id: i64,
    pub sitename: String,
    pub domains: String,
    pub active: String,
    pub license_start: Option<String>,
    pub license_end: Option<String>,
    pub not_enough_money: Option<bool>,
    pub number: Option<Vec<String>>,
}

impl ClientCalibri {
    pub async fn get_list_client() -> Result<Vec<ClientCalibri>, Box<dyn Error>> {
        let url: &str = "http://localhost:8070/api/status?status=true";

        let client: Client = Client::new();
        let req: reqwest::Response = client.get(url).send().await?;

        let body: String = req.text().await?;

        let json_data: Data = serde_json::from_str(&body).unwrap();
        let data: Vec<ClientCalibri> = json_data.data;

        Ok(data)
    }
}
