pub mod logparse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_content = std::fs::read_to_string("/var/log/pacman.log")?;

    let transactions = logparse::parse_log(&log_content)?;

    println!("Parsed {} unique transactions", transactions.len());

    for (timestamp, event_list) in transactions.iter().rev().take(5) {
        println!("Timestamp {}: {} events", timestamp, event_list.len());
        // dbg!(event_list);
    }

    Ok(())
}
