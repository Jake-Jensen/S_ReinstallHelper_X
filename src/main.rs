// Jake Jensen, 2023

#![allow(non_snake_case)]
#![allow(unused_parens)]

use std::io::Read;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::process::exit;

fn _Log(Message: &str) {
    println!("[Debug] {}", Message);
}

fn GetIP_MainServer() -> IpAddr {
    let mut ReturnIP = IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255));
    let hostname = "sapphirelabs.online";
    match hostname.parse::<IpAddr>() {
        Ok(ip) => println!("IP address is {}", ip),
        Err(_) => {
            // Perform lookup
            let ips: Vec<IpAddr> = (hostname, 0)
                .to_socket_addrs()
                .unwrap()
                .filter_map(|a| match a {
                    SocketAddr::V4(a) => Some(IpAddr::V4(*a.ip())),
                    SocketAddr::V6(a) => Some(IpAddr::V6(*a.ip())),
                })
                .collect();
            for ip in ips {
                match ip {
                    IpAddr::V4(ipv4) => {println!("IP address is {}", ipv4); 
                        ReturnIP = ip},
                    IpAddr::V6(ipv6) => println!("IP address is {}", ipv6),
                }
            }
        }
    }
    return ReturnIP;
    

}

fn Download_url(url: &str) -> Result<String, reqwest::Error> {
    // Send a GET request to the URL
    let mut response = reqwest::blocking::get(url)?;

    // Read the response body into a string
    let mut content = String::new();
    response
        .read_to_string(&mut content)
        .expect("Failed to read response");

    Ok(content)
}

fn DoDownload(URL: &str) {
    let url = URL;
    match Download_url(url) {
        Ok(content) => println!("Downloaded {} bytes from {}", content.len(), url),
        Err(e) => println!("Failed to download {}: {:?}", url, e),
    }
}

fn TestForMainServerConnection() -> bool {
    let MainServerIP = GetIP_MainServer();
    let MainServerPort = ":80";
    let Combined = format!("{}{}", MainServerIP.to_string(), MainServerPort.to_string());
    match TcpStream::connect(Combined) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn TestForConnection() -> bool {
    _Log("Attempting connection to 8.8.8.8 on port 53.");
    match TcpStream::connect("8.8.8.8:53") {
        Ok(_) => {_Log("Successfully connected to 8.8.8.8"); true},
        Err(_) => {_Log("Failed to connect to 8.8.8.8."); false},
    }
}

fn main() {

    #[cfg(target_os = "linux")]
    println!("Running in Linux mode.");

    #[cfg(target_os = "windows")]
    println!("Running in Windows mode.");

    println!("----- Sapphire's Reinstall Helper -----");
    println!("------------- Version 3.0 -------------");

    // Test for an internet connection
    if (!TestForConnection()) {
        println!("Failed to get an internet connection.");
    } else {
        if (!TestForMainServerConnection()) {
            println!("Failed to connect to the main distribution server. Aborting.");
            println!("Fake abort for debug testing.");
            exit(-1);
        } else {
            println!("Successfully got the main server IP and confirmed the connection.");
        }
    }
}
