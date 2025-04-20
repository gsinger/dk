
use colored::*;
use crate::container_helper;
use crate::dkutil::*;
use crate::volume_helper;
use crate::image_helper;


pub fn usage() {
    println!("{}", "SYSTEM:".cyan());
    print_colored("(y) . dk sys show         (w): Show extended information");
    print_colored("(y) . dk sys prune        (w): Delete unused data (containers, images, volumes, build cache)");
    print_colored("(y) . dk sys size         (w): Show data size (docker system df)");
   
}

pub fn cmd(arguments: &[String]) ->i32 {
    if arguments.is_empty() {
        show();
        return 0;
    }
    let command = &arguments[0];
    match command.as_str() {
        "show" => show(),
        "prune" => prune(),
        "size" => size(),
        _ => print_error("unknown command"),
    }
    return 0;

}

pub fn show() {
    println!("{}", "VOLUMES:".cyan());
    volume_helper::show();
    println!();
    println!("{}", "IMAGES:".cyan());
    image_helper::show();
    println!();
    println!("{}", "CONTAINERS:".cyan());    
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
