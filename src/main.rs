use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::time::Instant;

async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", r);
    let res = ws::start(MyWs::new(), &r, stream);
    println!("{:?}", res);
    res
}

/// Define HTTP actor
struct MyWs {
    hb: Instant,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl MyWs {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs::new(), &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
