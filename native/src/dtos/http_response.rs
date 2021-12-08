use neon::prelude::*;

pub struct HttpResponse {
    pub data: String,
}

impl HttpResponse {
    pub fn to_js<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        let data = cx.string(&self.data.to_string());
        let object = cx.empty_object();
        object.set(cx, "data", data).unwrap();

        Ok(object)
    }
}