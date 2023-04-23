use std::env;
// use psutil::process::Process;
use std::net::TcpStream;
use std::process::Command;
// use std::thread;
// use std::time::Duration;

use netstat2::*;
use psutil::process::processes;

fn get_open_ports_by_process(port: u16) -> u32 {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap();
    let mut pid = 1;
    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                if tcp_si.local_port == port {
                    println!("TCP {}: {:?}", tcp_si.local_port, si.associated_pids);
                    pid = si.associated_pids[0];
                    return pid;
                }
            }
            ProtocolSocketInfo::Udp(udp_si) => {
                if udp_si.local_port == port {
                    println!("Port: {} Pid: {:?}", udp_si.local_port, si.associated_pids);
                    pid = si.associated_pids[0];
                    return pid;
                }
            }
        }
    }
    pid
}

fn kill_process(pid: u32) -> std::io::Result<()> {
    println!("Got {:?}", pid);
    Command::new("kill")
        .arg("-9")
        .arg(format!("{}", pid))
        .status()?;
    Ok(())
}

fn main() {
    let hostname = "localhost";
    // let port = 2222;

    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check that we have at least one argument (the program name)
    if args.len() < 2 {
        println!("Usage: {} <port>", args[0]);
        return;
    }

    // Parse the port number from the argument
    let port = match args[1].parse::<u16>() {
        Ok(port) => port,
        Err(_) => {
            println!("Invalid port number: {}", args[1]);
            return;
        }
    };

    match TcpStream::connect(format!("{}:{}", hostname, port)) {
        Ok(_) => println!("Port {} is listening.", port),
        Err(_) => println!("Port {} is not listening.", port),
    }

    let processes = processes().unwrap();

    // thread::sleep(Duration::from_secs(1));

    // println!(
    //     "{:>6} {:>4} {:>4} {:.100}",
    //     "PID", "%CPU", "%MEM", "COMMAND"
    // );

    // for p in processes {
    //     let mut p = p.unwrap();

    // TODO the percent formatting is not working
    // println!("{:>6}", p.connections());
    // println!("{:?}", p.);
    // }
    let pid = get_open_ports_by_process(port);
    if pid == 1 {
        println!("No process found");
        return;
    }
    for p in processes {
        let p = p.unwrap();
        if p.pid() == pid as u32 {
            println!("Found matching command: {:?}", p);
        }
    }
    match kill_process(pid) {
        Ok(_) => println!("Process {} terminated", pid),
        Err(e) => println!("Failed to terminate process {}: {}", pid, e),
    }
}
