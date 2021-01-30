use std::env;
use std::str::FromStr;
//use std::io::{Read,Write};
use std::net::IpAddr;
use fastping_rs::Pinger;
use fastping_rs::PingResult::{Idle, Receive};
use dns_lookup::lookup_host;
use rasciigraph::{plot, Config};

struct Target{
    hostname: String,
    ip: IpAddr
}

fn main() {
    let target = get_target();

    //create pinger
    let (pinger, results) = match Pinger::new(None, None) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating Pinger: {}", e)
    };

    pinger.add_ipaddr(&target.ip.to_string());
    pinger.run_pinger();

    let mut pings = Vec::new();

    loop {
        match results.recv() {
            Ok(result) => {
                match result {
                    Idle{addr} => {
                        println!("Idle Address {}.", addr);
                    },
                    Receive{rtt,..} => {
                        pings.push(rtt.as_millis() as f64);
                        let p = plot(pings.clone(), Config::default().with_height(9).with_caption(target.hostname.to_string()));
                        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        println!("{}", p);
                    }
                }
            },
            Err(_) => panic!("Worker threads disconnected before the solution was found!"),
        }
    }
}

//get target from command line arg
fn get_target() -> Target {

    let arg:Vec<String>= env::args().collect();

    //first try to resolve as pure IP
    let target:Target = match IpAddr::from_str(&arg[1]) {
        Ok(ip) => Target {hostname : String::from(""), ip : ip},
        //otherwise try to resolve as hostname
        Err(_e) => match lookup_host(&arg[1]) {
            Ok(ips) => Target {hostname : arg[1].clone() , ip : ips[0]},
            Err(_e) => panic!("Could not resovle!")
        }
    };

    return target;
}

