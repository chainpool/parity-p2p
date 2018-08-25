extern crate substrate_network;
extern crate substrate_network_libp2p;
extern crate substrate_runtime_primitives;
extern crate substrate_primitives;
extern crate substrate_client as client;
extern crate substrate_bft as bft;

extern crate exchange_primitives;
extern crate exchange_executor;
extern crate exchange_runtime;

extern crate futures;
extern crate tokio;
extern crate ctrlc;
extern crate rhododendron;
#[macro_use]
extern crate hex_literal;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate clap;
extern crate ed25519;

use substrate_network_libp2p::AddrComponent;
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
use futures::{Future, Sink, Stream};
use tokio::runtime::Runtime;
use clap::{Arg, App};
use std::iter;
use std::net::Ipv4Addr;
use std::thread;

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
     println!("status");
     vec![2, 2]
  }

  fn on_connect(&mut self, ctx: &mut Context<Block>, who: NodeIndex, status: FullStatus) {
     println!("on_connect");
  }

  fn on_disconnect(&mut self, ctx: &mut Context<Block>, who: NodeIndex) {
     println!("on_disconnect");
  }

  fn on_message(&mut self, ctx: &mut Context<Block>, who: NodeIndex, message: message::Message<Block>) {
     println!("on_message");
  }

  fn on_abort(&mut self) {
     println!("on_abort!");
  }

  fn maintain_peers(&mut self, _ctx: &mut Context<Block>) {
     println!("maintain_peers!");
  }

  fn on_block_imported(&mut self, _ctx: &mut Context<Block>, hash: Hash, header: &Header) {
     unreachable!();
  }
}

pub type NetworkService = substrate_network::Service<Block, Protocol, Hash>;

pub type NetworkParam = substrate_network::Params<Block, Protocol, Hash>;

const FINALIZATION_WINDOW: u64 = 32;

const DOT_PROTOCOL_ID: substrate_network::ProtocolId = *b"exc";

fn genesis_config() -> GenesisConfig {
        let god_key = hex!("3d866ec8a9190c8343c2fc593d21d8a6d0c5c4763aaab2349de3a6111d64d124");
        let genesis_config = GenesisConfig {
                consensus: Some(ConsensusConfig {
                    code: include_bytes!("../runtime/wasm/target/wasm32-unknown-unknown/release/exchange_runtime.compact.wasm").to_vec(),   // TODO
                    authorities: vec![ed25519::Pair::from_seed(&god_key).public().into(),],
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
        println!("transactions");
        vec![(Hash::from(2), vec![])]
  }

  fn import(&self, transaction: &UncheckedExtrinsic) -> Option<Hash> {
       None
  }

  fn on_broadcasted(&self, propagations: HashMap<Hash, Vec<String>>) {
        println!("on_broadcasted");
  }
}

pub fn fake_justify(header: &Header) -> bft::UncheckedJustification<Hash> {
    let hash = header.hash();
    let authorities_keys = vec![
        ed25519::Pair::from_seed(&hex!("3d866ec8a9190c8343c2fc593d21d8a6d0c5c4763aaab2349de3a6111d64d124")),
    ];

    bft::UncheckedJustification::new(
        hash,
        authorities_keys.iter().map(|key| {
            let msg = bft::sign_message::<Block>(
                ::rhododendron::Vote::Commit(1, hash).into(),
                key,
                header.parent_hash
            );

            match msg {
                ::rhododendron::LocalizedMessage::Vote(vote) => vote.signature,
                _ => panic!("signing vote leads to signed vote"),
            }
        }).collect(),
        1,
    )
}

fn main() {
    let matches = App::new("parity p2p")
                     .version("0.1.0")
                     .arg(Arg::with_name("port")
                           .long("port")
                           .value_name("PORT")
                           .help("Specify p2p protocol TCP port")
                           .takes_value(true))
                      .arg(Arg::with_name("bootnodes")
                            .long("bootnodes")
                            .value_name("URL")
                            .help("Specify a list of bootnodes")
                            .takes_value(true)
                            .multiple(true))
                       .get_matches();
    let port = match matches.value_of("port") {
        Some(port) => port.parse().map_err(|_| "Invalid p2p port value specified.").unwrap(),
        None => 20222,
    };

    let mut net_conf = substrate_network_libp2p::NetworkConfiguration::new();
    net_conf.listen_address = iter::once(AddrComponent::IP4(Ipv4Addr::new(127, 0, 0, 1)))
            .chain(iter::once(AddrComponent::TCP(port)))
            .collect();
    net_conf.boot_nodes.extend(matches
            .values_of("bootnodes")
            .map_or(Default::default(), |v| v.map(|n| n.to_owned()).collect::<Vec<_>>()));
    env_logger::init();
    let executor = exchange_executor::NativeExecutor::with_heap_pages(8);
    let c = Arc::new(client::new_in_mem::<exchange_executor::NativeExecutor<exchange_executor::Executor>, Block, _>(executor, genesis_config()).unwrap());

    let mut n = 10;
    while n > 0 {
        let best_header = c.best_block_header().unwrap();
        println!("Best block: #{}", best_header.number);
        let builder = c.new_block().unwrap();
        let block = builder.bake().unwrap();
        let justification = fake_justify(&block.header);
        let justified = c.check_justification(block.header, justification).unwrap();
        c.import_block(client::BlockOrigin::File, justified, Some(block.extrinsics)).unwrap();
        thread::sleep_ms(5000);
        n = n - 1;
    }


    let param = NetworkParam {
       config: substrate_network::ProtocolConfig::default(),
       network_config: net_conf,
       chain: c.clone(),
       on_demand: None,
       transaction_pool: Arc::new(TransactionPool::new()),
       specialization: Protocol::new(),
    };
    let service = NetworkService::new(param, DOT_PROTOCOL_ID).unwrap();

    let mut runtime = Runtime::new().unwrap();
    let (exit_send, exit) = futures::sync::mpsc::channel(1);
    ctrlc::CtrlC::set_handler(move || {
      exit_send.clone().send(()).wait().expect("Error sending exit notification");
    });

    runtime.block_on(exit.into_future()).expect("Error running informant event loop");
}
