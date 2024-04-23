use axum::{
    routing::{get, post},
    Router,
};
use time::{macros::format_description, UtcOffset};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
use tracing_subscriber::fmt::time::OffsetTime;

mod api;
mod db;

use self::api::{topic, user};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let timer = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    // initialize tracing
    tracing_subscriber::fmt()
        .with_timer(timer)
        .with_target(false)
        .compact()
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let pool = db::establish_connection().await;

    let app = Router::new()
        .route("/api/", get(topic::get_topics))
        .route("/api/register", post(user::register))
        .route("/api/login", post(user::login))
        .route("/api/user/:username", get(user::get_user))
        .route("/api/user/list", get(user::get_users))
        .route("/api/topic/initiate", post(topic::create_topic))
        .route("/api/topic/:topic_id", get(topic::get_topic))
        .with_state(pool)
        .layer(trace_layer);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("Server listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// fn internal_error<E>(err: E) -> (StatusCode, String)
// where
//     E: std::error::Error,
// {
//     (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
// }
