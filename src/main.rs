use tonic::transport::Channel;
use routingpackage::routing_client::RoutingClient;
use routingpackage::DirectionRequest;
use routingpackage::HelloRequest;

use std::time::Instant;
use std::io::{self, Write}; // For reading user input

mod routingpackage {
    tonic::include_proto!("routingpackage"); // Path to your .proto file
}

//  .filter(Some("tonic"), log::LevelFilter::Debug)
// Function to initialize the connection and client
async fn init_client() -> Result<RoutingClient<Channel>, Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://localhost:50051")
        .connect()
        .await?;

    Ok(RoutingClient::new(channel))
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = init_client().await?;
    println!("connection");
    loop {
        println!("Enter 0 to exit or 1 to make a request:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let request = tonic::Request::new(HelloRequest {
            name: "World".into(), // Example name
        });

        let start_time = Instant::now();
        let response = client.say_hello(request).await?;
        let end_time = Instant::now();

        println!("Response: {:?}", response);
        println!("Function took {} milliseconds", end_time.duration_since(start_time).as_millis());
    }

    Ok(())
}
