
use colored::*;
use crate::container_helper;
use crate::dkutil::*;
use crate::volume_helper;
use crate::image_helper;


pub fn usage() {
    print_info("SYSTEM:");
    println!(
        "{}",
        "  . dk sys show     : Show extended information".yellow()
    );
    println!(
        "{}",
        "  . dk sys prune    : Delete unused data (containers, images, volumes, build cache)".yellow()
    );
    println!(
        "{}",
        "  . dk sys size     : Show data size (docker system df)".yellow()
    );
}

pub fn cmd(arguments: &[String]) {
    if arguments.is_empty() {
        show();
        return;
    }
    let command = &arguments[0];
    match command.as_str() {
        "show" => show(),
        "prune" => prune(),
        "size" => size(),
        _ => print_error("unknown command"),
    }
}

pub fn show() {
    image_helper::show();
    println!();
    volume_helper::show();
    println!();
    container_helper::show();
}

pub fn prune() {
    print_info("Pruning networks");
    print_and_run(&["docker", "network", "prune", "-f"]);
    print_info("Pruning volumes");
    print_and_run(&["docker", "volume", "prune", "-f"]);
    print_and_run(&["docker", "buildx", "prune", "-f"]);
}

pub fn size() {
    print_and_run(&["docker", "system", "df"]);
}
