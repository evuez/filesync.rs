mod watcher;
mod server;
mod guard;

use server::Server;
use std::env;
use std::process;


const HTTP_ADDR: &'static str = "127.0.0.1:8080";
const WEBSOCKET_ADDR: &'static str = "127.0.0.1:1234"; // Not used yet, see server.rs:120
//const KEY: &'static str = "long-and-random-string";

fn main() {
  let args: Vec<String> = env::args().collect();

  match args.len() {
    2 => {},
    _ => { println!("Missing `path` arg."); process::exit(1); }
  }

  let mut server = Server::bind(HTTP_ADDR, WEBSOCKET_ADDR);

  server.watch(args[1].clone());
  server.start();
}
