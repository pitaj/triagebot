use triagebot::agenda;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && args[1] == "planning" {
        let agenda = agenda::types_planning();
        print!("{}", agenda.call().await?);
        return Ok(());
    }

    eprintln!("Usage: types (planning)");

    Ok(())
}
