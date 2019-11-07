#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tera;

use std::collections::HashMap;

use actix_web::{
    App,
    error,
    Error,
    get,
    http,
    HttpResponse,
    HttpServer,
    middleware,
    post,
    Responder,
    web
};

#[get("/")]
fn index() -> impl Responder {
    HttpResponse::SeeOther()
        .header(http::header::LOCATION, "/get")
        .finish()
        .into_body()
}

#[get("/get")]
fn get(
    tmpl: web::Data<tera::Tera>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    // Get param in parsed query-string
    let empty = &String::from("");

    let name = query.get("name").unwrap_or(empty);
    let message = query.get("message").unwrap_or(empty);
    let vec = &vec![10, 20, 30];

    let mut ctx = tera::Context::new();
    ctx.insert("name", name);
    ctx.insert("message", &format!("{} GET!", message));
    ctx.insert("vec", vec);

    let s = tmpl.render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(s)
    )
}

#[post("/post")]
fn post(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<Example>,
) -> Result<HttpResponse, Error> {
    let name = &params.name;
    let message = &params.message;
    let vec = &vec![90, 80, 70];

    let mut ctx = tera::Context::new();
    ctx.insert("name", name);
    ctx.insert("message", &format!("{} POST!!", message));
    ctx.insert("vec", vec);

    let s = tmpl.render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(s)
    )
}

#[derive(Serialize, Deserialize)]
pub struct Example {
    name: String,
    message: String,
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        let tera = compile_templates!(
            concat!(env!("CARGO_MANIFEST_DIR"),
             "/templates/**/*")
         );

        App::new()
            .data(tera)
            .wrap(middleware::Logger::default())
            .service(index)
            .service(get)
            .service(post)
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}