use tide::http::{Url, Method, Request, Response};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


async fn hello_world(_req: tide::Request<()>) -> tide::Result {
    let response = tide::Response::builder(200)
        .body("Hello World")
        .build();
    Ok(response)
}

#[wasm_bindgen]
pub async fn request() {
    let mut app = tide::new();
    app.at("/").get(hello_world);

    let req = Request::new(Method::Get, Url::parse("http://localhost.com").unwrap());
    let mut res: Response = app.respond(req).await.unwrap();
    let body = res.body_string().await.unwrap();
    log(body.as_str());
}
