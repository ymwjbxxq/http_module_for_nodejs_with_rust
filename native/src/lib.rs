use crate::dtos::http_response::HttpResponse;
use dtos::http_request::HttpRequest;
use error::ApplicationError;
use neon::prelude::*;
use std::collections::HashMap;
use tokio::runtime::Runtime;
use once_cell::sync::OnceCell;
pub mod dtos;
pub mod error;

#[neon::main]
pub fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get", get)?;
    //cx.export_function("post", post)?;
    Ok(())
}

fn runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();

    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())))
}

fn get(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let request = HttpRequest::from_js(&mut cx).unwrap();
    let rt = runtime(&mut cx)?;
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();
    rt.spawn(async move {
        let response = fetch_url(&request.url).await;
        deferred.settle_with(&channel, move |mut cx| {
            // Convert a `reqwest::Error` to a JavaScript exception
            let http_response = response.or_else(|err| cx.throw_error(err.to_string()))?;
            let object = http_response.to_js(&mut cx)?;
            Ok(object)
        });
    });
    
    // Return the promise back to JavaScript
    Ok(promise)
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
