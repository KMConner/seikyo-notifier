use std::collections::HashMap;
use std::error::Error;

use serde::Deserialize;

use crate::seikyo_client::error::ApiError;

const GET_POINT_URL: &str = "https://mb.seikyou.jp/mobileapp_common/tohoku/getUserPoint/";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonResponse {
    status: String,
    data: IcData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IcData {
    ic_prep_zandaka: Option<String>,
}

pub fn get_prepaid_amount(token: &String) -> Result<u32, Box<dyn Error>> {
    let app_id = String::from("6501000001");
    let mut params = HashMap::new();
    params.insert(String::from("appId"), &app_id);
    params.insert(String::from("accessToken"), token);

    let client = reqwest::blocking::Client::new();
    let request = client.post(GET_POINT_URL)
        .form(&params)
        .send()?
        .text()?;

    let result: JsonResponse = serde_json::from_str(request.as_str())?;
    if result.status != "0" {
        return Err(Box::new(ApiError::new("status is not 0")));
    }

    match result.data.ic_prep_zandaka {
        None => Err(Box::new(ApiError::new("failed to parse amount value."))),
        Some(t) => Ok(t.parse::<u32>()?)
    }
}
