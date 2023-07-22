use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::dev::ServerHandle;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use prost::Message;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// ECHO BLOCK
#[derive(Clone, PartialEq, Eq, Message)]
pub struct Echo {
    #[prost(string, tag = "1")]
    pub msg: String,
}

#[post("/echo")]
async fn echo(req_body: ProtoBuf<Echo>) -> impl Responder {
    println!("model: {req_body:?}");
    HttpResponse::Ok().protobuf(req_body.0)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
pub async fn main() -> std::io::Result<ServerHandle> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    // .client_disconnect_timeout(Duration::from_nanos(1000))
    // .client_request_timeout(Duration::from_nanos(1000))
    .bind(("127.0.0.1", 8080))?
    .run();
    let handle = server.handle();
    match server.await {
        Ok(_) => Ok(handle),
        Err(e) => Err(e),
    }
}
