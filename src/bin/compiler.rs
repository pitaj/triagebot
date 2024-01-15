use triagebot::agenda;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && args[1] == "backlog_bonanza" {
        let agenda = agenda::compiler_backlog_bonanza();
        print!("{}", agenda.call().await?);
        return Ok(());
    }

    eprintln!("Usage: compiler (backlog_bonanza)");

    Ok(())
}
