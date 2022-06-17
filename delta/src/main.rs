use futures::prelude::*;
use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::{Swarm, SwarmEvent, dial_opts::DialOpts};
use libp2p::{identity, Multiaddr, PeerId};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    // https://docs.rs/libp2p/latest/libp2p/tutorials/ping/index.html#transport
    let transport = libp2p::development_transport(local_key).await?;

    // Create a ping network behaviour.
    //
    // https://docs.rs/libp2p/latest/libp2p/tutorials/ping/index.html#network-behaviour
    // For illustrative purposes, the ping protocol is configured to
    // keep the connection alive, so a continuous sequence of pings
    // can be observed.
    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));


    // https://docs.rs/libp2p/latest/libp2p/tutorials/ping/index.html#swarm

    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

        // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // https://docs.rs/libp2p/latest/libp2p/tutorials/ping/index.html#multiaddr
    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {}", addr)
    }


    // https://docs.rs/libp2p/latest/libp2p/tutorials/ping/index.html#continuously-polling-the-swarm
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            _ => {}
        }
    }

    Ok(())
}