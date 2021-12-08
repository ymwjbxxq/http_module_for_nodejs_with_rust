use crate::dtos::http_method::HttpMethod;
use neon::prelude::*;
use crate::error::ApplicationError;

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub url: String,
    pub method: HttpMethod,
}

impl HttpRequest {
    pub fn from_js<'a>(cx: &mut FunctionContext<'a>) -> Result<Self, ApplicationError> {
        let js_object_handle: Handle<JsObject> = cx.argument(0).unwrap();
        let js_object = js_object_handle
            .downcast::<JsObject, _>(cx)
            .unwrap_or(JsObject::new(cx));

        let rust_url = js_object
            .get(cx, "url")?
            .downcast::<JsString, _>(cx).expect("url is missing")
            .value(cx);

        let rust_method = js_object
            .get(cx, "method")?
            .downcast::<JsString, _>(cx).expect("method is missing")
            .value(cx);

        Ok(Self {
            url: rust_url,
            method: HttpMethod::from_str(&rust_method),
        })
    }
}