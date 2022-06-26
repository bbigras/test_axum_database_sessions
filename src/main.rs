use axum::{response::Redirect, routing::get, Router};
use axum_database_sessions::{AxumSession, AxumSessionConfig, AxumSessionLayer, AxumSessionStore};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let poll = connect_to_database().await.unwrap();

    //This Defaults as normal Cookies.
    //To enable Private cookies for integrity, and authenticity please check the next Example.
    let session_config = AxumSessionConfig::default().with_table_name("test_table");

    let session_store = AxumSessionStore::new(Some(poll.clone().into()), session_config);
    session_store.migrate().await.unwrap();

    // build our application with some routes
    let app = Router::new()
        .route("/", get(greet))
        .route("/redirect", get(redirect))
        .route("/logout", get(handler_logout))
        .layer(AxumSessionLayer::new(session_store));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn redirect(session: AxumSession) -> Redirect {
    let user = "redirect".to_string();
    session.set("user", user).await;

    Redirect::to("/")
}

async fn handler_logout(session: AxumSession) -> Redirect {
    println!("logout");
    // session.clear_all().await;
    // session.remove("user-id").await;
    session.destroy().await;

    Redirect::to("/")
}

async fn greet(session: AxumSession) -> String {
    let mut user: String = dbg!(session.get("user").await).unwrap_or_else(|| String::new());

    user += "a";
    session.set("user", &user).await;

    user
}

async fn connect_to_database() -> anyhow::Result<sqlx::Pool<sqlx::Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/postgres")
        .await?;

    Ok(pool)
}
