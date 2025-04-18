use std::{collections::HashMap, process::Command};



// a trait to abstract the command execution
pub trait CommandExecutor {
    fn execute(&self, command: &[&str]) -> Result<String, String>;
}

// real implementation that call the external command
pub struct RealCommandExecutor;

impl CommandExecutor for RealCommandExecutor {

    fn execute(&self, command: &[&str]) -> Result<String, String> {
        let output = Command::new(command[0])
            .args(&command[1..])
            .output()
            .map_err(|e| e.to_string())?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
    
}

// // mock for tests
// pub struct MockCommandExecutor {
//     responses: HashMap<String, Result<String, String>>,
// }

// impl MockCommandExecutor {
//     pub fn new() -> Self {
//         Self {
//             responses: HashMap::new(),
//         }
//     }
    
//     pub fn expect(&mut self, command: &[&str], response: Result<String, String>) {
//         self.responses.insert(command.join(" "), response);
//     }
// }

// impl CommandExecutor for MockCommandExecutor {
//     fn execute(&self, command: &[&str]) -> Result<String, String> {
//         let cmd_str = command.join(" ");
//         self.responses.get(&cmd_str)
//             .cloned()
//             .unwrap_or_else(|| panic!("Unexpected command: {}", cmd_str))
//     }
// }
