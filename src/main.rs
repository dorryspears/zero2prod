use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    // We return the application address to the caller!
    //print the port to the console
    println!("Listening on port {}", port);
    // We retrieve the port assigned to us by the OS
    run(listener)?.await
}
