use axum::{
    routing::get,
    Router
};

#[tokio::main]
async fn main() {
   let app = Router::new().route("/",get(root)).route("/hello",get(run));
   let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
   axum::serve(listener,app).await.unwrap();
}

async fn root() -> &'static str{
    "Hello Arun bhai. Naukri lag jayegi"
}

async fn run() -> &'static str{
    "Hello from the another server"
}