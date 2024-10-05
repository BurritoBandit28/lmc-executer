use std::collections::HashMap;
use std::fs;

pub struct LittleMan {
    // program counter
    pc : usize,
    // accumulator
    acc : i32,
    memory: Vec<String>,

}

impl LittleMan {
    pub fn new() -> LittleMan {
        Self {
            pc: 0,
            acc: 0,
            memory: vec![],
        }
    }

    pub fn inp(&mut self) {
        println!("Please enter a value");
        let mut value = String::new();
        std::io::stdin().read_line(&mut value).expect("Failed to read line");
        self.acc = value.trim().parse::<i32>().unwrap();
    }

    pub fn sta(&mut self, address : usize) {
        if self.memory.len() < address + 1 {
            for _x in 0..((address + 1) - self.memory.len()) {
                self.memory.push("0".to_string());
            }
        }
        self.memory[address] = self.acc.to_string();
    }

    pub fn add(&mut self, address : usize) {
        self.acc += self.memory[address].as_str().parse::<i32>().unwrap();
    }

    pub fn sub(&mut self, address : usize) {
        self.acc -= self.memory[address].as_str().parse::<i32>().unwrap();
    }

    pub fn lda(&mut self, address : usize) {
        match self.memory.get(address) {
            None => {
                println!("No such address {}", address);
            }
            Some(_) => {
                self.acc = self.memory.get(address).unwrap().parse::<i32>().unwrap();
            }
        }
    }

    pub fn translate(path : &str) -> Vec<String> {
        let mut file = fs::read_to_string(path).expect("File not found");

        file = file.replace("LDA", "5");
        file = file.replace("STA", "3");
        file = file.replace("ADD", "1");
        file = file.replace("SUB", "2");
        file = file.replace("INP", "901");
        file = file.replace("OUT", "902");
        file = file.replace("HLT", "000");
        file = file.replace("BRZ", "7");
        file = file.replace("BRP", "8");
        file = file.replace("BRA", "6");

        let mut buffer_idfk = vec![];
        let mut dat_key : HashMap<String, (String, String)> = HashMap::new();
        let mut branch_key : HashMap<String, String> = HashMap::new();
        let mut line_count = 0;
        for line in file.lines() {

            let mut line_as_vec = line.split_whitespace().collect::<Vec<&str>>();
            let mut is_dat_line = false;
            let mut ret_string = line_as_vec.join("");

            if line.is_empty() {
                continue;
            }
            if line.contains("DAT") {
                dat_key.insert(line_as_vec[0].to_string(), (line_count.to_string(), line_as_vec[2].to_string()));
                is_dat_line = true;
            }
            else if line_as_vec.join("").chars().nth(0).unwrap().is_alphabetic() {
                branch_key.insert(line_as_vec[0].to_string(), line_count.to_string());
                line_as_vec.remove(0);
                ret_string = line_as_vec.join("");
            }
            if !is_dat_line {
                buffer_idfk.push(ret_string)
            }
            line_count += 1;
        }


        file = buffer_idfk.join("\n");

        for (key, (val, _unused)) in dat_key.iter() {
            file = file.replace(key, val);
        }
        for (key, val) in branch_key.iter() {
            file = file.replace(key, val);
        }

        let mut memory : Vec<String> = vec![];
        for lines in file.lines() {
            memory.push(lines.to_string());
        }
        for (_unused, (val, data)) in dat_key.iter() {
            if memory.len() < val.parse::<usize>().unwrap() + 1 {
                for _x in 0..((val.parse::<usize>().unwrap() + 1) - memory.len()) {
                    memory.push("0".to_string());
                }
            }
            memory[val.parse::<usize>().unwrap()] = data.to_string();
        }
        memory
    }

    pub fn run(&mut self) {
        loop {
            let command = self.memory.get(self.pc);
            if command.is_none() {
                return;
            }
            let opcode = command.unwrap().clone().chars().nth(0).unwrap().to_string().parse::<i32>().unwrap();
            let address = command.unwrap().clone()[1..command.unwrap().len()].parse::<i32>().unwrap();

            match opcode {
                1 => {
                    self.add(address as usize);
                }
                2 => {
                    self.sub(address as usize);
                }
                3 => {
                    self.sta(address as usize)
                }
                5 => {
                    self.lda(address as usize);
                }
                6 => {
                    self.pc = address as usize;
                    continue
                }
                7 => {
                    if self.acc == 0 {
                        self.pc = address as usize;
                        continue
                    }
                }
                8 => {
                    if self.acc > -1 {
                        self.pc = address as usize;
                        continue
                    }
                }
                9 => {
                    if address == 1 {
                        self.inp()
                    }
                    else {
                        println!("{}", self.acc)
                    }
                }
                _ => {
                    return;
                }
            }
            self.pc += 1;
        }

    }

    pub fn set_memory(&mut self, memory : Vec<String>) {
        self.memory = memory;
    }

}
