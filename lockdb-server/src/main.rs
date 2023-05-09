mod server; 
mod commands;
use std::{ io::Write };
use lock_db::{Database, Table, Columns, Column};
use server::Server;
use commands::{ Commands };

use crate::commands::Command;

fn use_database(cmd_list: Commands, server: &mut Server) -> Result<(), ()> {
    if cmd_list._v.len() > 0 {
        let dbs = &server.databases;
        let mut use_index = 0;
        for ind in 0..dbs.len() {
            if dbs[ind].dbname == cmd_list._v[0] {
                use_index = ind;
            }
        }
        for ind in 0..server.databases.len() {
            if use_index != ind {
                server.databases[ind].in_use = false;
            }
        }
        server.databases[use_index].in_use = true;

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
                    server.databases.push(db);
                    
                    let dbs = &server.databases;
                    let mut use_index = 0;
                    for ind in 0..dbs.len() {
                        
                        if dbs[ind].dbname == db_name {
                            use_index = ind;
                        }
                    }
                    for ind in 0..server.databases.len() {
                        if use_index != ind {
                            server.databases[ind].in_use = false;
                        }
                    }
                    server.databases[use_index].in_use = true;

                    return Ok(());
                }
                Command::Table => {
                    let tablename: String = cmd_list._v[0].clone();
                    let table = Table::new(tablename);
                    server.databases[0].tables.push(table);
                    
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

fn run_show_command(cmd_list: Commands, server: &Server) -> Result<(),()> {
    
    match cmd_list._c[1] {
        Command::Database => {
            clear_screen(server);
            println!("Databases:");
            println!("Database Name     Table Name");
            if server.databases.len() > 0 {
                for d in &server.databases {
                    if d.tables.len() > 0 {
                        for t in &d.tables {
                            println!("{}     {}", d.dbname, t.tablename);
                        }
                    } else {
                        println!("{}     <none>", d.dbname);
                    }
                }
            } else {
                println!("<none>     <none>");
            }
            return Ok(())
        }
        Command::Table => {}
        _ => {}
    }
    
    Err(())
}

fn run_table_command(cmd_list: Commands, server: &mut Server) -> Result<(),()> {
    
    if cmd_list._c.len() > 2 {
        if cmd_list._v.len() > 3 {
            match cmd_list._c[1] {
                Command::Add => {
                    let tblname = cmd_list._v[0].clone();
                    match cmd_list._c[2] {
                        Command::Column => {
                            let colname = cmd_list._v[1].clone();
                            let val: Column = cmd_list._v[2].clone().try_into().map_err(|e| eprintln!("Couldn't Convert: {}",e))?;
                            //let _ty: Column = cmd_list._v[1].clone().try_into();
                            println!("{:?}", val);
                            let sze = cmd_list._v[3].clone().parse::<u64>().unwrap();
                            for n in 0..server.databases.len() {
                                if server.databases[n].in_use {
                                    for t in 0..server.databases[n].tables.len() {
                                        if server.databases[n].tables[t].tablename == tblname {
                                            server.databases[n].tables[t].columns.push(Columns::new(colname.clone(), val, sze));
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }

                    return Ok(());
                }
                _ => {}
            }
        }
    }

    eprintln!("Not enough Parameters Given");
    eprintln!("Command Help:\n table <tablename> add column <column_name> <column_type> <column_size>");
    Err(())
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

                    match &cmds._c[0] {
                        Command::Exit => {
                            exit = true;
                        }
                        Command::Create => { 
                            let _ = create_command(cmds.clone(), &mut server).map_err(|_| eprintln!("Couldnt Create Object"));
                        }
                        Command::Use => {
                            let _ = use_database(cmds.clone(), &mut server).map_err(|_| eprintln!("Couldnt Use Database.  Database doesn't Exist"));
                        }
                        Command::Table => {
                            let _ = run_table_command(cmds.clone(), &mut server).map_err(|_| eprintln!("Could not run commands against table."));
                        }
                        Command::Clear => {
                            clear_screen(&server);
                        }
                        Command::Help => {
                            show_help(&cmds);
                        }
                        Command::Show => {
                            let _ = run_show_command(cmds.clone(), &server);
                        }
                        _ => { },
                    }

                }
            },
            Err(_) => todo!(),
        }

        //println!("Should Exit: {}", exit);
    }
}









