use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    generate: bool,

    /// Number of times to greet
    #[arg(short, long, default_value_t = ("testfile.bin".to_string()))]     //
    name: String,
}

fn generate_file(file_name: String) {
    // Generate a test file
}

fn main() {
    let cli_args = Args::parse();
    if cli_args.generate {
        println!("Generating test file {}", cli_args.name);
        return;

    } else {
        println!("Generate is False");
    }
    println!("Name = {}", cli_args.name);
}
