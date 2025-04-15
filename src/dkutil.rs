

use colored::*;
use regex::Regex;
use std::process::Command;

pub fn print_info(info: &str) {
    println!("-- {}", info.green());
}

pub fn print_error(info: &str) {
    println!("-- {}", info.red());
}

/// Exécute une commande système et affiche la commande exécutée.
pub fn print_and_run(cmd: &[&str]) -> i32{
    let cmdstr = cmd.join(" ");
    print_info(&cmdstr);
    let status = Command::new(cmd[0])
        .args(&cmd[1..])
        .status()
        .expect("Failed to execute the command");
    
    let code = status.code().unwrap_or(0); 
    
    if !status.success() {
        let msg = format!("Exit code : {}",code);
        print_error(&msg);
    }
    return code;

}

pub fn is_integer(s: &str) -> bool {
    let re = Regex::new(r"^[+-]?\d+$").unwrap();
    re.is_match(s)
}

pub fn is_valid_rank(v: &str, max: usize) -> bool {
    if is_integer(v) {
        let rank: usize = v.parse().unwrap();
        rank > 0 && rank <= max
    } else {
        false
    }
}

