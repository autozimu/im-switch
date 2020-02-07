use im::{get_input_source, set_input_source};

fn main() -> () {
    if let Some(language) = std::env::args().nth(1) {
        // Set IM.
        set_input_source(&language)
    } else {
        // Get IM.
        println!("Current input source: {}", get_input_source())
    }
}
