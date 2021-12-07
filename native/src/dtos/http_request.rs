use neon::prelude::*;

use crate::error::ApplicationError;

#[derive(Debug)]
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

    pub fn to_js<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        let url = cx.string(&self.url);
        let method = cx.string(&self.method.to_string());

        let object = cx.empty_object();
        object.set(cx, "url", url).unwrap();
        object.set(cx, "method", method).unwrap();

        Ok(object)
    }
}

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
}
impl HttpMethod {
    pub fn from_str(s: &str) -> Self {
        match s {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            _ => HttpMethod::GET,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            HttpMethod::GET => "GET".to_string(),
            HttpMethod::POST => "POST".to_string(),
        }
    }
}
