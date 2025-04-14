// main.rs

use colored::*;
use regex::Regex;
use std::env;
use std::path::Path;
use std::process::Command;

// ----------------- Module Ports -----------------
mod ports {
    pub const PORTAINER: u16 = 25002;
    pub const DOKU: u16 = 25004;
    pub const KROKI: u16 = 25100;
    pub const EXCALIDRAW: u16 = 25101;
    pub const CADVISOR: u16 = 25005;
    pub const DOZZLE: u16 = 25006;
    pub const DBEAVER: u16 = 25007;
    pub const DASHY: u16 = 8080;
    pub const GLANCES: u16 = 61208;
    pub const SQLSERVER: u16 = 1433;
}

// ----------------- Fonctions d'affichage -----------------
fn print_info(info: &str) {
    println!("-- {}", info.green());
}

fn print_error(info: &str) {
    println!("-- {}", info.red());
}

/// Exécute une commande système et affiche la commande exécutée.
fn print_and_run(cmd: &[&str]) {
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

// ----------------- Module Utilitaire -----------------
mod util {
    use regex::Regex;
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
}

// ----------------- Module ImageHelper -----------------
mod image_helper {
    use super::{print_and_run, print_error, print_info};
    use std::env;
    use std::path::Path;
    use std::process::Command;

    /// Affiche l'aide pour les commandes liées aux images.
    pub fn usage() {
        use colored::*;
        print_info("IMAGES:");
        println!("{}","  . dk im                   : Show the list of images".yellow());
        println!("{}","  . dk im rm  <image*>      : Remove the specified images".yellow());
        println!("{}"," . dk im save <image*>     : Save the specified images".yellow());
        println!("{}","  . dk im load <file*>      : Load the specified image files".yellow());
        println!("{}","  . dk im scan <file*>      : Scan images for vulnerabilities".yellow());
    }

    /// Tente de récupérer (ou tirer) une image.  
    /// Ici, on recherche d'abord un fichier de sauvegarde, sinon on effectue un docker pull.
    pub fn pull_image(image: &str) {
        let parts: Vec<&str> = image.split(':').collect();
        let (name, tag) = if parts.len() == 1 {
            (parts[0], "latest")
        } else {
            (parts[0], parts[1])
        };

        if is_image_pulled(name, tag) {
            return;
        }

        // Construction du nom de fichier de sauvegarde
        let filename = format!("{}.{}.tar.gz", name, tag).replace("/", "_");
        let root_path = env::current_dir().expect("Impossible d'obtenir le répertoire courant");
        let filepath = root_path.join(&filename);
        if filepath.exists() {
            println!("Image found. ({:?})", filepath);
            // Charge l'image via docker load
            print_and_run(&["docker", "load", "-i", filepath.to_str().unwrap()]);
        } else {
            println!("Image not found. Need to pull it");
            print_and_run(&["docker", "pull", image]);
        }
        // Sauvegarde l'image pour de futurs usages
        __save_one(name, tag, Some(root_path.to_str().unwrap()));
    }

    /// Retourne true si l'image est déjà présente (recherche via `docker images`).
    pub fn is_image_pulled(image_name: &str, tag: &str) -> bool {
        let output = Command::new("docker")
            .arg("images")
            .output()
            .expect("Échec de l'exécution de 'docker images'");
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Une recherche simpliste dans la sortie
        stdout.contains(&format!("{}:{}", image_name, tag))
    }

    /// Récupère la liste des images Docker en format structuré.
    pub fn get_images() -> Vec<Vec<String>> {
        // Utilisation du format personnalisé de 'docker images'
        let output = Command::new("docker")
            .args(&[
                "images",
                "--format",
                "{{.ID}}|{{.Repository}}|{{.Tag}}|{{.Size}}|{{.CreatedAt}}",
            ])
            .output()
            .expect("Échec de 'docker images'");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut table_data = Vec::new();
        let mut index = 1;
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() < 5 {
                continue;
            }
            table_data.push(vec![
                index.to_string(),
                parts[0].to_string(),
                parts[1].to_string(),
                parts[2].to_string(),
                parts[3].to_string(),
                parts[4].to_string(),
            ]);
            index += 1;
        }
        table_data
    }

    /// Affiche la liste des images dans un format tabulaire.
    pub fn show() {
        let images = get_images();
        println!(
            "{:<5} {:<15} {:<20} {:<10} {:<10} {:<20}",
            "Index", "ID", "Name", "Tag", "Size", "Created"
        );
        for row in images {
            println!(
                "{:<5} {:<15} {:<20} {:<10} {:<10} {:<20}",
                row[0], row[1], row[2], row[3], row[4], row[5]
            );
        }
    }

    /// Traite la commande 'im' avec ses arguments.
    pub fn cmd(arguments: &[String]) {
        if arguments.is_empty() {
            show();
            return;
        }
        let command = &arguments[0];
        let args = &arguments[1..];
        match command.as_str() {
            "rm" => {
                if args.is_empty() {
                    println!("Error: 'rm' command requires at least one image");
                    return;
                }
                remove(args);
            }
            "save" => {
                if args.is_empty() {
                    println!("Error: 'save' command requires at least one image");
                    return;
                }
                save(args);
            }
            "load" => {
                if args.is_empty() {
                    println!("Error: 'load' command requires at least one file");
                    return;
                }
                load(args);
            }
            "scan" => {
                if args.is_empty() {
                    println!("Error: 'scan' command requires at least one image");
                    return;
                }
                scan(args);
            }
            _ => {
                print_error("unknown command");
            }
        }
    }

    fn remove(filters: &[String]) {
        for image in filters {
            print_info(&format!("Removing image {}", image));
            print_and_run(&["docker", "rmi", image]);
        }
    }

    fn save(filters: &[String]) {
        for image in filters {
            print_info(&format!("Saving image {}", image));
            // À implémenter : utiliser "docker save" puis compresser en gzip.
            // Pour l'instant, on se contente d'afficher la commande.
            print_and_run(&["docker", "save", image, "-o", &format!("{}.tar", image)]);
        }
    }

    fn load(filters: &[String]) {
        for file in filters {
            print_info(&format!("Loading image file {}", file));
            print_and_run(&["docker", "load", "-i", file]);
        }
    }

    fn scan(filters: &[String]) {
        for image in filters {
            print_info(&format!("Scanning image {}", image));
            print_and_run(&[
                "docker",
                "run",
                "--tty",
                "--rm",
                "-v",
                "trivy:/cachedata",
                "ghcr.io/aquasecurity/trivy:latest",
                "image",
                "--scanners",
                "vuln",
                "--cache-dir",
                "/cachedata",
                image,
            ]);
        }
    }

    /// Sauvegarde une image (fonction interne).
    fn __save_one(name: &str, tag: &str, path: Option<&str>) {
        let full_image_name = format!("{}:{}", name, tag);
        let mut filename = format!("{}.{}.tar.gz", name, tag).replace("/", "_");
        print_info(&format!("saving image {} into {}", name, filename));
        if let Some(p) = path {
            filename = format!("{}/{}", p, filename);
        }
        // À implémenter : appeler "docker save" et compresser
        print_and_run(&["docker", "save", &full_image_name, "-o", &filename]);
    }
}

