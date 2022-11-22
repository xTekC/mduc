mod xcli;
use futures::executor::block_on;
use xcli::cli::cli_main;

#[tokio::main]
async fn main() {
    block_on(cli_main());
}
