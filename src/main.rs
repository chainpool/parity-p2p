extern crate substrate_network;
extern crate substrate_libp2p_network;
extern crate substrate_runtime_primitives;
extern crate substrate_client as client;
extern crate exchange_primitives;


use substrate_network::specialization::Specialization;
use substrate_network::{NodeIndex, Context, message};
use substrate_network::StatusMessage as GenericFullStatus;
use exchange_primitives::{Block, Header, Hash};

pub struct Protocol {
  version: u64,
}

type FullStatus = GenericFullStatus<Block>;

impl Protocol {
  pub fn new() -> Self {
    Protocol {
      version: 0,
    }
  }
}

impl Specialization<Block> for Protocol {
  fn status(&self) -> Vec<u8> {
     unreachable!();
  }

  fn on_connect(&mut self, ctx: &mut Context<Block>, who: NodeIndex, status: FullStatus) {
     unreachable!();
  }

  fn on_disconnect(&mut self, ctx: &mut Context<Block>, who: NodeIndex) {
     unreachable!();
  }

  fn on_message(&mut self, ctx: &mut Context<Block>, who: NodeIndex, message: message::Message<Block>) {
     unreachable!();
  }

  fn on_abort(&mut self) {
     unreachable!();
  }

  fn maintain_peers(&mut self, _ctx: &mut Context<Block>) {
     unreachable!();
  }

  fn on_block_imported(&mut self, _ctx: &mut Context<Block>, hash: Hash, header: &Header) {
     unreachable!();
  }
}

pub type NetworkService = substrate_network::Service<Block, Protocol, Hash>;

pub type NetworkParam = substrate_network::Params<Block, Protocol, Hash>;

const DOT_PROTOCOL_ID: substrate_network::ProtocolId = *b"exc";

fn main() {
    let param = NetworkParam {
       config: substrate_network::ProtocolConfig::default(),
       network_config: substrate_network_libp2p::NetworkConfiguration::default(),
       
    };
    println!("Hello, world!");
}
