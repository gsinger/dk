

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
pub fn print_and_run(cmd: &[&str]) {
    let cmdstr = cmd.join(" ");
    print_info(&cmdstr);
    let status = Command::new(cmd[0])
        .args(&cmd[1..])
        .status()
        .expect("Échec de l'exécution de la commande");
    if !status.success() {
        print_error("La commande s'est terminée avec une erreur");
    }
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

