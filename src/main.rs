use actix_web::client::Client;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use futures::Future;
use std::net::ToSocketAddrs;
use url::Url;

fn forward(
    req: HttpRequest,
    payload: web::Payload,
    url: web::Data<Url>,
    client: web::Data<Client>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut new_url = url.get_ref().clone();
    new_url.set_path(req.uri().path());
    new_url.set_query(req.uri().query());

    let forwarded_req = client
        .request_from(new_url.as_str(), req.head())
        .no_decompress();
    let forwarded_req = if let Some(addr) = req.head().peer_addr {
        forwarded_req.header("x-forwarded-for", format!("{}", addr.ip()))
    } else {
        forwarded_req
    };

    forwarded_req
        .send_stream(payload)
        .map_err(Error::from)
        .map(|res| {
            let mut client_resp = HttpResponse::build(res.status());
            for (header_name, header_value) in res
                .headers()
                .iter()
                .filter(|(h, _)| *h != "connection" && *h != "content-length")
            {
                client_resp.header(header_name.clone(), header_value.clone());
            }
            client_resp.streaming(res)
        })
}

fn main() -> std::io::Result<()> {
    
    let forward_url = Url::parse(&format!(
        "https://{}",
        ("127.0.0.1", 9062)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap()
    ))
    .unwrap();

    HttpServer::new(move || {
        App::new()
            .data(Client::new())
            .data(forward_url.clone())
            .wrap(middleware::Logger::default())
            .default_service(web::route().to_async(forward))
    })
    .bind(("0.0.0.0", 8080))?
    .system_exit()
    .workers(4)
    .run()
}