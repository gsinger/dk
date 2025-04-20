
use prettytable::{Attr, Cell, Row, Table, color, format};
use colored::*;
use std::process::Command;
use crate::dkutil::*;

pub fn usage() {
    println!("{}", "CONTAINERS:".cyan());

    print_colored("(y) . dk ps              (w): Show state of the containers");
    print_colored("(y) . dk rm (b)<container*> (w): Remove container(s)");
    print_colored("(y) . dk shell           (w): Run a bash shell into the container");
}

pub fn get_containers() -> Vec<Vec<String>> {
    let output = Command::new("docker")
        .args(&[
            "ps",
            "-a",
            "--format",
            "{{.Names}}|{{.ID}}|{{.Image}}|{{.Status}}",
        ])
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

pub fn show()  {
    let containers = get_containers();

    let mut table = Table::new();

    let format = format::FormatBuilder::new()
        .column_separator(' ') // séparateur de colonne
        .borders(' ') // bordures
        .padding(1, 1)
        .build();
    table.set_format(format);

    table.set_titles(Row::new(vec![
        Cell::new("Index")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("ID")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("Name")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("Image")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("Status")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN))
    ]));

    for r in containers {
        table.add_row(Row::new(vec![
            Cell::new(&r[0]),
            Cell::new(&r[2]),
            Cell::new(&r[1]).with_style(Attr::Bold),
            Cell::new(&r[3]),
            Cell::new(&r[4])
        ]));
    }

    table.printstd();
}

pub fn remove(filters: &[String]) -> i32 {
    let mut retcode=0;
    let ids = translate_to_id(filters);


    for f  in ids {
        print_info(&format!("Removing container {}", f));
        let i= print_and_run(&["docker", "rm", "-f", &f]);
    
        if i != 0 {
            retcode = i;
            print_error(&format!("Error removing container {}", f));    
        }
    }
    retcode
}


pub fn exec_shell(container: &str) -> i32 {
    print_info(&format!("Executing shell in container {}", container));
    return print_and_run(&["docker", "exec", "-it", container, "/bin/bash"]);
}


fn translate_to_id(filters: &[String]) -> Vec<String> {
    let containers = get_containers();
    let max_rank = containers.iter().count();
    
    filters.iter().map(|f| {
        if is_valid_rank(f, max_rank) {
            let row = &containers[f.parse::<usize>().unwrap() - 1];
            row[1].clone()
        } else {
            f.to_string()
        }
    }).collect()
}