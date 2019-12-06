#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use futures::StreamExt;

use kube::{
    api::{Informer, Object, RawApi, Void, WatchEvent},
    client::APIClient,
    config,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Noop {}

type KubeNoop = Object<Noop, Void>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // All logging disable by default, enable some while testing
    std::env::set_var("RUST_LOG", "info,kube=trace");
    env_logger::init();
    info!("Hello operator incluster3");

    // let config = match env::var("HOME").expect("have HOME dir").as_ref() {
    //     "/root" => kube::config::incluster_config(),
    //     _ => kube::config::load_kube_config(),
    // }.expect("Failed to load kube config");

    //let config = config::load_kube_config().await?;
    let config = config::incluster_config().expect("Failed to get incluster config");
    let client = APIClient::new(config);
    let namespace = std::env::var("NAMESPACE").unwrap_or("default".into());

    let resource = RawApi::customResource("rustnoops")
        .version("v1alpha1")
        .group("test.com")
        .within(&namespace);

    let informer = Informer::raw(client, resource).init().await?;
    loop {
        let mut events = informer.poll().await?.boxed();
        while let Some(event) = events.next().await {
            handle(event?).await?;
        }
    }

    Ok(())
}

async fn handle(event: WatchEvent<KubeNoop>) -> anyhow::Result<()> {
    match event {
        WatchEvent::Added(o) => {
            println!("Added: {}", o.metadata.name);
        }
        WatchEvent::Modified(o) => {
            println!("Modified: {}", o.metadata.name);
        }
        WatchEvent::Deleted(o) => {
            println!("Deleted: {}", o.metadata.name);
        }
        WatchEvent::Error(e) => {
            println!("Error event: {:?}", e);
        }
    }
    Ok(())
}
