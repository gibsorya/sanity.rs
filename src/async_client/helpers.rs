use serde_json::Value;

pub async fn get_json(
    reqwest_res: reqwest::Response,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data: Value = serde_json::from_str(&reqwest_res.text().await?)?;

    Ok(data)
}