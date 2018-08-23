use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::prelude::*;
use std::net::Shutdown;
use std::io::Read;
use std::io::{Error, ErrorKind};
use std::str;
use std::result::Result;

// required to have error side of Result be the same for handle_client
fn handle_input(bytes: &[u8]) -> Result<&str, Error> {
    match str::from_utf8(&bytes) {
        Ok (s) => {
            Ok(s)
        }
        Err (err) => {
            // TODO: I guess I can just pass error along with type here? cool!
            // TODO: maybe create some enum of possible domain errors here instead
            Err(Error::new(ErrorKind::InvalidInput, err))
        }
    }
}

// todo: echo all text instead of insta-shutdown
fn handle_client(mut socket: UnixStream) {
    let res = socket.write_all(b"echo server sez: hello world")
        .and_then( |_| {
            loop {

                // reused on each iteration of loop
                let mut buffer = [0; 64];
                // read up to 64 bytes
                let res =
                    socket.read(&mut buffer[..])
                    .and_then(|n| {
                        println!("read {} bytes", n);
                        // NOTE: can hit failure here if a unicode char boundary is hit at, eg, bytes 64, 65
                        handle_input(&buffer[0..n])
                          .and_then(|s| {
                            println!("got string from stream: {}", s);
                            socket.write(&buffer[0..n])
                          }).and_then(|n| {
                              println!("wrote {} bytes", n);
                              Ok(())
                          })
                    });

                if res.is_err() {break res}
            }
    });

    match res {

      Ok (_) => {
          println!("???, this shouldn't terminate without an error");
      }
      Err (err) => {
          println!("reading from stream failed, err: {}", err);
          // note: weird, I guess this just no-ops instead of failing if stream is already ded?
          socket.shutdown(Shutdown::Both).expect("shutdown function failed");
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
              //Q: what does || actually do? it looks like lambda syntax for an empty function, maybe that's it
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
