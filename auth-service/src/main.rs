// use axum::{response::Html, routing::get, Router};
// use tower_http::services::ServeDir;
// use auth_service::Application;

// #[tokio::main]
// // async fn main() {
// //     let app = Router::new()
// //         .nest_service("/", ServeDir::new("assets"))
// //         .route("/hello", get(hello_handler));

// //     // Here we are using ip 0.0.0.0 so the service is listening on all the configured network interfaces.
// //     // This is needed for Docker to work, which we will add later on.
// //     // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
// //     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
// //     println!("listening on {}", listener.local_addr().unwrap());

// //     axum::serve(listener, app).await.unwrap();
// // }

// // async fn hello_handler() -> Html<&'static str> {
// //     // TODO: Update this to a custom message!
// //     Html("<h1>Hello, World!</h1>")
// // }

// async fn main() {
//     let app = Application::build("0.0.0.0:3000")
//         .await
//         .expect("Failed to build app");

//     app.run().await.expect("Failed to run app");
// }

use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    app_state::AppState,
    services::{
        hashmap_two_fa_code_store::HashmapTwoFACodeStore,
        hashmap_user_store::HashmapUserStore,
        hashset_banned_token_store::HashsetBannedTokenStore,
        mock_email_client::MockEmailClient 
    },
    utils::constants::prod,
    Application
};

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));

    let email_client = Arc::new(MockEmailClient);

    let app_state = AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
        email_client,
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}