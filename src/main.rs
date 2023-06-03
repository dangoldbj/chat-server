use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("localhost:8080").await.unwrap();

    let (socket, _addr) = listner.accept().await.unwrap();
}
