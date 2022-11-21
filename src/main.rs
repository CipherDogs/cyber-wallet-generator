use clap::{App, Arg};

mod generator;

use generator::*;

fn main() {
    let matches = App::new("cyber-wallet-generator")
        .version("1.0.0")
        .author("DEADBLACKCLOVER <deadblackclover@protonmail.com>")
        .about("CLI address generator for cyber blockchain")
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .value_name("INT")
                .help("Sets a address generate count"),
        )
        .arg(
            Arg::with_name("chain")
                .long("chain")
                .value_name("NAME")
                .help("Sets the generation prefix"),
        )
        .get_matches();

    let count_arg = matches.value_of("count").unwrap_or("1");
    let count = count_arg.parse().unwrap();

    let chain_arg = matches.value_of("chain").unwrap_or("bostrom");

    for i in 0..count {
        let acc = generate(chain_arg);

        println!("# {}", i + 1);
        println!("address: {}", acc.address);
        println!("mnemonic: {}", acc.mnemonic);
        println!("---------------------------------------------------");
    }
}
