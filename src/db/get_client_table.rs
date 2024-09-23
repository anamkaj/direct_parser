use chrono::DateTime;
use chrono::Utc;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Data {
    data: Vec<ClientTableList>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientTableList {
    pub id: i64,
    pub name: String,
    pub url_site: String,
    pub data_end: String,
    pub date_start: String,
    pub region_client: String,
    pub url_crm: String,
    pub pay_company: String,
    pub plan: String,
    pub specific_client: String,
    pub account_manager: String,
    pub specialist_ads: String,
    pub status_ads: bool,
    pub status_client: bool,
    pub count_metrika: i64,
    pub direct_login: String,
    pub call_tracking_id: i64,
    pub center_accounting: String,
    pub plan_click: i64,
    pub percentage_lead: f32,
    pub created_at: DateTime<Utc>,
    pub uniq_id: String,
}

impl ClientTableList {
    pub async fn get_client_list() -> Result<Vec<ClientTableList>, Box<dyn std::error::Error>> {
        let client: Client = Client::new();

        let req: reqwest::Response = client
            .get("http://localhost:8090/api/client_list")
            .send()
            .await?;

        let body: String = req.text().await?;
        let json_data: Data = serde_json::from_str(&body)?;

        let data: Vec<ClientTableList> = json_data.data;

        Ok(data)
    }
}
