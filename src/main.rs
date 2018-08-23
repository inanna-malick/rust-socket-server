use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::prelude::*;
use std::net::Shutdown;
use std::io::Read;
use std::str;

// todo: echo all text instead of insta-shutdown
fn handle_client(mut socket: UnixStream) {
    socket.write_all(b"echo server sez: hello world").unwrap();

    // reused on each iteration of loop
    let mut buffer = [0; 64];

    loop {

      // read up to 64 bytes
      match socket.read(&mut buffer[..]) {
          Ok(n) => {
              // todo expose failure possiblity?
              // Q: why does that need '&'? It's not like it mutates the buffer..
              // Guess: it's just transfering ownership, provides guarantee that, eg, no other thread
              // can nuke the buffer while its in use
              let s = str::from_utf8(&buffer[0..n]).unwrap();
              println!("got string from stream: {}", s);

              //TODO handle failure
              socket.write(&buffer[0..n]).unwrap();
          }
          Err (err) => {
              println!("reading from stream failed, err: {}", err);
              socket.write_all(b"bye bye now").unwrap();
              // TODO: magic string that triggers this
              socket.shutdown(Shutdown::Both).expect("shutdown function failed");
              break;
          }
      }
    }


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

  // todo: catch CTRL-C and do cleanup
  println!("Hello, world!");
}
