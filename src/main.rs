
use {
    hyper::{
        Body, Error, Client, Request, Response, Server, Uri,
        service::{make_service_fn, service_fn}
    },
    std::net::SocketAddr,
    std::str::FromStr
};

fn forward_uri<B>(forward_url: &'static str, req: &Request<B>) -> Uri {
    
    let forward_uri = match req.uri().query() {
        Some(query) => format!("{}{}?{}", forward_url, req.uri().path(), query),
        None => format!("{}{}", forward_url, req.uri().path()),
    };

    Uri::from_str(forward_uri.as_str()).unwrap()
}

async fn call(forward_url: &'static str, mut _req: Request<Body>) -> 
    Result<Response<Body>, hyper::Error> {
        *_req.uri_mut() = forward_uri(forward_url, &_req);
        let url_str = forward_uri(forward_url, &_req);
        let res = Client::new().get(url_str).await;
        res
}


async fn run_server(forward_url: &'static str, addr: SocketAddr) {
    let server = Server::bind(&addr).serve(make_service_fn(move |_| {
        async move { Ok::<_, Error>(service_fn(move |req| call(forward_url, req))) }
    }));
    if let Err(err) = server.await {
        eprintln!("server error: {}", err);
    }
}
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let url = "http://127.0.0.1:9061";
    run_server(url, addr).await
}