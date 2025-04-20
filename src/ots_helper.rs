
use crate::dkutil::*;
use colored::*;
use crate::config::*;

pub fn usage() {
    let _config=get_config();
    println!("{}", "OTS:".cyan());
    print_colored("(y) . ots up (b)<image>       (w): Create and run a container based on the specified image");    
    print_colored("(y) . ots down (b)<image>     (w): Delete the specified container");

    let mut ots_list = String::new();
    for c in _config.ots {
        ots_list.push_str(&c.name);
        ots_list.push_str("|");
    }
    ots_list.pop();
    let ots=format!("(y) The supported ots are  (w): (b){}",ots_list);

    print_colored(&ots);
    print_colored("                         (y) (see ~/.dk.dk_config.json)");
}

pub fn cmd(arguments: &[String]) -> i32 {
    if arguments.is_empty() {
        usage();
        return 0;
    }
    let command = &arguments[0];
    match command.as_str() {
        "up" => up_container(&arguments[1..]),
        "down" => down_container(&arguments[1..]),
        _ => print_error("unknown command"),
    }
    return 0;
}

fn down_container(arguments: &[String])
{
    if arguments.is_empty() {
        print_error("Error: 'up' command requires at least one container");
        return;
    }
    let config=get_config();
    for arg in arguments {
        let mut found = false;
        for c in &config.ots {
            if c.name == *arg {
                found = true;
                let name = format!("ots_{}", c.name);
                print_info(&format!("Stopping and removing container {}", name));
                print_and_run(&["docker", "rm", "-f", &name]);
            }
        }
        if !found {
            print_error(&format!("Container {} not found", arg));
        }
    }
}

fn up_container(arguments: &[String]) {
    if arguments.is_empty() {
        print_error("Error: 'up' command requires at least one container");
        return;
    }
    let config=get_config();
    for arg in arguments {
        let mut found = false;
        for c in &config.ots {
            if c.name == *arg {
                found = true;
                print_info(&format!("Starting container {}", c.name));
                let cmd: Vec<&str> = c.command_line.split_whitespace().collect();
                print_and_run(&cmd);
            }
        }
        if !found {
            print_error(&format!("Container {} not found", arg));
        }
    }
}



fn get_config() -> DkConfig {
    // Charger la configuration depuis le fichier
    match DkConfig::load_from_file() {
        Ok(config) => {
            config
        }
        Err(_e) => {
            DkConfig::create_default()
        }
    }
}



//             "glances" => {
//                 let image = "nicolargo/glances:latest";
//                 image_helper::pull_image(image);
//                 print_info("Starting glances (http://localhost:61208)");
//                 print_and_run(&[
//                     "docker",
//                     "run",
//                     "-d",
//                     "--name",
//                     "glances",
//                     "--restart",
//                     "unless-stopped",
//                     "-v",
//                     "/var/run/docker.sock:/var/run/docker.sock:ro",
//                     "-p",
//                     "61208:61208",
//                     "--pid",
//                     "host",
//                     "--privileged",
//                     "-e",
//                     "GLANCES_OPT=-w",
//                     "-e",
//                     "PUID=1000",
//                     "-e",
//                     "PGID=1000",
//                     "-e",
//                     "TZ=Europe/Paris",
//                     image,
//                 ]);
//             }
//            

//             }
//             "cadvisor" => {
//                 let image = "gcr.io/cadvisor/cadvisor";
//                 image_helper::pull_image(image);
//                 print_info(&format!(
//                     "Starting cadvisor (http://localhost:{})",
//                     CADVISOR
//                 ));
//                 print_and_run(&[
//                     "docker",
//                     "run",
//                     "-v",
//                     "/:/rootfs:ro",
//                     "-v",
//                     "/var/run:/var/run:rw",
//                     "-v",
//                     "/sys:/sys:ro",
//                     "-v",
//                     "/var/lib/docker/:/var/lib/docker:ro",
//                     "--restart",
//                     "unless-stopped",
//                     "-p",
//                     &format!("{}:8080", CADVISOR),
//                     "-d",
//                     "--name",
//                     "ots_cadvisor",
//                     image,
//                 ]);
//             }
//             "dozzle" => {
//                 let image = "amir20/dozzle";
//                 image_helper::pull_image(image);
//                 print_info(&format!("Starting dozzle (http://localhost:{})", DOZZLE));
//                 print_and_run(&[
//                     "docker",
//                     "run",
//                     "--detach",
//                     "-v",
//                     "/var/run/docker.sock:/var/run/docker.sock",
//                     "-e",
//                     "DOZZLE_LEVEL=Debug",
//                     "--restart",
//                     "unless-stopped",
//                     "-p",
//                     &format!("{}:8080", DOZZLE),
//                     image,
//                 ]);
//             }
//             _ => {
//                 print_error(&format!("unknown image to run : {}", im));
//             }
//         }
//     }
// }

