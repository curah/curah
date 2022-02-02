mod commands;

fn main() {
    if let Err(e) = commands::run() {
        eprintln!("Error: {}", e);
    }
}
