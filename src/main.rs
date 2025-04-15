// main.rs

use colored::*;
use regex::Regex;
use std::env;
use std::path::Path;
use std::process::Command;

mod ports;
mod image_helper;
mod dkutil;

// ----------------- Module ImageHelper -----------------


// ----------------- Module ContainerHelper -----------------


// ----------------- Module VolumeHelper -----------------
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

// ----------------- Module SystemHelper -----------------
mod system_helper {
    use super::{container_helper, volume_helper, print_and_run, print_error, print_info};
    

    use colored::*;
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
        crate::image_helper::show();
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
}

// ----------------- Module OtsHelper -----------------
mod ots_helper {
    use super::{print_and_run, print_error, print_info, ports};
    use std::env;
    use colored::*;
    use crate::ports::ports::*;

    pub fn usage() {
        print_info("OTS:");
        println!(
            "{}",
            "  . otsup <images>  : Create and run containers based on the specified images".yellow()
        );
        println!(
            "{}",
            "         supported images: plantuml|portainer|excalidraw|rabbitmq|doku|dashy|glances|dozzle|kroki|postgres|sqlserver|registry|ctop|cadvisor"
                .yellow()
        );
    }

    pub fn down(images: &[String]) {
        for im in images {
            let container_name = format!("ots_{}", im);
            print_info(&format!("Removing container {}", container_name));
            print_and_run(&["docker", "rm", "-f", &container_name]);
        }
    }

