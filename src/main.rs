use std::{io::{self, Write}, process, str::FromStr};

#[macro_use]
mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    get_input("input a miner address: ", &mut miner_addr);
    get_input("difficulty: ", &mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("");
    println!("generationg genesis block!");
    let mut chain = blockchain::Chain::new(miner_addr, diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Verify");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("5) Show all chain");
        println!("0) Exit");
        println!("Enter your choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting!");
                process::exit(0);
            },
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                get_input("enter sender address: ", &mut sender);
                get_input("enter receiver address: ", &mut receiver);
                get_input("enter amount: ", &mut amount);

                let res = chain.new_transaction(sender.trim().to_string(),
                                                receiver.trim().to_string(),
                                                amount.trim().parse().unwrap());

                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                }
            },
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            },
            3 => {
                let mut new_diff = String::new();
                get_input("enter new difficulty: ", &mut new_diff);
                match chain.update_difficulty(new_diff.trim().parse().unwrap()) {
                    true => println!("Difficulty updated successfully"),
                    false => println!("Difficulty updating failed"),
                }
            },
            4 => {
                let mut new_reward = String::new();
                get_input("enter new reward: ", &mut new_reward);
                match chain.update_reward() {
                    true => println!("Reward updated successfully"),
                    false => println!("Reward updating failed"),
                }
            },
            5 => {
                println!("{:#?}", chain);
            },
            _ => {
                println!("Command not found!");
            },
        }
    }
}

fn get_input<S: Into<String>, T: FromStr>(s: S, val: &mut T)
    where <T as FromStr>::Err: std::fmt::Debug
{
    let str: String = s.into();
    println!("{}", str);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
//            let input = &input[str.len()..];
            *val = input.trim().parse::<T>().unwrap();
        },
        Err(e) => {
            eprintln!("Input has inciorrect format! Error: {}", e);
        },
    }
}