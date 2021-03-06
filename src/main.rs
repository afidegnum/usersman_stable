pub mod content;
pub mod errors;
pub mod mail;

use actix_cors::Cors;

use askama::Template;
use std::collections::HashMap;

pub mod auth;
pub mod config;
pub mod media;
pub mod pages;
pub mod templ;

// For google ID

// use crate::auth::{
//     auth_url, callback, change_password, change_password_user, index_html, login_html, main_css,
//     main_js, register_email, register_html, register_user,
// };

use crate::auth::{
    change_password, change_password_user, get_me, get_secrets, index_one, login, login_html,
    logout, main_css, main_js, register_email, register_html, register_user, update_secret,
};

use crate::config::KEY_LENGTH;
use crate::errors::ServiceError;
use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::{Compress, Logger};
use actix_web::{http, http::header, middleware, web, App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use env_logger;
use rand::{thread_rng, Rng};
use std::io;
use tokio_postgres::NoTls;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[derive(Template)]
#[template(path = "user.html")]
struct UserTemplate<'a> {
    name: &'a str,
    text: &'a str,
}

pub fn get_random_string(n: usize) -> String {
    let mut rng = thread_rng();
    (0..)
        .filter_map(|_| {
            let c: char = (rng.gen::<u8>() & 0x7f).into();
            match c {
                ' '..='~' => Some(c),
                _ => None,
            }
        })
        .take(n)
        .collect()
}

async fn update_secrets() -> Result<(), ServiceError> {
    let config = config::Config::from_env().unwrap();
    // update_secret(&CONFIG.secret_path).await?;
    update_secret(&*config.ggle_clnt.secret_path).await?;
    update_secret(&*config.ggle_clnt.secret_path).await
}

// pub async fn start_app() -> Result<(), ServiceError> {
//     update_secrets().await?;
//     get_secrets(&CONFIG.secret_path, &CONFIG.jwt_secret_path).await?;
//     run_app(CONFIG.port, SECRET_KEY.get(), CONFIG.domain.clone()).await
// }

async fn index(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = if let Some(name) = query.get("name") {
        UserTemplate {
            name,
            text: "Welcome!",
        }
        .render()
        .unwrap()
    } else {
        Index.render().unwrap()
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
//
#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    let config = config::Config::from_env().unwrap();
    let domain = config.srv_cnf.host.clone();
    let pool = config.pg.create_pool(NoTls).unwrap();
    let bind_addr = format!("{}:{}", config.srv_cnf.host, config.srv_cnf.port);

    println!(
        "Starting server at http://{}:{}",
        config.srv_cnf.host, config.srv_cnf.port
    );

    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(
                Cors::default()
                    // .supports_credentials()
                    // .send_wildcard()

                    // .allowed_headers(vec![
                    //     http::header::ORIGIN,
                    //     header::AUTHORIZATION,
                    //     header::ACCEPT,
                    //     header::CONTENT_TYPE,
                    //     header::REFERER,
                    //     header::ALLOW
                    // ])
                    .expose_any_header()
                    .allow_any_method()
                    .allow_any_header()
                    .allow_any_origin()
                    // .allowed_origin("http://127.0.0.1:8081")
                    // .allowed_origin_fn(|origin, _req_head| {
                    //     origin.as_bytes().ends_with(b":8081")
                    // })
                    .max_age(3600),
        )
            .wrap(Compress::default())
            // .wrap(middleware::NormalizePath::default())
            .wrap(Logger::default())
            .wrap(Logger::new(
                "%a %r %s %b %% %U %{FOO}i %{FOO}o  %{User-Agent}i",
            ))
            .wrap(IdentityService::new(CookieIdentityPolicy::new(&private_key)
				       .name("auth-main")
				       .path("/")
				       .domain(&domain)
				       .max_age(36000) //   TODO Parametize into config.rs
				       .secure(false),
            ))
            .service(
        web::scope("/api")
            .service(
        web::resource("/auth")
            .route(web::post().to(login))
            .route(web::delete().to(logout))
            .route(web::get().to(get_me)),
            )
            .service(web::resource("/invitation").route(web::post().to(register_email)))
            .service(
        web::resource("/register/{invitation_id}")
            .route(web::post().to(register_user)),
            )
            .service(
        web::resource("/password_change")
            .route(web::post().to(change_password_user)),
            )
        // .service(web::resource("/auth_url").route(web::post().to(auth_url)))
        // .service(web::resource("/callback").route(web::get().to(callback))), // .service(web::resource("/status").route(web::get().to(status))),
            )
            .service(
        web::scope("/auth")
            .service(web::resource("/index.html").route(web::get().to(index_one)))
            .service(web::resource("/main.css").route(web::get().to(main_css)))
            .service(web::resource("/main.js").route(web::get().to(main_js)))
            .service(web::resource("/register.html").route(web::get().to(register_html)))
            .service(web::resource("/login.html").route(web::get().to(login_html)))
            .service(
        web::resource("/change_password.html")
            .route(web::get().to(change_password)),
            ),
        )

            .service(web::resource("/").name("home").route(web::get().to(pages::index)))
            .service(web::resource("/favicon.ico").route(web::get().to(|| HttpResponse::Ok())))
            .service(web::resource("/{content}").name("content").route(web::get().to(pages::index)))
        // .service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/admin/*").route(web::get().to(index)))
        .service(web::scope("/content").configure(content::init_routes))
        .service(fs::Files::new("/", "./templates").show_files_listing())
    })
    .bind(bind_addr)?
    .run()
    .await
}
