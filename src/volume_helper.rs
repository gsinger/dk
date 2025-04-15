mod volume_helper {
    use super::{print_and_run, print_error, print_info};
    use std::process::Command;
    use colored::*;

    pub fn usage() {
        print_info("VOLUMES:");
        println!("{}","  . dk vol                  : Show the list of volumes".yellow());
        println!("{}","  . dk vol prune            : Delete all unused volumes".yellow());
        println!("{}","  . dk vol rm <volume*>     : Remove specified volumes".yellow());
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
        for f in filters {
            print_info(&format!("Removing volume {}", f));
            print_and_run(&["docker", "volume", "rm", f]);
        }
    }
}
