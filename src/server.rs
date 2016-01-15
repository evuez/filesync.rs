extern crate chan;
extern crate handlebars;
extern crate hyper;
extern crate rustc_serialize;
extern crate websocket;
extern crate walkdir;

use std::collections::BTreeMap;
use std::default::Default;
use std::fs::File;
use std::io::{Write, Read};
use std::sync::{Arc, Mutex};
use std::thread;
use self::handlebars::Handlebars;
use self::hyper::Server as HttpServer;
use self::hyper::server::Handler;
use self::hyper::net::Fresh;
use self::hyper::server::request::Request;
use self::hyper::server::response::Response;
use self::rustc_serialize::json::{Json, ToJson};
use self::walkdir::WalkDir;
use self::websocket::{Message, Sender};
use watcher;


#[derive(Default)]
pub struct Server<'a> {
  http_addr: &'a str,
  websocket_addr: &'a str,
  watchpath: String,
}

impl<'a> Server<'a> {
  pub fn bind(http_addr: &'a str, websocket_addr: &'a str) -> Server<'a> {
    Server {
      http_addr: http_addr,
      websocket_addr: websocket_addr,
      ..Default::default()
    }
  }

  pub fn watch(&mut self, path: String) {
    self.watchpath = path;
  }

  pub fn start(&self) {
    http_server(self.http_addr);
    websocket_server(self.websocket_addr, self.watchpath.clone());
  }
}


fn websocket_server(addr: &str, watchpath: String) {
  let websocket_server = websocket::Server::bind(addr).unwrap();
  let (sx, rx) = chan::sync(0);

  let treepath = watchpath.clone();
  let rootpath = watchpath.clone();

  thread::spawn(move || {
    watcher::watch(&watchpath[..], sx);
  });
  let wait_group = chan::WaitGroup::new();

  let tree = Arc::new(Mutex::new(WalkDir::new(treepath)
    .into_iter()
    .filter_map(|e| e.ok())
    .map(|e| e.path().display().to_string().replace(&rootpath[..], ""))
    .collect::<Vec<String>>()));


  for connection in websocket_server {
    let tree = tree.clone();
    let wait_group = wait_group.clone();
    let rx = rx.clone();
    wait_group.add(1);

    thread::spawn(move || {
      let request = connection.unwrap().read_request().unwrap();

      let response = request.accept();
      let mut client = response.send().unwrap();

      for fop in rx {
        if let Some(path) = fop.path.to_str() {

          let mut code = String::new();
          let mut file = File::open(path).unwrap();
          file.read_to_string(&mut code).unwrap();

          let mut data: BTreeMap<String, Json> = BTreeMap::new();
          data.insert("code".to_string(), code.to_json());
          data.insert("tree".to_string(), tree.lock().unwrap().to_json());

          let message = Message::text(data.to_json().to_string());
          client.send_message(&message).unwrap();
        }
      }

      wait_group.done();
    });
  }

  wait_group.wait();
}




fn render() -> String {
  let mut handlebars = Handlebars::new();

  let mut file = File::open("static/filesync.html").unwrap();
  let mut template = String::new();
  file.read_to_string(&mut template).unwrap();

  handlebars.register_template_string("filesync", template).ok().unwrap();

  let mut data: BTreeMap<String, Json> = BTreeMap::new();

  data.insert("websocket_addr".to_string(), "127.0.0.1:1234".to_json());

  handlebars.render("filesync", &data).ok().unwrap()
}


fn http_handler(_: Request, response: Response<Fresh>) {
  let mut response = response.start().unwrap();

  response.write_all(render().as_bytes()).unwrap();
  response.end().unwrap();
}


fn http_server(addr: &str) {
  let addr: String = String::from(addr);

  thread::spawn(move || {
    let http_server = HttpServer::http(&addr[..]).unwrap();
    http_server.handle(http_handler).unwrap();
  });
}
