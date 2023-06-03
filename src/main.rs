mod actor;
mod server_impl;

use actor::{Echo, EchoActor};
use coerce::actor::system::ActorSystem;
use coerce::actor::IntoActor;
use coerce::remote::system::RemoteActorSystem;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::actor::StartServer;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate async_trait;

#[tokio::main]
async fn main() {
    let (tracer, _uninstall) = opentelemetry_jaeger::new_pipeline()
        .with_service_name("example-main")
        .install()
        .unwrap();
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    tracing_subscriber::registry()
        .with(opentelemetry)
        .try_init()
        .unwrap();

    let system = ActorSystem::new();
    let remote = RemoteActorSystem::builder()
        .with_tag("example-main")
        .with_actor_system(system)
        .with_handlers(|handlers| handlers.with_handler::<EchoActor, Echo>("EchoActor.Echo"))
        .build()
        .await;

    remote
        .clone()
        .cluster_worker()
        .listen_addr("localhost:30100")
        .start()
        .await;

    let echo = EchoActor
        .into_actor(Some("echo-actor".to_string()), remote.actor_system())
        .await
        .expect("unable to start echo actor");

    echo.send(StartServer()).await.expect("sdf");
}
