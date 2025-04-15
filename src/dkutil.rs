use colored::*;
use regex::Regex;
use std::process::Command;

pub fn print_info(info: &str) {
    println!("-- {}", info.green());
}

pub fn print_error(info: &str) {
    println!("-- {}", info.red());
}

/// Execute a system command and dhow the executed command
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
        // Utiliser match au lieu de unwrap pour gérer l'erreur de parsing
        match v.parse::<usize>() {
            Ok(rank) => rank > 0 && rank <= max,
            Err(_) => false // Retourne false si le parsing échoue (par exemple pour les nombres négatifs)
        }
    }  else
    {
        false
    }
    
}

pub fn print_colored(text: &str) {
    // Regex pour capturer les labels valides et le texte qui suit
    let re = Regex::new(r"\((w|y|b)\)([^(]*)").unwrap();
    
    let mut last_end = 0;
    let mut current_color = "w"; // Couleur par défaut: blanc
    
    // Parcourir tous les matches de la regex
    for cap in re.captures_iter(text) {
        let full_match = cap.get(0).unwrap();
        let color = cap.get(1).unwrap().as_str();
        let content = cap.get(2).unwrap().as_str();
        
        // Si il y a du texte entre le dernier match et celui-ci, l'afficher avec la couleur courante
        if full_match.start() > last_end {
            let between_text = &text[last_end..full_match.start()];
            print!("{}", colorize(between_text, current_color));
        }
        
        // Afficher le contenu avec la nouvelle couleur
        print!("{}", colorize(content, color));
        
        // Mettre à jour la couleur courante et la position
        current_color = color;
        last_end = full_match.end();
    }
    
    // Afficher le reste du texte s'il y en a
    if last_end < text.len() {
        print!("{}", colorize(&text[last_end..], current_color));
    }
    
    println!();
}

fn colorize(text: &str, color: &str) -> ColoredString {
    match color {
        "w" => text.white(),
        "y" => text.yellow(),
        "b" => text.bright_blue(),
        _ => text.normal()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_info() {
        // Test visuel : vérifie que la fonction ne panique pas
        print_info("This is a test message");
    }

    #[test]
    fn test_print_error() {
        // Test visuel : vérifie que la fonction ne panique pas
        print_error("This is an error message");
    }

    #[test]
    fn test_is_integer() {
        assert!(is_integer("123"));
        assert!(is_integer("-123"));
        assert!(is_integer("+123"));
        assert!(!is_integer("123.45"));
        assert!(!is_integer("abc"));
        assert!(!is_integer(""));
    }

    #[test]
    fn test_is_valid_rank() {
        assert!(is_valid_rank("1", 5));
        assert!(is_valid_rank("5", 5));
        assert!(!is_valid_rank("0", 5));
        assert!(!is_valid_rank("6", 5));
        assert!(!is_valid_rank("-1", 5));
        // assert!(!is_valid_rank("abc", 5));
    }

    #[test]
    fn test_print_colored() {
        // Test visuel : vérifie que la fonction ne panique pas
        print_colored("(w)White text (y)Yellow text (b)Blue text");
    }

    #[test]
    fn test_print_and_run_success() {
        // Test avec une commande simple qui réussit
        let exit_code = print_and_run(&["echo", "Hello, world!"]);
        assert_eq!(exit_code, 0);
    }

    #[test]
    fn test_print_and_run_failure() {
        // Test avec une commande qui échoue
        let exit_code = print_and_run(&["false"]);
        assert_ne!(exit_code, 0);
    }

    #[test]
    fn test_colorize() {
        // Test des couleurs
        assert_eq!(colorize("test", "w").to_string(), "test".white().to_string());
        assert_eq!(colorize("test", "y").to_string(), "test".yellow().to_string());
        assert_eq!(colorize("test", "b").to_string(), "test".bright_blue().to_string());
        assert_eq!(colorize("test", "x").to_string(), "test".normal().to_string());
    }
}
