use std::io::{self, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};
use std::thread;
use std::io::Read;

fn main() -> io::Result<()> {
    loop {
        println!("\x1b[1mWelcome to VBencher!\x1b[0m");

        // User enters URL for benchmark
        let mut url = String::new();
        print!("Enter URL to benchmark (empty to quit): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut url)?;
        let url = url.trim();
        if url.is_empty() {
            break;
        }

        // Parse URL to get host and path
        let url_parts: Vec<&str> = url.splitn(2, "/").collect();
        let host = url_parts[0];
        let path = if url_parts.len() > 1 { url_parts[1] } else { "/" };

        // Ask user how many requests to send
        let mut input = String::new();
        print!("How many requests do you want to send? ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        let num_requests = input.trim().parse::<u32>().unwrap_or(5);

        // Execute requests on the website
        let mut total_time = Duration::from_secs(0);
        for i in 0..num_requests {
            println!("\x1b[36mSending request {}...\x1b[0m", i + 1);
            let start_time = Instant::now();
            let addr = (host, 80).to_socket_addrs()?.next().ok_or(io::ErrorKind::AddrNotAvailable)?;
            let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(10))?;
            let request = format!("GET /{} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path, host);
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
        let mut input = String::new();
        print!("Do you want to benchmark another website? (y/n): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            break;
        }
    }

    println!("Thank you for using VBencher!");
    Ok(())
}
