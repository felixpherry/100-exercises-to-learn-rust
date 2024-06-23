use outro_08::launch;

fn main() {
    if let Err(e) = launch() {
        println!("Error: {:?}", e);
    }
}
