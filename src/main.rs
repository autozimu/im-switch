use failure::Fallible;
use im::{get_input_source, set_input_source};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Arguments {
    /// Set up the input method to use inputmethodname
    #[structopt(short = "s")]
    inputmethodname: Option<String>,
}

fn main() -> Fallible<()> {
    let args = Arguments::from_args();

    if let Some(inputmethodname) = args.inputmethodname {
        // Set IM.
        set_input_source(&inputmethodname)
    } else {
        // Get IM.
        println!("Current input source: {}", get_input_source());
        Ok(())
    }
}
