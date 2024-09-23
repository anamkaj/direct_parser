use crate::models::convert_tvs::CampaignData;
use sqlx::{Pool, Postgres};

pub async fn insert_stat_client(
    pool: Pool<Postgres>,
    data: Vec<CampaignData>,
) -> Result<(), Box<dyn std::error::Error>> {
    let insert: &str = "INSERT INTO campaign_data 
    (update_date, clicks, cost, avg_impression_position, avg_traffic_volume, avg_cpc, avg_pageviews, bounce_rate, client_login) 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    ON CONFLICT (client_login) 
    DO UPDATE SET
        update_date = EXCLUDED.update_date,
        clicks = EXCLUDED.clicks,
        cost = EXCLUDED.cost,
        avg_impression_position = EXCLUDED.avg_impression_position,
        avg_traffic_volume = EXCLUDED.avg_traffic_volume,
        avg_cpc = EXCLUDED.avg_cpc,
        avg_pageviews = EXCLUDED.avg_pageviews,
        bounce_rate = EXCLUDED.bounce_rate";

    for st in data {
        sqlx::query(&insert)
            .bind(st.update_date)
            .bind(st.clicks)
            .bind(st.cost)
            .bind(st.avg_impression_position)
            .bind(st.avg_traffic_volume)
            .bind(st.avg_cpc)
            .bind(st.avg_pageviews)
            .bind(st.bounce_rate)
            .bind(st.client_login)
            .execute(&pool)
            .await?;
    }

    Ok(())
}
