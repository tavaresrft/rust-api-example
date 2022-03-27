mod cli;
mod server;

use anyhow::Result;


use server::start_server;

#[actix_web::main]
async fn main() -> Result<()> {

    let args = cli::Cli::parse();
    start_server(args.host.as_str(), args.port).await?;

    Ok(())
}