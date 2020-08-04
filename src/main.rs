#![feature(proc_macro_hygiene, decl_macro)]

use rocket::get;
use rocket_contrib::json::Json;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, routes_with_openapi, JsonSchema};

#[derive(serde::Serialize, JsonSchema)]
struct Response {
    reply: String,
}

#[openapi]
#[get("/")]
fn my_controller() -> Json<Response> {
    Json(Response {
        reply: "show me the docs!".to_string(),
    })
}

fn get_docs() -> SwaggerUIConfig {
    use rocket_okapi::swagger_ui::UrlObject;

    SwaggerUIConfig {
        url: "/my_resource/openapi.json".to_string(),
        ..Default::default()
    }
}

fn main() {
    rocket::ignite()
        .mount("/my_resource", routes_with_openapi![my_controller])
        .mount("/swagger", make_swagger_ui(&get_docs()))
        .launch();
}
