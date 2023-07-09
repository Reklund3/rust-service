use crate::server_impl;
use actix_web::dev::ServerHandle;
use coerce::actor::context::ActorContext;
use coerce::actor::message::Handler;
use coerce::actor::Actor;
use coerce_macros::JsonMessage;

pub struct PostServer {
    server: Option<ServerHandle>,
}

impl PostServer {
    pub fn new() -> Self {
        Self { server: None }
    }
}

impl Actor for PostServer {}

#[derive(JsonMessage, Serialize, Deserialize)]
#[result("String")]
pub struct StartServer();

#[async_trait]
impl Handler<StartServer> for PostServer {
    async fn handle(&mut self, _message: StartServer, _ctx: &mut ActorContext) -> String {
        tokio::task::spawn_blocking(move || {
            let _s = server_impl::main();
            // self.actor_ref(_ctx).send(AddServerHandler { result: s });
            // self.server = Some(s.unwrap());
        });
        println!("Server started!!!");
        "Server started".to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub enum ShutdownResult {
    Success(),
    Failure(),
}

#[derive(JsonMessage, Serialize, Deserialize)]
#[result(ShutdownResult)]
pub struct Shutdown();

#[async_trait]
impl Handler<Shutdown> for PostServer {
    async fn handle(&mut self, _message: Shutdown, _ctx: &mut ActorContext) -> ShutdownResult {
        println!("Server received shutdown request.");
        match self.server.clone() {
            Some(handle) => {
                println!("Releasing handle.");
                handle.stop(true).await;
                ShutdownResult::Success()
            }
            None => {
                println!("Handle already released");
                ShutdownResult::Success()
            }
        }
    }
}
