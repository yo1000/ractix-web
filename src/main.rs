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
    let s: String = match query.get("name") {
        Some(name) => {
            // submitted form
            let mut ctx = tera::Context::new();
            ctx.insert("name", &name.to_owned());
            ctx.insert("text", &"Welcome!".to_owned());

            tmpl.render("index.html", &ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error"))?
        }
        None => {
            let mut ctx = tera::Context::new();
            ctx.insert("name", "");
            ctx.insert("text", "");

            tmpl.render("index.html", &ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error"))?
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(s)
    )
}

#[post("/post")]
fn post() -> impl Responder {
    HttpResponse::Ok()
        .body("POST!!")
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