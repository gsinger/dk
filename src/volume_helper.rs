use colored::*;
use std::process::Command;
use crate::dkutil::*;


pub fn usage() {
    
    println!("{}", "VOLUMES:".cyan());
    print_colored("(y) . dk vol              (w): Show state of the containers");
    print_colored("(y) . dk vol prune        (w): Delete all unused volumes");
    print_colored("(y) . dk vol rm (b)<volume*> (w): Delete specified volumes");
}

pub fn get_volumes() -> Vec<Vec<String>> {
    let output = Command::new("docker")
        .args(&["volume", "ls", "--format", "{{.Name}}"])
        .output()
        .expect("Échec de 'docker volume ls'");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut table = Vec::new();
    let mut index = 1;
    for line in stdout.lines() {
        table.push(vec![index.to_string(), line.to_string()]);
        index += 1;
    }
    table
}

pub fn show() {
    let volumes = get_volumes();
    println!("{:<5} {:<20}", "Index", "Volume Name");
    for row in volumes {
        println!("{:<5} {:<20}", row[0], row[1]);
    }
}

pub fn cmd(arguments: &[String]) {
    if arguments.is_empty() {
        show();
        return;
    }
    let command = &arguments[0];
    match command.as_str() {
        "rm" => {
            if arguments.len() < 2 {
                println!("Error: 'rm' command requires at least one volume");
                return;
            }
            rm(&arguments[1..]);
        }
        _ => print_error("unknown command"),
    }
}

fn rm(filters: &[String]) {
    let volumes = translate_to_id(filters);
    for v in volumes {
        print_info(&format!("Removing volume {}", v));
        print_and_run(&["docker", "volume", "rm", &v]);
    }
}


fn translate_to_id(filters: &[String]) -> Vec<String> {
    let volumes = get_volumes();
    let max_rank = volumes.iter().count();
    
    filters.iter().map(|f| {
        if is_valid_rank(f, max_rank) {
            let row = &volumes[f.parse::<usize>().unwrap() - 1];
            row[1].clone()
        } else {
            f.to_string()
        }
    }).collect()
}