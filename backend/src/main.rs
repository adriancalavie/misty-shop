use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
// use chrono::{DateTime, Duration, Utc};
// use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // TODO init Surreal db

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/user", post(create_user))
        .route(
            "/users",
            get(get_users).layer(
                CorsLayer::new()
                    // "*" allows all the path to have CORS access
                    .allow_origin("*".parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::GET]),
            ),
        );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn new_user(name: String) -> User {
    User {
        id: Uuid::new_v4().to_string(),
        username: name,
    }
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user: User = new_user(payload.username).await;

    // TODO Save user in db

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

async fn get_users() -> impl IntoResponse {
    let users = vec![
        new_user("Adrian".to_owned()).await,
        new_user("Diana".to_owned()).await,
        new_user("Dinu".to_owned()).await,
    ];

    (StatusCode::OK, Json(users))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    // TODO Check if it's safe to store uuid as string
    id: String,
    username: String,
}
