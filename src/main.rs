use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
use tracing_subscriber::fmt::time::ChronoLocal;

mod api;
mod db;

use self::api::{topic, user};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()))
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
        .route("/api/settings", get(user::get_my_settings))
        .route("/api/settings/update", post(user::update_my_settings))
        .route("/api/my-topics", get(user::get_my_topics))
        .route("/api/my-favorites", get(user::get_my_favorites))
        .route("/api/favor", post(user::favor))
        .route("/api/topic/initiate", post(topic::create_topic))
        .route("/api/topic/:topic_id", get(topic::get_topic))
        .route("/api/topic/update/:topic_id", get(topic::get_update_topic))
        .route("/api/topic/update", post(topic::topic_update))
        .route("/api/topic/comment", post(topic::topic_comment))
        .route("/api/profile/:username", get(topic::get_user_profile))
        .route(
            "/api/profile/:username/favorites",
            get(topic::get_user_favorites),
        )
        .with_state(pool)
        .layer(trace_layer);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("Server listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
