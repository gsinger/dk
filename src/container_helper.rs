mod container_helper {
    use super::{print_and_run, print_info};
    use std::process::Command;
    use colored::*;

    pub fn usage() {
        println!("{}","CONTAINERS:".cyan());
        println!("{}                {}","  . dk ps".yellow(),": Show state of the containers".white());
        println!("{} {}  {}","  . dk rm ".yellow(), "<container*>".bright_blue(), ": Remove container(s)".white());
        println!("{} {} {}","  . dk shell".yellow(),"<container>".bright_blue(),": Run a bash shell into the container".white());
    }

    pub fn get_containers() -> Vec<Vec<String>> {
        let output = Command::new("docker")
            .args(&["ps", "-a", "--format", "{{.Names}}|{{.ID}}|{{.Image}}|{{.Status}}"])
            .output()
            .expect("Échec de 'docker ps'");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut table = Vec::new();
        let mut index = 1;
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() < 4 {
                continue;
            }
            table.push(vec![
                index.to_string(),
                parts[0].to_string(),
                parts[1].to_string(),
                parts[2].to_string(),
                parts[3].to_string(),
            ]);
            index += 1;
        }
        table
    }

    pub fn show() {
        let containers = get_containers();
        println!(
            "{:<5} {:<20} {:<15} {:<20} {:<30}",
            "Index", "Name", "ID", "Image", "Status"
        );
        for row in containers {
            println!(
                "{:<5} {:<20} {:<15} {:<20} {:<30}",
                row[0], row[1], row[2], row[3], row[4]
            );
        }
    }

    pub fn remove(filters: &[String]) {
        for f in filters {
            print_info(&format!("Removing container {}", f));
            print_and_run(&["docker", "rm", "-f", f]);
        }
    }

    pub fn exec_shell(container: &str) {
        print_info(&format!("Executing shell in container {}", container));
        print_and_run(&["docker", "exec", "-it", container, "/bin/bash"]);
    }
}