// ----------------- Module ContainerHelper -----------------
mod container_helper {
    use super::{print_and_run, print_info};
    use std::process::Command;
    use colored::*;

    pub fn usage() {
        print_info("CONTAINERS:");
        println!("{}","  . dk ps                   : Show state of the containers".yellow());
        println!("{}","  . dk rm      <container*>  : Remove container(s)".yellow());
        println!("{}","  . dk shell   <container>   : Run a bash shell into the container".yellow());
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
    use super::{container_helper, image_helper, volume_helper, print_and_run, print_error, print_info};

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
}

// ----------------- Module OtsHelper -----------------
mod ots_helper {
    use super::{container_helper, image_helper, print_and_run, print_error, print_info, ports};
    use std::env;

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
                    print_info(&format!("Starting sqlserver (port={})", ports::SQLSERVER));
                    print_and_run(&[
                        "docker",
                        "run",
                        "-d",
                        "--name",
                        "ots_sqlserver",
                        "-v",
                        "sqlserver_data:/var/opt/mssql",
                        "-p",
                        &format!("{}:1433", ports::SQLSERVER),
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
                    print_info(&format!("Starting dashy (port={})", ports::DASHY));
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
                    print_info(&format!("Starting portainer (port={})", ports::PORTAINER));
                    print_and_run(&[
                        "docker",
                        "run",
                        "-d",
                        "--name",
                        "ots_portainer",
                        "-p",
                        &format!("{}:9000", ports::PORTAINER),
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
                    print_info(&format!("Starting kroki (http://localhost:{})", ports::KROKI));
                    print_and_run(&[
                        "docker",
                        "run",
                        "-d",
                        "--name",
                        "ots_kroki",
                        "--restart",
                        "unless-stopped",
                        "-p",
                        &format!("{}:8000", ports::KROKI),
                        image,
                    ]);
                }
                "excalidraw" => {
                    let image = "excalidraw/excalidraw:latest";
                    image_helper::pull_image(image);
                    print_info(&format!("Starting excalidraw (http://localhost:{})", ports::EXCALIDRAW));
                    print_and_run(&[
                        "docker",
                        "run",
                        "-d",
                        "--name",
                        "ots_excalidraw",
                        "--restart",
                        "unless-stopped",
                        "-p",
                        &format!("{}:80", ports::EXCALIDRAW),
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
                    print_info(&format!("Starting doku (http://localhost:{})", ports::DOKU));
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
                        &format!("{}:9090", ports::DOKU),
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
                    print_info(&format!("Starting cadvisor (http://localhost:{})", ports::CADVISOR));
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
                        &format!("{}:8080", ports::CADVISOR),
                        "-d",
                        "--name",
                        "ots_cadvisor",
                        image,
                    ]);
                }
                "dozzle" => {
                    let image = "amir20/dozzle";
                    image_helper::pull_image(image);
                    print_info(&format!("Starting dozzle (http://localhost:{})", ports::DOZZLE));
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
                        &format!("{}:8080", ports::DOZZLE),
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