use crate::little_man::LittleMan;

mod little_man;

fn main() {
    let mut lmc = LittleMan::new();
    println!("Please enter a Little Man computer program path");
    let mut value = String::new();
    std::io::stdin().read_line(&mut value).expect("Failed to read line");
    lmc.set_memory(LittleMan::translate(value.trim()));
    lmc.run();
    println!("Program completed!");
    std::thread::sleep(std::time::Duration::from_secs(3));
}
