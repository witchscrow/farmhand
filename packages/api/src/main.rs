mod config;
mod jwt;
mod middleware;
mod routes;

use axum::{
    extract::DefaultBodyLimit,
    middleware as axum_mw,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use config::Config;
use routes::upload::UPLOAD_CHUNK_SIZE;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Shared state available to the API
pub struct AppState {
    db: PgPool,
    config: Config,
}

#[tokio::main]
async fn main() {
    // Start the tracer
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "api=debug,db=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Initialize our application configuration
    let config = Config::new();
    // Create our listener on configured address
    let listener = tokio::net::TcpListener::bind(&config.get_address())
        .await
        .unwrap();
    // Initialize a connection to the database
    let db = db::connect_to_database()
        .await
        .expect("Could not connect to database");
    // Run migrations
    let _mig = db::run_migrations(&db)
        .await
        .expect("Could not run database migrations");
    // Store shared data as state between routes
    let state = Arc::new(AppState { db, config });
    routes::upload::init_cleanup().await;
    // Initialize our router with the shared state and required routes
    let app = Router::new()
        .route("/", get(index))
        .nest(
            "/auth",
            Router::new()
                .route("/register", post(routes::auth::register))
                .route("/login", post(routes::auth::login)),
        )
        .nest(
            "/user",
            Router::new()
                .route("/me", get(routes::user::get_user))
                .layer(axum_mw::from_fn_with_state(
                    state.clone(),
                    middleware::auth::auth_middleware,
                )),
        )
        .route(
            "/upload",
            post(routes::upload::upload_video)
                .layer(DefaultBodyLimit::max(UPLOAD_CHUNK_SIZE * 8)) // Increased for concurrent uploads
                .layer(axum_mw::from_fn_with_state(
                    state.clone(),
                    middleware::auth::auth_middleware,
                )),
        )
        // TODO: Attach this to the upload route when you re-add it
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new())
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Seconds),
                ),
        );
    // Start the server
    tracing::info!(
        "API Server started on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

/// Render the root index page
async fn index() -> impl IntoResponse {
    "Welcome to the farmhand api"
}
