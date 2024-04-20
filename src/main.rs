use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};

mod api;
mod db;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let pool = db::establish_connection().await;

    let app = Router::new()
        .route("/api/", get(api::topic::get_topics))
        .route("/api/register", post(api::user::create_user))
        .route("/api/user/:username", get(api::user::get_user))
        .route("/api/user/list", get(api::user::get_users))
        .route("/api/verify/:password", get(api::user::verify_pwd))
        .route("/api/topic/initiate", post(api::topic::create_topic))
        .route("/api/topic/:topic_id", get(api::topic::get_topic))
        .with_state(pool)
        .layer(trace_layer);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// fn internal_error<E>(err: E) -> (StatusCode, String)
// where
//     E: std::error::Error,
// {
//     (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
// }
