mod actor;
mod server_impl;

use crate::actor::StartServer;
use actor::{PostServer, Shutdown};
use coerce::actor::system::ActorSystem;
use coerce::actor::IntoActor;
use coerce::remote::system::RemoteActorSystem;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

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
        .with_handlers(|handlers| {
            handlers.with_handler::<PostServer, Shutdown>("PostServer.Shutdown")
        })
        .build()
        .await;

    remote
        .clone()
        .cluster_worker()
        .listen_addr("localhost:30100")
        .start()
        .await;

    let server = PostServer::new()
        .into_actor(Some("post-service".to_string()), remote.actor_system())
        .await
        .expect("unable to start post service actor");

    server
        .send(StartServer())
        .await
        .expect("Failed to start the server.");

    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for event");

    server
        .send(Shutdown())
        .await
        .expect("Failed to gracefully shutdown the server.");
}
