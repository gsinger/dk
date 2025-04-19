use serde::{Deserialize, Serialize};
use std::{fs};
use std::path::{PathBuf};
use std::io::{self, Write};


#[derive(Serialize, Deserialize)]
pub struct DkConfig {
    pub ots: Vec<Ots>,
}

#[derive(Serialize, Deserialize)]
pub struct Ots {
    pub name: String,
    pub port: u32,
    pub command_line: String
}

const CONFIG_DIRECTORY: &str = ".dk";
const CONFIG_FILE_NAME: &str = "dk_config.json";


impl DkConfig {
    
    /// Save the DkConfig to a JSON file
    ///
    /// This method serializes the `DkConfig` instance into a JSON string
    /// and writes it to the configuration file located in the user's home directory.
    pub fn save_to_file(&self) -> io::Result<()> {
        let config_path = Self::get_config_path();
        let json = serde_json::to_string_pretty(self)?;
        let mut file = fs::File::create(config_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Load a DkConfig from a JSON file
    ///
    /// This method reads the configuration file, deserializes its JSON content,
    /// and returns a `DkConfig` instance.
    pub fn load_from_file() -> io::Result<DkConfig> {
        let config_path = Self::get_config_path();
        let json = fs::read_to_string(config_path)?;
        let config: DkConfig = serde_json::from_str(&json)?;
        Ok(config)
    }

    /// Create a default DkConfig instance
    ///
    /// This method creates a default configuration with predefined OTS services
    /// and saves it to the configuration file. 
    /// It returns the default `DkConfig` instance.
    pub fn create_default() -> DkConfig {
        let default_config = DkConfig {
            ots: vec![
                Ots {
                    name: String::from("portainer"),
                    port: 25003,
                    command_line: String::from(
                    "docker run -d \
                    --name ots_portainer \
                    -p 9000:9000 \
                    -p 25003:9443 \
                    -v /var/run/docker.sock:/var/run/docker.sock \
                    -v portainer_data:/data \
                    --restart unless-stopped \
                                        portainer/portainer-ce:latest")
                },

                Ots {
                name:String::from("sqlserver"),
                port:1433,
                command_line: String::from(
                "docker run -d \
                --name ots_sqlserver \
                -v sqlserver_data:/var/opt/mssql \
                -p 1433:1433 \
                -e ACCEPT_EULA=Y \
                -e SA_PASSWORD=Sh@dokN0tD€ad! \
                --restart unless-stopped \
                mcr.microsoft.com/mssql/server:2022-latest")
                },

                Ots {
                    name: String::from("kroki"),
                    port: 25100,
                    command_line: String::from(
                        "docker run -d \
                        --name ots_kroki \
                        -p 25100:8000 \
                        --restart unless-stopped \
                        yuzutech/kroki")
                },

                Ots {
                    name: String::from("excalidraw"),
                    port: 25101,
                    command_line: String::from(
                        "docker run -d \
                        --name ots_excalidraw \
                        -p 25000:80 \
                        --restart unless-stopped \
                        excalidraw/excalidraw")
                },
                Ots {
                    name: String::from("rabbitmq"),
                    port: 25101,
                    command_line: String::from(
                        "docker run -d \
                        --name ots_rabbitmq \
                        -p 15672:15672 \
                        -p 5672:5672 \
                        --mount type=volume,src=ots_rabbitmq,dst=/var/lib/rabbitmq
                        --restart unless-stopped \
                        rabbitmq:4.1.0-management")

                }
            ],
        };
        
        match default_config.save_to_file() {
            Ok(_) => {},
            Err(e) => 
            eprintln!("Error while saving the default configuration: {}", e),
        }

        default_config
    }

    /// Get the full path to the configuration file
    ///
    /// This method constructs the path to the configuration file in the user's home directory.

    fn get_config_path() -> PathBuf {
        if let Some(home_dir) = dirs::home_dir() {
            let full_path: PathBuf = home_dir.join(CONFIG_DIRECTORY);
            Self::ensure_config_directory_exists(&full_path);
            full_path.join(CONFIG_FILE_NAME)
        } else {
            panic!("Unable to retrieve home directory!");
        }
    }

/// Ensure the configuration directory exists
    ///
    /// This method creates the configuration directory if it does not already exist.
        fn ensure_config_directory_exists(config_dir: &PathBuf) {
        if !config_dir.exists() {
            fs::create_dir_all(config_dir).expect("unable to create config directory");
        }
    }

}