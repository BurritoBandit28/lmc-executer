use std::env;
use crate::little_man::LittleMan;

mod little_man;

fn main() {
    let mut lmc = LittleMan::new();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file specified!");
        return;
    }

    lmc.set_memory(LittleMan::translate(&args[1]));
    lmc.run();
    println!("Program completed!");
    std::thread::sleep(std::time::Duration::from_secs(3));
}
