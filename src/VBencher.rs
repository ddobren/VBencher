use std::io::{self, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};
use std::thread;
use std::io::Read;

fn main() -> io::Result<()> {
    loop {
        println!("\x1b[1mWelcome to VBencher!\x1b[0m");

        // Prompt user to enter URL for benchmark
        let mut url = String::new();
        print!("Enter URL to benchmark (empty to quit): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut url)?;
        let url = url.trim();
        if url.is_empty() {
            break;
        }

        // Parse URL to get host, protocol, and path
        let (protocol, host, path) = match url.find("://") {
            Some(pos) => {
                let (protocol, rest) = url.split_at(pos + 3);
                match rest.find('/') {
                    Some(path_pos) => (&protocol[..protocol.len() - 3], &rest[..path_pos], &rest[path_pos..]),
                    None => (&protocol[..protocol.len() - 3], rest, "/"),
                }
            },
            None => match url.find('/') {
                Some(path_pos) => ("http", &url[..path_pos], &url[path_pos..]),
                None => ("http", url, "/"),
            }
        };

        // Ask user how many requests to send
        let num_requests = loop {
            print!("How many requests do you want to send? ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            match input.trim().parse::<u32>() {
                Ok(num) => break num,
                Err(_) => println!("Please enter a valid number."),
            }
        };

        // Ask user if they want to enable keep-alive connections
        let keep_alive_enabled = loop {
            print!("Enable keep-alive connections? (y/n): ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            match input.trim().to_lowercase().as_str() {
                "y" => break true,
                "n" => break false,
                _ => println!("Please enter 'y' or 'n'."),
            }
        };

        // Execute requests on the website
        let mut total_time = Duration::from_secs(0);
        for i in 0..num_requests {
            println!("\x1b[36mSending request {}...\x1b[0m", i + 1);
            let start_time = Instant::now();
            let addr = (host, get_port_for_protocol(protocol)).to_socket_addrs()?.next().ok_or(io::ErrorKind::AddrNotAvailable)?;
            let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(10))?;
            let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\nConnection: {}\r\n\r\n", path, host, if keep_alive_enabled {"keep-alive"} else {"close"});
            stream.write_all(request.as_bytes())?;
            let mut buffer = [0; 4096];
            loop {
                match stream.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(_) => continue,
                    Err(_) => break,
                }
            }
            let elapsed_time = start_time.elapsed();
            total_time += elapsed_time;
            thread::sleep(Duration::from_secs(1)); // wait one second between requests
        }

        // Print summary
        let avg_time_ms = total_time.as_millis() / num_requests as u128;
        println!("\x1b[32mCompleted {} requests in {:.2} seconds (avg response time: {} ms)\x1b[0m",
                 num_requests, total_time.as_secs_f64(), avg_time_ms);

        // Ask user if they want to repeat the benchmark
        let repeat = loop {
            print!("Do you want to benchmark another website? (y/n): ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            match input.trim().to_lowercase().as_str() {
                "y" => break true,
                "n" => break false,
                _ => println!("Please enter 'y' or 'n'."),
            }
        };
        if !repeat {
            break;
        }
    }

    println!("Thank you for using VBencher!");
    Ok(())
}

fn get_port_for_protocol(protocol: &str) -> u16 {
    match protocol {
        "http" => 80,
        "https" => 443, 
        _ => panic!("Unsupported protocol."),
    }
}
