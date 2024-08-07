#![warn(clippy::missing_docs_in_private_items)]

//! Server of Fractal is back end process running behind the webview to serve
//! the front end
//!
//! Server uses gRPC/tonic to serve all request (other than the front end assets
//! for webview) defined in <workspace>/proto/* folder
//! The API primarily consists of:
//! * `script`: Evaluates the user defined logic to funnel into GPU logic
//! * `engine`: Compiles a graphical pipeline that accepts values defined in
//!   script phase
//! * `stream`: Manages the buffer calculated by the `engine` step

mod dispatcher;

use std::net::SocketAddr;

use tonic::transport::Server;

use dispatcher::{EventLayer, Hello};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let layer = tower::ServiceBuilder::new().layer(EventLayer);

    Server::builder()
        .layer(layer)
        .add_service(Hello::service())
        .serve(addr)
        .await?;

    Ok(())
}
