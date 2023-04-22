use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response};
use tokio;
use tokio::runtime::Builder;

use crate::datastore::DataStore;
use crate::parser;
use crate::datastore::DataStoreService;

pub struct ServerOptions {
    port: u16,
}

impl ServerOptions {
    pub fn new(port: u16) -> ServerOptions {
        ServerOptions { port }
    }
}

pub fn server(options: ServerOptions) {
    // build tokio runtime
    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(server_main(options));
}

async fn server_main(options: ServerOptions) {
    let data_store = Arc::new(DataStore::new());

    let make_svc = make_service_fn(move |_conn| {
        let data_store = data_store.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, data_store.clone())
            }))
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], options.port));

    println!("Listening on http://{}", addr);
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle_request(
    req: Request<Body>,
    data_store: Arc<DataStore>,
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/") => {
            let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let body_str = String::from_utf8_lossy(&body);
            println!("POST request body: '{}'", body_str);
            let command = parser::Parser::new(body_str.to_string()).parse();
            println!("Parsed: {:?}", command);
            let response_str = match command {
                Ok(command) => data_store.execute(command).unwrap_or("Nil".to_string()),
                Err(msg) => msg,
            };
            Ok(Response::new(Body::from(response_str + "\n")))
        }
        _ => {
            let response = Response::builder()
                .status(404)
                .body(Body::from("Not Found"))
                .unwrap();

            Ok(response)
        }
    }
}
