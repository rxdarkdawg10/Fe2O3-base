mod server; 
mod commands;
use std::{ io::Write };
use server::Server;
use commands::{ Commands };

use crate::commands::Command;


fn create_command(cmd_list: &Commands) {
    if cmd_list._c.len() > 1 {
        if cmd_list._v.len() > 0 {
            match cmd_list._c[1] {
                Command::Database => {
                    println!("{:?} -> {:?} -> {}", cmd_list._c[0], cmd_list._c[1], cmd_list._v[0]);
                    return;
                }
                Command::Table => {
                    println!("{:?} -> {:?} -> {}", cmd_list._c[0], cmd_list._c[1], cmd_list._v[0]);
                    return;
                }
                _ => {}
            }
        }
    }

    eprintln!("Not enough Parameters Given");
}

fn clear_screen(server: &Server) {
    print!("\x1b[2J\x1b[1;1H"); //x1b[2J clear screen // \x1b[1;1h position row1:col1
    println!("Welcome to Lock Database Server v:{v}", v = "0.0.1a");
    println!("Server Running on Port: {}", server._port);
}

fn show_help(_cmds: &Commands) {
    println!("Help: Commands");
    println!("      clear       Clears the screen of all output.");
    println!("      create      Create a database object.");
    println!("      exit        Closes the Server");
    println!("      help        Shows this message");
}


fn main() {
    let server = Server::new(1433);
    clear_screen(&server);    
    
    let mut exit = false;
    let mut line = String::new();
    
    while !exit {
        let mut lock = std::io::stdout().lock();
        writeln!(lock, "Enter Command: ").unwrap();
        std::io::stdout().flush().unwrap();
        match std::io::stdin().read_line(&mut line) {
            Ok(n) => {
                if n > 0 {
                    let replaced = line.replace("\n", "");
                    line.clear();
                    let mut cmds = Commands::new();
                    for _ in 0..replaced.split(" ").count() {
                        let val: Commands = replaced.split(" ").collect(); 
                        cmds = val;
                    }
                    for c in  &cmds._c {
                        match c {
                            Command::Exit => {
                                exit = true;
                            }
                            Command::Create => { 
                                create_command(&cmds);
                            }
                            Command::Clear => {
                                clear_screen(&server);
                            }
                            Command::Help => {
                                show_help(&cmds);
                            }
                            _ => { },
                        }
                    }
                }
            },
            Err(_) => todo!(),
        }


        //println!("Should Exit: {}", exit);
    }
}




