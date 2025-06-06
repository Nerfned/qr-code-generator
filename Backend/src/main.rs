mod api;
mod content;
mod db_initializer;
mod layout;
pub mod postgresql;
mod psql;
mod qr_json;
mod qrdata;
mod qrgenerator;
mod response;
mod version;
use crate::{postgresql::get_pool, qr_json::Base64};
use db_initializer::{create_database_if_not_exists, create_tables};

use crate::postgresql::init_db;

use self::{qr_json::QrJson, qrgenerator::qr_generator};
use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    response::Response,
    routing::{post, put},
    Router,
};
use base64::{
    alphabet::STANDARD,
    engine::{GeneralPurpose, GeneralPurposeConfig},
    Engine,
};

use config_file::FromConfigFile;
use serde::Deserialize;

use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Deserialize)]
struct Config {
    port: u16,
    dir: String,
    file: String,

    username: String,
    password: String,
    host: String,
    database: String,
}

#[tokio::main]
async fn main() {
    let config = match Config::from_config_file("config.toml") {
        Ok(content) => content,
        Err(err) => {
            println!("{}", err);
            panic!("CONFIG ERROR")
        }
    };

    let admin_conn_str = format!(
        "postgres://{}:{}@{}/postgres",
        config.username, config.password, config.host
    );

    create_database_if_not_exists(&admin_conn_str, &config.database)
        .await
        .expect("Failed to create database");

    let db_url = format!(
        "postgres://{}:{}@{}/{}",
        config.username, config.password, config.host, config.database
    );
    init_db(&db_url).await;

    create_tables(get_pool())
        .await
        .expect("Failed to create tables");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            "qr_server=info,warn,error,tower_http=debug",
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .nest_service(
            "/api",
            Router::new()
                .route("/load-qr", put(api::load_qr))
                .route("/send-data", put(api::send_data))
                .route("/check-user", put(api::username))
                .route("/check-email", put(api::check_email))
                .route("/login", put(api::login))
                .route("/save-qr", put(api::save_qr))
                .route("/change-email", put(api::change_email))
                .route("/username", put(api::username))
                .route("/change-password", put(api::change_password))
                .route("/registry", put(api::registry))
                .route("/delete-qr", put(api::delete_qr))
                .route("/change-qr", put(api::change_qr))
                .route("/change-icon", put(api::change_icons))
                .fallback(post(accept_form)),
        )
        .nest_service(
            "/",
            ServeDir::new(config.dir).not_found_service(ServeFile::new(config.file)),
        )
        .nest_service(
            "/user/",
            Router::new()
                .route("/create-dynqr", put(api::create_dyn_qr))
                .route("/change-dynqr", put(api::change_dyn_qr)),
        )
        .layer(DefaultBodyLimit::disable())
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_headers([CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST]),
        );
    let listener = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("listening on {listener}");

    axum::Server::bind(&listener)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn accept_form(mut multipart: Multipart) -> Response<String> {
    let mut data = None;
    let mut color = None;
    let mut layout = None;

    let mut logo = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let content_type = field.content_type().map(|value| value.to_string());
        let body = field.bytes().await.unwrap();

        if name == "base64" {
            let result = serde_json::from_slice::<Base64>(&body).unwrap();
            logo = Some(result.logo);
        }
        if name == "logo" {
            let content_type = content_type.unwrap();
            println!("{content_type}");
            logo = {
                let base64_engine = GeneralPurpose::new(&STANDARD, GeneralPurposeConfig::new());
                let image = base64_engine.encode(&body);

                Some(format!("data:{content_type};base64,{image}"))
            };
        } else if name == "qr" {
            let result = serde_json::from_slice::<QrJson>(&body).unwrap();

            color = Some(result.color);
            layout = Some(result.layout);
            data = qrdata::qrdata(result.data);
        }
    }

    let qr = qr_generator(data.unwrap(), &color.unwrap(), layout.unwrap(), logo);

    let mut response = Response::new(qr);
    response
        .headers_mut()
        .append(CONTENT_TYPE, HeaderValue::from_static("image/svg+xml"));
    response
}
