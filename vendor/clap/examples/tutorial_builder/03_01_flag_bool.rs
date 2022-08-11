use clap::{command, Arg, ArgAction};

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    println!(
        "verbose: {:?}",
        *matches
            .get_one::<bool>("verbose")
            .expect("defaulted by clap")
    );
}
