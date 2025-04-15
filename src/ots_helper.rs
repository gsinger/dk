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
