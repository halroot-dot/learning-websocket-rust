extern crate ws;
use ws::listen;

fn main() {
    listen("127.0.0.1:8080", |out| {
        move |msg|{
            let response: String = format!("Hello {}", msg);
            out.send(response)
        }
    }).unwrap()
}
