use log::debug;
pub mod logparse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let log_content = std::fs::read_to_string("/var/log/pacman.log")?;

    let start = std::time::Instant::now();
    let transactions = logparse::parse_log(&log_content)?;
    let duration = start.elapsed();
    debug!("Parsing took {:?}", duration);

    println!("Parsed {} unique transactions", transactions.len());

    for (timestamp, event_list) in transactions.iter().rev().take(5) {
        println!("Timestamp {}: {} events", timestamp, event_list.len());
        // dbg!(event_list);
    }

    Ok(())
}