    pub fn up(images: &[String]) {
        for im in images {
            match im.as_str() {
                "sqlserver" => {
                    let image = "mcr.microsoft.com/mssql/server:2022-latest";
                    image_helper::pull_image(image);
                    print_info(&format!("Starting sqlserver (port={})", SQLSERVER));
                    print_and_run(&[
                        "docker",
                        "run",
                        "-d",
                        "--name",
                        "ots_sqlserver",
                        "-v",
                        "sqlserver_data:/var/opt/mssql",
                        "-p",
                        &format!("{}:1433", SQLSERVER),
                        "-e",
                        "ACCEPT_EULA=Y",
                        "-e",
                        "SA_PASSWORD=Sh@dokN0tD€ad!",
                        "--restart",
                        "unless-stopped",
                        image,
                    ]);
                }
                "dashy" => {
                    let image = "lissy93/dashy:latest";
                    image_helper::pull_image(image);
                    print_info(&format!("Starting dashy (port={})", DASHY));
                    let root_path = env::current_dir().unwrap();
                    let vol = format!("{}/dk-compose/dashy/config.yml", root_path.to_str().unwrap());
                    print_and_run(&[
                        "docker",
                        "run",
                        "-d",
                        "-v",
                        &format!("{}:/app/user-data/conf.yml", vol),
                        "--name",
                        "ots_dashy",
                        "--restart",
                        "always",
                        "--network",
                        "host",
                        "--restart",
                        "unless-stopped",
                        image,
                    ]);
                }
                "portainer" => {
                    let image = "portainer/portainer-ce:latest";
                    image_helper::pull_image(image);
                    print_info(&format!("Starting portainer (port={})", PORTAINER));
                    print_and_run(&[
                        "docker",
                        "run",
                        "-d",
                        "--name",
                        "ots_portainer",
                        "-p",
                        &format!("{}:9000", PORTAINER),
                        "-p",
                        "25003:9443",
                        "-v",
                        "/var/run/docker.sock:/var/run/docker.sock",
                        "-v",
                        "portainer_data:/data",
                        "--restart",
                        "unless-stopped",
                        image,
                    ]);
                }
                "kroki" => {
                    let image = "yuzutech/kroki";
                    image_helper::pull_image(image);
                    print_info(&format!("Starting kroki (http://localhost:{})", KROKI));
                    print_and_run(&[
                        "docker",
                        "run",
                        "-d",
                        "--name",
                        "ots_kroki",
                        "--restart",
                        "unless-stopped",
                        "-p",
                        &format!("{}:8000", KROKI),
                        image,
                    ]);
                }
                "excalidraw" => {
                    let image = "excalidraw/excalidraw:latest";
                    image_helper::pull_image(image);
                    print_info(&format!("Starting excalidraw (http://localhost:{})", EXCALIDRAW));
                    print_and_run(&[
                        "docker",
                        "run",
                        "-d",
                        "--name",
                        "ots_excalidraw",
                        "--restart",
                        "unless-stopped",
                        "-p",
                        &format!("{}:80", EXCALIDRAW),
                        image,
                    ]);
                }
                "glances" => {
                    let image = "nicolargo/glances:latest";
                    image_helper::pull_image(image);
                    print_info("Starting glances (http://localhost:61208)");
                    print_and_run(&[
                        "docker",
                        "run",
                        "-d",
                        "--name",
                        "glances",
                        "--restart",
                        "unless-stopped",
                        "-v",
                        "/var/run/docker.sock:/var/run/docker.sock:ro",
                        "-p",
                        "61208:61208",
                        "--pid",
                        "host",
                        "--privileged",
                        "-e",
                        "GLANCES_OPT=-w",
                        "-e",
                        "PUID=1000",
                        "-e",
                        "PGID=1000",
                        "-e",
                        "TZ=Europe/Paris",
                        image,
                    ]);
                }
                "rabbitmq" => {
                    let image = "rabbitmq:3.12-management-alpine";
                    image_helper::pull_image(image);
                    print_info("Starting rabbitmq (http://localhost:15672)");
                    print_and_run(&[
                        "docker",
                        "run",
                        "-d",
                        "--name",
                        "ots_rabbitmq",
                        "--restart",
                        "unless-stopped",
                        "-p",
                        "15672:15672",
                        "-p",
                        "5672:5672",
                        "--mount",
                        "type=volume,src=ots_rabbitmq,dst=/var/lib/rabbitmq",
                        "--restart",
                        "always",
                        image,
                    ]);
                }
                "doku" => {
                    let image = "amerkurev/doku";
                    image_helper::pull_image(image);
                    print_info(&format!("Starting doku (http://localhost:{})", DOKU));
                    print_and_run(&[
                        "docker",
                        "run",
                        "--name",
                        "ots_doku",
                        "--restart",
                        "unless-stopped",
                        "-it",
                        "-d",
                        "-v",
                        "/var/run/docker.sock:/var/run/docker.sock:ro",
                        "-v",
                        "/:/hostroot:ro",
                        "-p",
                        &format!("{}:9090", DOKU),
                        image,
                    ]);
                }
                "ctop" => {
                    let image = "quay.io/vektorlab/ctop:latest";
                    image_helper::pull_image(image);
                    print_and_run(&[
                        "docker",
                        "run",
                        "--name",
                        "ots_ctop",
                        "-it",
                        "--rm",
                        "--volume",
                        "/var/run/docker.sock:/var/run/docker.sock",
                        image,
                    ]);
                }
                "cadvisor" => {
                    let image = "gcr.io/cadvisor/cadvisor";
                    image_helper::pull_image(image);
                    print_info(&format!("Starting cadvisor (http://localhost:{})", CADVISOR));
                    print_and_run(&[
                        "docker",
                        "run",
                        "-v",
                        "/:/rootfs:ro",
                        "-v",
                        "/var/run:/var/run:rw",
                        "-v",
                        "/sys:/sys:ro",
                        "-v",
                        "/var/lib/docker/:/var/lib/docker:ro",
                        "--restart",
                        "unless-stopped",
                        "-p",
                        &format!("{}:8080", CADVISOR),
                        "-d",
                        "--name",
                        "ots_cadvisor",
                        image,
                    ]);
                }
                "dozzle" => {
                    let image = "amir20/dozzle";
                    image_helper::pull_image(image);
                    print_info(&format!("Starting dozzle (http://localhost:{})", DOZZLE));
                    print_and_run(&[
                        "docker",
                        "run",
                        "--detach",
                        "-v",
                        "/var/run/docker.sock:/var/run/docker.sock",
                        "-e",
                        "DOZZLE_LEVEL=Debug",
                        "--restart",
                        "unless-stopped",
                        "-p",
                        &format!("{}:8080", DOZZLE),
                        image,
                    ]);
                }
                _ => {
                    print_error(&format!("unknown image to run : {}", im));
                }
            }
        }
    }
}

// ----------------- Affichage de l'aide générale -----------------
fn show_usage() {
    println!("{}", "dk version 4.4 - G. Singer 2018-2025".bright_magenta());
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