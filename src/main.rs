extern crate substrate_network;
extern crate substrate_network_libp2p;
extern crate substrate_runtime_primitives;
extern crate substrate_primitives;
extern crate substrate_client as client;
extern crate exchange_primitives;
extern crate exchange_executor;
extern crate exchange_runtime;
extern crate substrate_client_db as client_db;
extern crate substrate_state_db as state_db;

#[macro_use]
extern crate hex_literal;

use substrate_network::specialization::Specialization;
use substrate_network::{NodeIndex, Context, message};
use substrate_network::StatusMessage as GenericFullStatus;
use exchange_primitives::{Block, Header, Hash, UncheckedExtrinsic};
use exchange_runtime::{GenesisConfig,
    ConsensusConfig, CouncilConfig, DemocracyConfig, SessionConfig, StakingConfig,
    TimestampConfig};
use std::sync::Arc;
use std::path::PathBuf;
use std::collections::HashMap;

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

//pub type Backend = client::in_mem::Backend<Block, Hash, substrate_primitives::RlpCodec>;

//pub type Executor = client::LocalCallExecutor<Backend, exchange_executor::NativeExecutor<exchange_executor::Executor>>;

//pub type Client = client::Client<Backend, Executor, Block>; 

const FINALIZATION_WINDOW: u64 = 32;

const DOT_PROTOCOL_ID: substrate_network::ProtocolId = *b"exc";

fn genesis_config() -> GenesisConfig {
        let god_key = hex!["3d866ec8a9190c8343c2fc593d21d8a6d0c5c4763aaab2349de3a6111d64d124"];
        let genesis_config = GenesisConfig {
                consensus: Some(ConsensusConfig {
                    code: vec![],   // TODO
                    authorities: vec![god_key.clone().into()],
                }),
                system: None,
                session: Some(SessionConfig {
                    validators: vec![god_key.clone().into()],
                    session_length: 720,    // that's 1 hour per session.
                    broken_percent_late: 30,
                }),
                staking: Some(StakingConfig {
                    current_era: 0,
                    intentions: vec![],
                    transaction_base_fee: 100,
                    transaction_byte_fee: 1,
                    transfer_fee: 0,
                    creation_fee: 0,
                    reclaim_rebate: 0,
                    existential_deposit: 500,
                    balances: vec![(god_key.clone().into(), 1u128 << 63)].into_iter().collect(),
                    validator_count: 12,
                    sessions_per_era: 24,   // 24 hours per era.
                    bonding_duration: 90,   // 90 days per bond.
                    early_era_slash: 10000,
                    session_reward: 100,
                }),
                democracy: Some(DemocracyConfig {
                    launch_period: 120 * 24 * 14,   // 2 weeks per public referendum
                    voting_period: 120 * 24 * 28,   // 4 weeks to discuss & vote on an active referendum
                    minimum_deposit: 1000,  // 1000 as the minimum deposit for a referendum
                }),
                council: Some(CouncilConfig {
                    active_council: vec![],
                    candidacy_bond: 1000,   // 1000 to become a council candidate
                    voter_bond: 100,        // 100 down to vote for a candidate
                    present_slash_per_voter: 1, // slash by 1 per voter for an invalid presentation.
                    carry_count: 24,        // carry over the 24 runners-up to the next council election
                    presentation_duration: 120 * 24,    // one day for presenting winners.
                    approval_voting_period: 7 * 120 * 24,   // one week period between possible council elections.
                    term_duration: 180 * 120 * 24,  // 180 day term duration for the council.
                    desired_seats: 0, // start with no council: we'll raise this once the stake has been dispersed a bit.
                    inactive_grace_period: 1,   // one addition vote should go by before an inactive voter can be reaped.

                    cooloff_period: 90 * 120 * 24, // 90 day cooling off period if council member vetoes a proposal.
                    voting_period: 7 * 120 * 24, // 7 day voting period for council members.
                }),
                timestamp: Some(TimestampConfig {
                    period: 5,                  // 5 second block time.
                }),
        };
    genesis_config
}
pub struct TransactionPool {
}

impl TransactionPool {
  pub fn new() -> Self {
    TransactionPool {
    }
  }
}


impl substrate_network::TransactionPool<Hash, Block> for TransactionPool {
  fn transactions(&self) -> Vec<(Hash, UncheckedExtrinsic)> {
        unreachable!();
  }

  fn import(&self, transaction: &UncheckedExtrinsic) -> Option<Hash> {
        unreachable!();
  }

  fn on_broadcasted(&self, propagations: HashMap<Hash, Vec<String>>) {
        unreachable!();
  }
}

fn main() {
//    let backend = Arc::new(Backend::new(client_db::DatabaseSettings{
//            cache_size: None, path: PathBuf::from(r"./"), pruning:state_db::PruningMode::default(),}, FINALIZATION_WINDOW));
//    let executor = Executor::new(backend, exchange_executor::NativeExecutor::with_heap_pages(8));
    let executor = exchange_executor::NativeExecutor::with_heap_pages(8);
    let client = client::new_in_mem::<exchange_executor::NativeExecutor<exchange_executor::Executor>, Block, _>(executor, genesis_config()).unwrap();
    let param = NetworkParam {
       config: substrate_network::ProtocolConfig::default(),
       network_config: substrate_network_libp2p::NetworkConfiguration::default(),
       chain: Arc::new(client),
       on_demand: None,
       transaction_pool: Arc::new(TransactionPool::new()),
       specialization: Protocol::new(),
    };
    NetworkService::new(param, DOT_PROTOCOL_ID);
    println!("Hello, world!");
}
