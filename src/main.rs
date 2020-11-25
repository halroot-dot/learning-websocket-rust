extern crate ws;
use ws::listen;
fn main() {
    let listen_url = "0.0.0.0:8082";
    //let listen_url ="127.0.0.1:8082";

    println!("Runing Server");

    listen(listen_url, |out| {
        move |msg| {
            let response: String = format!("Hello {}", msg);
            println!("{}", response);
            out.send(response)
        }
    })
    .unwrap()
    // TODO: Addressが既に使われている場合、unwrap()でErrが返されpanicになる
}
