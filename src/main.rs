use dns_lookup::lookup_host;
use fastping_rs::{
    PingResult::{Idle, Receive},
    Pinger,
};
use rasciigraph::{plot, Config};
use std::env;
use std::net::IpAddr;
use std::str::FromStr;

struct Target {
    hostname: String,
    ip: IpAddr,
}

fn main() {
    let target = match get_target() {
        Ok(target) => target,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    //create pinger
    let (pinger, results) = match Pinger::new(None, None) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating Pinger: {}", e),
    };

    pinger.add_ipaddr(&target.ip.to_string());
    pinger.run_pinger();

    let mut pings = Vec::new();

    loop {
        match results.recv() {
            Ok(result) => match result {
                Idle { addr } => {
                    println!("Idle Address {}.", addr);
                }
                Receive { rtt, .. } => {
                    pings.push(rtt.as_millis() as f64);
                    let p = plot(
                        pings.clone(),
                        Config::default()
                            .with_height(9)
                            .with_caption(target.hostname.to_string()),
                    );
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    println!("{}", p);
                }
            },
            Err(_) => panic!("Worker threads disconnected before the solution was found!"),
        }
    }
}

//get target from command line arg
fn get_target() -> Result<Target, String> {
    let arg: Vec<String> = env::args().collect();
    if arg.len() < 2 {
        print_help();
        return Err("No target provided".to_string());
    }

    let target_string = &arg[1];

    //first try to resolve as pure IP
    let target: Target = match IpAddr::from_str(&target_string) {
        Ok(ip) => Target {
            hostname: String::from(""),
            ip,
        },
        //otherwise try to resolve as hostname
        Err(_e) => match lookup_host(&target_string) {
            Ok(ips) => Target {
                hostname: arg[1].clone(),
                ip: ips[0],
            },
            Err(_e) => {
                return Err(format!(
                    "'{}' did not resolve to a valid IP address",
                    target_string
                ))
            }
        },
    };

    return Ok(target);
}

fn print_help() {
    println!("studs <target>")
}
