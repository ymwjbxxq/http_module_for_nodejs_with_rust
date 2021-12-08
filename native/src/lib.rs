use crate::dtos::http_response::HttpResponse;
use dtos::http_request::HttpRequest;
use error::ApplicationError;
use neon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio;
pub mod dtos;
pub mod error;

#[neon::main]
pub fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get", get)?;
    //cx.export_function("post", post)?;
    Ok(())
}

fn get(mut cx: FunctionContext) -> JsResult<JsObject> {
    let request = HttpRequest::from_js(&mut cx).unwrap();
    let shared_request = Arc::from(request.clone());
    let future = async move {
        let res = fetch_url(&shared_request.url).await;
        res
    };

    let http_response = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future);

    let http_response = match http_response {
        Ok(res) => res,
        Err(e) => {
          return cx.throw_error(e.to_string());
        }
    };

    let object = http_response.to_js(&mut cx)?;
    return Ok(object);
}

async fn fetch_url(url: &str) -> Result<HttpResponse, ApplicationError> {
    let resp = reqwest::get(url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    let response = HttpResponse {
        data: serde_json::to_string(&resp)?,
    };

    Ok(response)
}
