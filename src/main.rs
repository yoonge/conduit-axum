use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Json, Router,
};
use deadpool_diesel::{postgres, Runtime};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{debug, info, Level};

use conduit_axum::models::{NewUser, User};
use conduit_axum::schema::users;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let db_url = std::env::var("DATABASE_URL").expect("`DATABASE_URL` must be set.");
    let manager = postgres::Manager::new(db_url, Runtime::Tokio1);
    let pool = postgres::Pool::builder(manager)
        .build()
        .expect("Failed to create pool.");

    {
        let conn = pool.get().await.expect("Failed to get connection.");
        info!("Database connection established.");
        conn.interact(|pgc| pgc.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        // .route("/users", post(create_user));
        .with_state(pool)
        .layer(trace_layer);

    // run our app with hyper, listening globally on port 3000
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
    debug!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `NewUser` type
    State(pool): State<postgres::Pool>,
    Json(payload): Json<NewUser<'_>>,
) -> Result<Json<User>, (StatusCode, String)> {
    let conn = pool
        .get()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let new_user = conn
        .interact(|pgc| {
            diesel::insert_into(users::table)
                .values(&payload)
                .returning(User::as_returning())
                .get_result(pgc);
        })
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(new_user))
}
