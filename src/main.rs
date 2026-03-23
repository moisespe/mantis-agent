mod collector;
mod api;

#[tokio::main]
async fn main(){
    let app = api::routes::create_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5032")
        .await
        .unwrap();

    println!("Mantis Agent Online http://0.0.0.0:5032");
    axum::serve(listener, app).await.unwrap();
}