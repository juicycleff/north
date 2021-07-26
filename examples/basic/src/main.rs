#[macro_use]
extern crate north;

use north::ServiceTrait;

#[tokio::main]
pub async fn main() {
    let app = north::power().graceful_shutdown().up();
    app.start().await
}
