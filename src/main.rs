//use std::env;
//use std::io::{Read,Write};
//use std::net::IpAddr;
use fastping_rs::Pinger;
use fastping_rs::PingResult::{Idle, Receive};
//#[macro_use]
use dns_lookup::lookup_host;
use rasciigraph::{plot, Config};

fn main() {
    let hostname = "google.com";
    let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();
    println!("{}", ips[0]);
    let (pinger, results) = match Pinger::new(None, None) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating Pinger: {}", e)
    };

    pinger.add_ipaddr(&ips[0].to_string());
    pinger.run_pinger();

    let mut pings = Vec::new();

    loop {
        match results.recv() {
            Ok(result) => {
                match result {
                    Idle{addr} => {
                        println!("Idle Address {}.", addr);
                    },
                    Receive{rtt, addr} => {
                        pings.push(rtt.as_millis() as f64);
                        let p = plot(pings.clone(), Config::default().with_height(5).with_caption(hostname.to_string()));
                        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        println!("{}", p);
                    }
                }
            },
            Err(_) => panic!("Worker threads disconnected before the solution was found!"),
        }
    }
}


