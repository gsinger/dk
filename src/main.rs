use colored::*;
use std::env;

pub mod container_helper;
pub mod dkutil;
pub mod image_helper;
pub mod ots_helper;
pub mod ports;
pub mod volume_helper;
pub mod system_helper;

// ----------------- Module OtsHelper -----------------

// ----------------- Affichage de l'aide générale -----------------
fn show_usage() {
    println!("{}",  "dk version 4.4 - G. Singer 2018-2025".bright_magenta() );
    
    container_helper::usage();
    println!();
    image_helper::usage();
    println!();
    volume_helper::usage();
    println!();
    system_helper::usage();
    println!();
    ots_helper::usage();
}

// ----------------- Fonction main -----------------
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        show_usage();
        return;
    }
    let command = args[1].as_str();
    let arguments = &args[2..];
    match command {
        "shell" => {
            if arguments.len() != 1 {
                println!("Error: 'shell' command takes one argument");
            } else {
                container_helper::exec_shell(&arguments[0]);
            }
        }
        "ps" => {
            if !arguments.is_empty() {
                println!("Error: 'ps' command does not take any arguments");
            } else {
                container_helper::show();
            }
        }
        "rm" => {
            if arguments.is_empty() {
                println!("Error: 'rm' command requires at least one container");
            } else {
                container_helper::remove(arguments);
            }
        }
        "trunclog" => {
            println!("'trunclog' not yet implemented");
        }
        "sys" => {
            system_helper::cmd(arguments);
        }
        "otsup" => {
            if arguments.is_empty() {
                println!("Error: 'otsup' command requires at least one container");
            } else {
                ots_helper::up(arguments);
                container_helper::show();
            }
        }
        "otsdown" => {
            if arguments.is_empty() {
                println!("Error: 'otsdown' command requires at least one container");
            } else {
                ots_helper::down(arguments);
                container_helper::show();
            }
        }
        "vol" => {
            volume_helper::cmd(arguments);
        }
        "im" => {
            image_helper::cmd(arguments);
        }
        _ => println!("Error: Unknown command '{}'", command),
    }
}
