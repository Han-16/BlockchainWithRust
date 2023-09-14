use super::{App, Block};
use lobp2p:: {
    floodsub::{Floodsub, FloodsubEvent, Topic},
    identity,
    mdns::{Mdns, MdnsEvent},
    swarm::{NetworkBehaviourEventProcesss, Swarm},
    NetworkBehaviour, PeerId,
};
use log::{error, info};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tokio::sync::mpsc;

pub static KEYS: Lazy = Lazy::new(identity::Keypair::generate_ed25519); // Lazy: 값을 필요로 할 때까지 계산/초기화하지 않는 것
pub static PEER_ID: Lazy = Lazy::new(|| PeerId::from(KEYS.public()));
pub static CHAIN_TOPIC: Lazy = Lazy::new(|| Topic::new(":chains"));
pub static BLOCK_TOPIC: Lazy  = Lazy::new(|| Topic::new("blocks"));


#[derive(Debug, Serialize, Deserialize)]
pub struct ChainResponse {
    pub blocks: Vec,
    pub receiver: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalChainRequest {
    pub from_peer_id: String,
}

pub enum EventType {
    LocalChainRequest(ChainResponse),
    Input(String),
    Init,
}

#[derive(NetworkBehaviour)]
pub struct AppBehaviour {
    pub floodsub: Floodsub,
    pub mdns: Mdns,
    #[behaviour(ignore)]
    pub response_sender: mpsc::UnboundedSender,
    #[behaviour(ignore)]
    pub init_sender: mpsc::UnboundedSender,
    #[behaviour(ignore)]
    pub app: App,
}

impl AppBehaviour {
    pub async fn new (
        app: App,
        response_sender: mpsc::UnboundedSender,
        init_sender: mpsc::UnboundedSender, ) -> Self { let mut behaviour =  Self {
            app,
            floodsub: Floodsub::new(*PEER_ID),
            mdns: Mdns::new(Default::default())
                                    .await
                                    .expect("can create mdns"),
            response_sender,
            init_sender,
        };
        behaviour.floodsub.subscribe(CHAIN_TOPIC.clone());
        behaviour.floodsub.subscribe(BLOCK_TOPIC.clone());

        behaviour
    }
}