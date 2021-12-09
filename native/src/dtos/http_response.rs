use neon::prelude::*;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub data: String,
}

impl HttpResponse {
    pub fn to_js<'a>(&self, cx: &mut TaskContext<'a>) -> JsResult<'a, JsObject> {
        let data = cx.string(&self.data.to_string());
        let object = cx.empty_object();
        object.set(cx, "data", data).unwrap();

        Ok(object)
    }
}