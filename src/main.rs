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

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/user/create", post(api::user::create_user))
        .with_state(pool)
        .layer(trace_layer);

    // run our app with hyper, listening globally on port 3000
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// async fn create_user(
//     // this argument tells axum to parse the request body
//     // as JSON into a `NewUser` type
//     State(pool): State<Pool>,
//     Json(new_user): Json<NewUser<'static>>
// ) -> Result<usize, (StatusCode, String)> {
//     let mut conn = pool.get().await.map_err(internal_error)?;

//     let res = diesel::insert_into(users::table)
//         .values(new_user)
//         // .returning(users::all_columns)
//         .execute(&mut conn)
//         .await
//         .map_err(internal_error)?;

//     Ok(res)
// }

// fn internal_error<E>(err: E) -> (StatusCode, String)
// where
//     E: std::error::Error,
// {
//     (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
// }
