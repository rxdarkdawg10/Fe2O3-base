mod server; 
mod commands;
use std::{ io::Write, str::Split };
use lock_db::{Database, Table};
use server::Server;
use commands::{ Commands };

use crate::commands::Command;

fn use_database(cmd_list: Commands, server: &mut Server) -> Result<(), ()> {
    if cmd_list._v.len() > 0 {
        let dbs = &server.Databases;
        let mut use_index = 0;
        for ind in 0..dbs.len() {
            if dbs[ind].dbname == cmd_list._v[0] {
                use_index = ind;
            }
        }
        for ind in 0..server.Databases.len() {
            if use_index != ind {
                server.Databases[ind].in_use = false;
            }
        }
        server.Databases[use_index].in_use = true;

        return Ok(())
    }
    return Err(())
}

fn create_command<'a>(cmd_list: Commands, server: &mut Server) -> Result<(), ()> {
    if cmd_list._c.len() > 1 {
        if cmd_list._v.len() > 0 {
            match cmd_list._c[1] {
                Command::Database => {
                    let dbname = cmd_list._v[0].clone();
                    let db_name = dbname.clone();
                    let db = Database::new(dbname);
                    server.Databases.push(db);
                    
                    let dbs = &server.Databases;
                    let mut use_index = 0;
                    for ind in 0..dbs.len() {
                        
                        if dbs[ind].dbname == db_name {
                            use_index = ind;
                        }
                    }
                    for ind in 0..server.Databases.len() {
                        if use_index != ind {
                            server.Databases[ind].in_use = false;
                        }
                    }
                    server.Databases[use_index].in_use = true;

                    return Ok(());
                }
                Command::Table => {
                    let tablename: String = cmd_list._v[0].clone();
                    let table = Table::new(tablename);
                    server.Databases[0].tables.push(table);
                    
                    return Ok(());
                }
                _ => {}
            }
        }
    }

    eprintln!("Not enough Parameters Given");
    Err(())
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
    println!("      use         Use Database Object");
    println!("      help        Shows this message");
}


fn main() {
    let mut server = Server::new(1433);
    clear_screen(&server);    
    
    let mut exit = false;
    
    while !exit {
        let mut line = String::new();
        let mut cmds = Commands::new();
        let mut lock = std::io::stdout().lock();
        writeln!(lock, "Enter Command: ").unwrap();
        std::io::stdout().flush().unwrap();
        match std::io::stdin().read_line(&mut line) {
            Ok(n) => {
                if n > 0 {
                    let tmp = line.replace("\n", "");
                    let tmp2 = tmp.to_owned();
                    let tmp3 = tmp2.split(" ").to_owned();
                    let tmp4 = tmp3.collect::<Vec<_>>();

                    for t in tmp4 {
                        let t1 = t.to_owned();
                        match Command::new(t1) {
                            Ok(c) => {
                                cmds._c.push(c);
                            }
                            Err(c) => {
                                cmds._v.push(c);
                            }
                        }                        
                    }

                    // let replaced: Commands = line.split(" ").collect();
                    //line.clear();
                    
                    // for _ in 0..replaced.split(" ").count() {
                        // let val: Commands = replaced.split(" ").collect(); 
                    // cmds = replaced;
                    // }
                    for c in  &cmds._c {
                        match c {
                            Command::Exit => {
                                exit = true;
                            }
                            Command::Create => { 
                                create_command(cmds.clone(), &mut server).expect("Couldnt Create Object");
                            }
                            Command::Use => {
                                use_database(cmds.clone(), &mut server).expect("Couldnt Use Database.  Database doesn't Exist");
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


        println!("{:?}", server);

        //println!("Should Exit: {}", exit);
    }
}






