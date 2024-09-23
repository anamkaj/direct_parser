use crate::{
    db::{get_client_table::ClientTableList, insert_stat_client::insert_stat_client},
    models::convert_tvs::CampaignData,
};

use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tokio::time::sleep;

pub async fn req_stat_client(pool: Pool<Postgres>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Запуск парсинга статистики");
    dotenv().ok();
    let token: String = std::env::var("ACCESS_TOKEN").unwrap();
    let client: reqwest::Client = reqwest::Client::builder().build()?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept-Language", "ru".parse()?);
    headers.insert("processingMode", "auto".parse()?);
    headers.insert("returnMoneyInMicros", "false".parse()?);
    headers.insert("skipReportSummary", "true".parse()?);
    headers.insert("IncludeVAT", "true".parse()?);
    headers.insert("Content-Type", "application/json".parse()?);
    headers.insert("Authorization", token.parse()?);

    let client_list: Vec<ClientTableList> = ClientTableList::get_client_list().await?;

    let filter_active_client: Vec<ClientTableList> = client_list
        .into_iter()
        .filter(|x| x.status_ads == true && x.status_client == true && x.direct_login != "")
        .collect();

    let len_client = &filter_active_client.len();

    println!("👤Количество активных клиентов: {}", len_client);

    for cl in &filter_active_client {
        println!("Processing client: {:?}", cl.direct_login);
        sleep(Duration::from_secs(10)).await;

        headers.insert("Client-Login", cl.direct_login.parse()?);

        let json_req: serde_json::Value = serde_json::json!({
            "params": {
                "SelectionCriteria": {
                    "DateFrom": format!("{}", cl.date_start),
                    "DateTo": format!("{}", cl.data_end),
                },
                "FieldNames": [
                    "Clicks",
                    "Cost",
                    "AvgImpressionPosition",
                    "AvgTrafficVolume",
                    "AvgCpc",
                    "AvgPageviews",
                    "BounceRate",
                    "ClientLogin"
                ],
                "ReportName": format!("{}", cl.direct_login),
                "ReportType": "CUSTOM_REPORT",
                "DateRangeType": "CUSTOM_DATE",
                "Format": "TSV",
                "IncludeVAT": "YES",
                "IncludeDiscount": "YES"
            }
        });

        let request = client
            .request(
                reqwest::Method::GET,
                "https://api.direct.yandex.com/json/v5/reports",
            )
            .headers(headers.clone())
            .body(json_req.to_string());

        let response: reqwest::Response = request.send().await?;
        let status: reqwest::StatusCode = response.status();

        //* Если отчет не может быть построен online , повторный запрос   */
        if status == reqwest::StatusCode::CREATED {
            sleep(Duration::from_secs(60)).await;
            let request = client
                .request(
                    reqwest::Method::GET,
                    "https://api.direct.yandex.com/json/v5/reports",
                )
                .headers(headers.clone())
                .body(json_req.to_string());

            let response: reqwest::Response = request.send().await?;
            let body: &String = &response.text().await?;

            let _ = add_stat_client(pool.clone(), &body).await?;
        }

        let body: &String = &response.text().await?;
        let _ = add_stat_client(pool.clone(), &body).await?;
    }

    Ok(())
}

async fn add_stat_client(
    pool: Pool<Postgres>,
    body: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<CampaignData> = CampaignData::transform_tvs(&body).await?;
    let _ = insert_stat_client(pool, data).await?;

    Ok(())
}
