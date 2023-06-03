use coerce::actor::context::ActorContext;
use coerce::actor::message::Handler;
use coerce::actor::Actor;
use coerce_macros::JsonMessage;
use crate::server_impl;

pub struct EchoActor;

impl Actor for EchoActor {}

#[derive(JsonMessage, Serialize, Deserialize)]
#[result("String")]
pub struct Echo(pub String);

#[derive(JsonMessage, Serialize, Deserialize)]
#[result("String")]
pub struct StartServer();

#[async_trait]
impl Handler<Echo> for EchoActor {
    async fn handle(&mut self, message: Echo, _ctx: &mut ActorContext) -> String {
        println!("{:?}", message.0);
        message.0
    }
}
#[async_trait]
impl Handler<StartServer> for EchoActor {
    async fn handle(&mut self, _message: StartServer, _ctx: &mut ActorContext) -> String {
        let _main1 = tokio::task::spawn_blocking(move ||{ server_impl::main()});
        println!("Server started!!!");
        "Server started".to_string()
    }
}