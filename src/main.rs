use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::prelude::*;
use std::net::Shutdown;
use std::io::Read;
use std::io::{Error, ErrorKind};
use std::str;

use std::net::TcpStream;


fn connect(){
    // test using: socat - TCP-LISTEN:34254
    let addr = "localhost:34254";
    let mut socket = TcpStream::connect(addr).unwrap();

    // no shit - all the unix socket stuff just works here (or at least compiles)
    // thought: use nom to handle basic parsing (eg len-prefixed json blobs) then use some json lib (eg serde-rs/json) to parse that (better than hand rolling my own format..)
    socket.write_all(b"echo client sez: hello world")
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
                        // HMMM: the https://doc.rust-lang.org/beta/std/str/struct.Utf8Error.html Utf8Error type does provide
                        // valid_up_to which tells you when it hit an invalid byte, could do something with that (eg store any invalid bytes and retry)
                        // could just drop the unicode thing entirely and use some kinda parser/combinator'd type to control (todo: figure out thing to control)
                        // OR: do a (x-byte msg len uint, proto'd msg) stream such that all read ops can just be of known max len
                        // still have potential problem if read times out/finishes w/o full msg, could need complex logic... ugh
                        //hmm, looks like nom handles this?
                        // > nom has been designed for a correct behaviour with partial data: if there is not enough data to decide, nom will tell you it needs more instead of silently returning a wrong result. Whether your data comes entirely or in chunks, the result should be the same.
                        // nom website has example as hex color parser, could do that and have program control, idk, LED? Color temparature of some widget? w/e, works for me.. interesting project
                        // AH, IDEA: have format be list of (hex color code, duration) and use to control some sort of color-changing thingy
                        // or, simplest way of doing it: have it echo back terminal control codes to display requested color or something? idk, plenty of options here that are nontrivial enough to be interesting..

                        str::from_utf8(&buffer[0..n])
                          .map_err( |e| Error::new(ErrorKind::InvalidInput, e))
                          .and_then(|s| {
                            println!("got string from stream: {}", s);
                            socket.write(&buffer[0..n])
                          }).map(|n| {
                              println!("wrote {} bytes", n);
                          })
                    });

                if res.is_err() {break res}
            }
        }).or_else( |err| {

            println!("reading from stream failed, err: {}", err);
            // note: weird, I guess this just no-ops instead of failing if stream is already ded?
            socket.shutdown(Shutdown::Both)

        }).expect("panic! at the disco (couldn't shut down socket for some reason)")

}

fn handle_client(mut socket: UnixStream) {
    // note to self: this is ugly af..
    socket.write_all(b"echo server sez: hello world")
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
                        // HMMM: the https://doc.rust-lang.org/beta/std/str/struct.Utf8Error.html Utf8Error type does provide
                        // valid_up_to which tells you when it hit an invalid byte, could do something with that (eg store any invalid bytes and retry)
                        // could just drop the unicode thing entirely and use some kinda parser/combinator'd type to control (todo: figure out thing to control)
                        // OR: do a (x-byte msg len uint, proto'd msg) stream such that all read ops can just be of known max len
                        // still have potential problem if read times out/finishes w/o full msg, could need complex logic... ugh
                        //hmm, looks like nom handles this?
                        // > nom has been designed for a correct behaviour with partial data: if there is not enough data to decide, nom will tell you it needs more instead of silently returning a wrong result. Whether your data comes entirely or in chunks, the result should be the same.
                        // nom website has example as hex color parser, could do that and have program control, idk, LED? Color temparature of some widget? w/e, works for me.. interesting project
                        // AH, IDEA: have format be list of (hex color code, duration) and use to control some sort of color-changing thingy
                        // or, simplest way of doing it: have it echo back terminal control codes to display requested color or something? idk, plenty of options here that are nontrivial enough to be interesting..

                        str::from_utf8(&buffer[0..n])
                          .map_err( |e| Error::new(ErrorKind::InvalidInput, e))
                          .and_then(|s| {
                            println!("got string from stream: {}", s);
                            socket.write(&buffer[0..n])
                          }).map(|n| {
                              println!("wrote {} bytes", n);
                          })
                    });

                if res.is_err() {break res}
            }
        }).or_else( |err| {

            println!("reading from stream failed, err: {}", err);
            // note: weird, I guess this just no-ops instead of failing if stream is already ded?
            socket.shutdown(Shutdown::Both)

        }).expect("panic! at the disco (couldn't shut down socket for some reason)")

}


fn main() {

  connect();

  // does this create a socket? unwrap will panic if it fails..
  let listener = UnixListener::bind("/tmp/rust-unix-test.sock").unwrap();

  // accept connections and process them, spawning a new thread for each one
  for stream in listener.incoming() {
      match stream {
          Ok(stream) => {
              /* connection succeeded */
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
