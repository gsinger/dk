
use colored::*;
use prettytable::{Attr, Cell, Row, Table, color, format};
use std::env;
use std::process::Command;
use crate::dkutil::*;



/// Affiche l'aide pour les commandes liées aux images.
pub fn usage() {
    print_info("IMAGES:");
    println!(
        "{}",
        "  . dk im                   : Show the list of images".yellow()
    );
    println!(
        "{}",
        "  . dk im rm  <image*>      : Remove the specified images".yellow()
    );
    println!(
        "{}",
        " . dk im save <image*>     : Save the specified images".yellow()
    );
    println!(
        "{}",
        "  . dk im load <file*>      : Load the specified image files".yellow()
    );
    println!(
        "{}",
        "  . dk im scan <file*>      : Scan images for vulnerabilities".yellow()
    );
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
    let filename = format!("{name}.{tag}.tar.gz").replace("/", "_");
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
        Cell::new("Tag")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("Size")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("Created")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
    ]));

    for r in images {
        table.add_row(Row::new(vec![
            Cell::new(&r[0]),
            Cell::new(&r[1]),
            Cell::new(&r[2]).with_style(Attr::Bold),
            Cell::new(&r[3]),
            Cell::new(&r[4]),
            Cell::new(&r[5]),
        ]));
    }

    table.printstd();
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
