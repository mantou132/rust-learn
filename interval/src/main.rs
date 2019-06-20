// extern crate futures;
// extern crate tokio;

// mod interval;
// mod stream;

// use self::interval::Interval;
// use self::stream::IntervalStream;
// use tokio::prelude::*;

// fn main() {
//     let interval = Interval::from_millis(500); // half a second
//     let interval_stream = IntervalStream::new(interval);
//     let future = interval_stream.for_each(|curr| {
//         println!("Counter: {}", curr);
//         futures::future::ok(())
//     });

//     tokio::run(future)
// }

// extern crate tokio;

// use tokio::prelude::*;
// use tokio::fs;
// use std::env::args;

// fn main() {
//     let future = future::poll_fn(|| {
//         for dir in args().skip(1) {
//             let future = fs::read_dir(dir)
//                 .flatten_stream()
//                 .map_err(|e| eprintln!("Error reading directory: {}", e))
//                 .for_each(|entry| {
//                     println!("{:?}", entry.path());
//                     future::ok(())
//                 })
//                 ;
//             tokio::spawn(future);
//         }
//         Ok(Async::Ready(()))
//     });
//     tokio::run(future)
// }

// extern crate tokio;

// use std::net::{ToSocketAddrs, SocketAddr};
// use tokio::io::{read_to_end, write_all};
// use tokio::net::TcpStream;
// use tokio::prelude::*;
// use std::fs::File;

// fn resolve_addr(host: &str) -> Result<SocketAddr, String> {
//     let mut addr_iter = match host.to_socket_addrs() {
//         Ok(addr_iter) => addr_iter,
//         Err(e) => return Err(format!("Invalid host name {:?}: {:?}", host, e)),
//     };
//     match addr_iter.next() {
//         None => Err(format!("No addresses found for host: {:?}", host)),
//         Some(addr) => Ok(addr),
//     }
// }

// fn download(host: String, path: String, filename: String) -> impl Future<Item=(), Error=()> {
//     let addr = match resolve_addr(&host) {
//         Ok(addr) => addr,
//         Err(e) => {
//             eprintln!("Error resolving address: {}", e);
//             return future::Either::A(future::err(()));
//         }
//     };
//     let req_body = format!(
//         "GET {} HTTP/1.1\r\nHost: {}:80\r\nConnection: close\r\n\r\n",
//         path,
//         host,
//         );

//     future::Either::B(TcpStream::connect(&addr)
//         .and_then(|stream| {
//             write_all(stream, req_body).and_then(|(stream, _body)| {
//                 let buffer = vec![];
//                 read_to_end(stream, buffer).and_then(|(_stream, buffer)| {
//                     File::create(filename).and_then(|mut file| {
//                         file.write_all(&buffer)
//                     })
//                 })
//             })
//         }).map_err(|e| eprintln!("Error occured: {:?}", e)))
// }

// fn main() {
//     tokio::run(future::poll_fn(|| {
//         let mut args = std::env::args().skip(1);
//         loop {
//             match (args.next(), args.next(), args.next()) {
//                 (Some(host), Some(path), Some(filename)) => {
//                     tokio::spawn(download(host, path, filename));
//                 }
//                 _ => return Ok(Async::Ready(())),
//             }
//         }
//     }))
// }

extern crate tokio;

use tokio::prelude::*;
use tokio::net::TcpListener;
use tokio::codec::{Decoder, LinesCodec};

fn main() {
    let addr = "127.0.0.1:3000".parse().expect("couldn't parse address");
    let listener = TcpListener::bind(&addr).expect("couldn't bind address");
    let future = listener
        .incoming()
        .for_each(|socket| {
            let lines_codec = LinesCodec::new();
            let (sink, stream) = lines_codec.framed(socket).split();

            let future = sink
                .send(String::from("Welcome to the echo server"))
                .and_then(|sink| {
                    let stream = stream
                        .map(|line| {
                            println!("{}", line);
                            format!("You said: {}", line)
                        })
                        ;
                    sink.send_all(stream)
                        .map(|_| println!("Connection closed"))
                })
                .map_err(|e| eprintln!("Error reading directory: {}", e))
                ;
            tokio::spawn(future);
            future::ok(())
        })
        .map_err(|e| eprintln!("An error occurred: {:?}", e))
        ;
    tokio::run(future)
}