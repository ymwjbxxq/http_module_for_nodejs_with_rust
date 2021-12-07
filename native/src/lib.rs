use dtos::http_request::{HttpRequest};
use neon::prelude::*;

pub mod dtos;
pub mod error;

#[neon::main]
pub fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("run", run)?;
    //cx.export_function("get", get)?;
    Ok(())
}

fn run(mut cx: FunctionContext) -> JsResult<JsObject> {
    let request = HttpRequest::from_js(&mut cx).unwrap();
    let object = request.to_js(&mut cx)?;
    return Ok(object);
}


