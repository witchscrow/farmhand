mod app_state;
mod config;
mod jwt;
mod middleware;
mod routes;
mod s3;

pub use app_state::AppState;
use axum::{
    middleware as axum_mw,
    response::IntoResponse,
    routing::{delete, get, post},
    Router,
};
use config::Config;

use std::sync::Arc;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Start the tracer
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "api=debug,db=debug,queue=debug,tower_http=debug,axum::rejection=trace".into()
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
    // Store shared data as state between routes
    let app_state = AppState::new(config)
        .await
        .expect("Could not construct app state");
    let state = Arc::new(app_state);
    // Initialize our router with the shared state and required routes
    let app = Router::new()
        .route("/", get(index))
        .nest(
            "/auth",
            Router::new()
                .route("/register", post(routes::auth::register))
                .route("/login", post(routes::auth::login))
                .nest(
                    "/twitch",
                    Router::new()
                        .route("/", get(routes::auth::oauth::twitch::oauth_redirect))
                        .route(
                            "/callback",
                            get(routes::auth::oauth::twitch::oauth_callback).layer(
                                axum_mw::from_fn_with_state(
                                    state.clone(),
                                    middleware::auth::auth_middleware,
                                ),
                            ),
                        ),
                ),
        )
        .nest(
            "/user",
            Router::new()
                .route("/me", get(routes::user::get_self))
                .layer(axum_mw::from_fn_with_state(
                    state.clone(),
                    middleware::auth::auth_middleware,
                )),
        )
        .nest(
            "/upload",
            Router::new()
                .route("/start", post(routes::upload::cloud::init_upload))
                .route("/finish", post(routes::upload::cloud::complete_upload))
                .layer(axum_mw::from_fn_with_state(
                    state.clone(),
                    middleware::auth::auth_middleware,
                )),
        )
        .nest(
            "/video",
            Router::new()
                .route("/", get(routes::video::get_videos))
                .route("/", delete(routes::video::delete_videos))
                .layer(axum_mw::from_fn_with_state(
                    state.clone(),
                    middleware::auth::auth_middleware,
                )),
        )
        .nest_service("/videos", tower_http::services::ServeDir::new("videos"))
        .route("/health", get(routes::health::health_check))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::debug_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        path = %request.uri().path(),
                        query_params = %request.uri().query().unwrap_or_default()
                    )
                })
                .on_request(|request: &axum::http::Request<_>, _span: &tracing::Span| {
                    tracing::info!(
                        method = %request.method(),
                        path = %request.uri().path(),
                        "incoming request"
                    );
                })
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
