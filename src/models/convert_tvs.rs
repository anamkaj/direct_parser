use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Default, Deserialize, Serialize, FromRow)]
pub struct CampaignData {
    pub update_date: String,
    pub clicks: i64,
    pub cost: f64,
    pub avg_impression_position: Option<f64>,
    pub avg_traffic_volume: f64,
    pub avg_cpc: f64,
    pub avg_pageviews: f64,
    pub bounce_rate: f64,
    pub client_login: String,
}

impl CampaignData {
    pub async fn transform_tvs(
        content: &String,
    ) -> Result<Vec<CampaignData>, Box<dyn std::error::Error>> {
        let lines: std::str::Lines = content.lines();
        let skipped_lines: Vec<&str> = lines.into_iter().skip(2).collect();
        let mut records: Vec<CampaignData> = Vec::new();

        for line in &skipped_lines {
            let update_date: String = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let columns: Vec<&str> = line.split('\t').collect();

            // if columns.len() > 11 {
            //     println!("Skipping invalid line: {}", line);
            //     continue;
            // }

            // if columns.len() < 11 {
            //     println!("Skipping invalid line: {}", line);
            //     continue;
            // }

            // if columns.len() == 0 {
            //     println!("Skipping invalid array: {}", line);
            //     continue;
            // }

            let record = CampaignData {
                update_date: update_date.clone(),
                clicks: columns[0].parse().unwrap_or_default(),
                cost: columns[1].parse().unwrap_or_default(),
                avg_impression_position: check_avg_imp(columns[2]).await,
                avg_traffic_volume: columns[3].parse().unwrap_or_default(),
                avg_cpc: columns[4].parse().unwrap_or_default(),
                avg_pageviews: columns[5].parse().unwrap_or_default(),
                bounce_rate: columns[6].parse().unwrap_or_default(),
                client_login: columns[7].trim().to_string(),
            };

            records.push(record);
        }

        Ok(records)
    }
}

async fn check_avg_imp(avg: &str) -> Option<f64> {
    match avg {
        "--" => Some(0.0),
        _ => Some(avg.parse().unwrap()),
    }
}
