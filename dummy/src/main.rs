// use clap::{Parser, Subcommand};

// #[derive(Parser)]
// #[command(name = "Bitcoin CLI")]
// #[command(version = "1.0")]
// #[command(about = "Bitcoin Core RPC Client", long_about = None)]
// struct Cli {
//     #[command(subcommand)]
//     command: Option<Commands>
// }

// #[derive(Subcommand)]
// enum Commands {
//     /// Returns hash of block in best-block-chain at height provided.
//     Getblockhash { 
//         #[arg(
//             required = true,
//             help = "(numeric, required) The height index",
//         )]
//         height: u64 
//     },

//     Getbitcoin {
//         #[arg(
//             required = true,
//             help = "(numeric, required) The anount returned in satoshis",
//         )]
//         amount: u64
//     }
// }

// fn main() {
//     let cli = Cli::parse();

//     match &cli.command {
//         Some(Commands::Getblockhash { height }) => {
//             println!("returns blockhash for height: {height:?}")
//         }
//         Some(Commands::Getbitcoin { amount }) => {
//             println!("returns bitcoin for amount: {amount:?}")
//         }
//         None => {
//             eprintln!("Error: too few parameters")
//         }
//     }
// }

// use clap::Parser;


// /// Simple greeting program
// #[derive(Parser, Debug)]
// #[command(name = "greet", version, about = "Says hello")]
// struct Args {
//     /// Name of the person to greet (positional argument)
//    /// #[arg(default_value_t = String::from("hariom"))]
//     name: String,

//     num : u32,

//     /// Enable verbose mode (flag, --verbose or -v)
//     #[arg(short, long)]
//     verbose: bool,
// }

// fn main() {
//     let args = Args::parse();

//     println!("User no : , {}!", args.num);
//     println!("Hello, {}!", args.name);

//     if args.verbose {
//         println!("Verbose mode is ON");
//     }
// }


use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli{
    #[command(subcommand)]
    commands : Option<Commands>
}

#[derive(Subcommand, Debug)]
enum Commands{
    add{
        a: u32,
        b: u32,
    },
    mul{
        a: u32,
        b: u32,
    }
}

fn main(){

    let cli = Cli::parse();

    match &cli.commands {
         Some(Commands::add{a, b})=>{
            println!("Addition = {}", a + b);
         }
         Some(Commands::mul{a, b})=>{
            println!("Mltiplication = {}",a*b);
         }
         None => {
            println!("No command\n");
         }
    }
}


// use clap::{Parser, Subcommand};

// #[derive(Parser, Debug)]
// struct Cli {
//     #[command(subcommand)]
//     commands: Option<Commands>,
// }

// #[derive(Subcommand, Debug)]
// enum Commands {
//     Add {
//         a: u32,
//         b: u32,
//     },
//     Mul {
//         a: u32,
//         b: u32,
//     },
// }

// fn main() {
//     let cli = Cli::parse();

//     match &cli.commands {
//         Some(Commands::Add { a, b }) => {
//             println!("Addition = {}", a + b);
//         }
//         Some(Commands::Mul { a, b }) => {
//             println!("Multiplication = {}", a * b);
//         }
//         None => {
//             println!("No command\n");
//         }
//     }
// }
