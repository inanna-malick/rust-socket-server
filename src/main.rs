use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::prelude::*;
use std::net::Shutdown;


fn handle_client(mut socket: UnixStream) {
    socket.write_all(b"hello world").unwrap();
    socket.write_all(b"bye bye now").unwrap();
    socket.shutdown(Shutdown::Both).expect("shutdown function failed");
}


fn main() {



  // does this create a socket? unwrap will panic if it fails..
  let listener = UnixListener::bind("/tmp/rust-unix-test.sock").unwrap();

  // accept connections and process them, spawning a new thread for each one
  for stream in listener.incoming() {
      match stream {
          Ok(stream) => {
              /* connection succeeded */
              //Q: what does || actually do?
              thread::spawn(|| handle_client(stream));
          }
          Err(err) => {
              /* connection failed */
              println!("Connection failed, err: {}", err);
              break;
          }
      }
  }


  println!("Hello, world!");
}
