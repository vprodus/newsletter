use std::net::TcpListener;
use newsletter::configuration::get_configuration;
use newsletter::routes::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration()
                          .expect("Failed to read configuration.");
    // We have removed the hard-coded `8000` - it's now coming from our settings! 
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address)
                      .expect("Failed to bind random port");
    println!("server runing at {}", &address);
    run(listener)?.await
}
