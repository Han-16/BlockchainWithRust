use serde::{Deserialize, Serialize};

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
    pub response_sender: mpsc::UnboudedSender,
    #[behaviour(ignore)]
    pub init_sender: mpsc::UnboudedSender,
    #[behaviour(ignore)]
    pub app: App,
}