use hyper;
use hyper::client::conn::http1;
use hyper::client::conn::http1::SendRequest;
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio;
pub struct Client {
    host: String,
    port: u16,
    tcp_stream: tokio::net::TcpStream,
    sender: SendRequest<String>,
}

pub async fn test() {
    let uri = hyper::Uri::from_static("google.com");
    let socket_address = format!("{}:{}", uri.host().unwrap(), uri.port_u16().unwrap_or(80));
    let tcp_stream = tokio::net::TcpStream::connect(socket_address).await.unwrap();
    let io = TokioIo::new(tcp_stream);
    let (mut sender, connection) = http1::handshake::<_, String>(io).await.unwrap();
    tokio::task::spawn(async move {
        if let Err(err) = connection.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = uri.authority().unwrap().clone();

    let path = uri.path();
    let req = Request::builder().body(String::new());
    println!("{:?}", sender.send_request(req.expect("Req error")).await);
}
