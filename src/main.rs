mod traits;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::process::{Child, ExitStatus};
use traits::kubernetes::Pod;

mod kubernetes_kubectl_implementation;
mod tui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_yml = read_config();

    tui::run(config_yml.unwrap()).await?;

    Ok(())
}

pub struct PodUI {
    pod: Pod,
    process: Option<Child>,
}

impl PodUI {
    pub fn get_service(&self) -> String {
        self.pod.get_service().clone()
    }

    pub fn get_namespace(&self) -> String {
        self.pod.get_namespace().clone()
    }

    pub fn get_context(&self) -> String {
        self.pod.get_context().clone()
    }

    pub fn get_port(&self) -> i32 {
        self.pod.get_port().clone()
    }

    pub fn is_running(&mut self) -> bool {
        self.process.as_mut().map_or(false, |child| {
            match child.try_wait() {
                Ok(Some(_status)) => false, // Process has exited
                Ok(None) => true,           // Process is still running
                Err(_e) => false,           // Error checking the process
            }
        })
    }

    pub fn stop(&mut self) -> io::Result<ExitStatus> {
        if let Some(ref mut child) = self.process {
            child.kill()?;
            let status = child.wait()?;
            self.process = None; // Clear the process after killing it
            Ok(status)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No running process"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Config {
    contexts: Vec<ContextConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            contexts: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ContextConfig {
    context: String,
    namespaces: Vec<NameSpaceConfig>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct NameSpaceConfig {
    namespace: String,
    pods: Vec<PodConfig>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PodConfig {
    service: String,
    port: i32,
}

fn read_config() -> Result<Vec<PodUI>, serde_yml::Error> {
    let home = home_dir().expect("Failed to get home directory");
    let env_var = "KFORWARD_CONFIG";

    let config_file: PathBuf = if let Ok(env_path) = std::env::var(env_var) {
        PathBuf::from(env_path).join("config.yml")
    } else {
        let config_dir: PathBuf = [home.to_str().unwrap(), ".config", "kforward"]
            .iter()
            .collect();
        config_dir.join("config.yml")
    };

    if !config_file.exists() {
        if let Some(parent) = config_file.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        File::create(&config_file).unwrap();
        println!("Created new configuration file {:?}", config_file);
    } else {
        println!("Configuration file already exists at {:?}", config_file);
    }
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&config_file)
        .unwrap();

    let mut result = Vec::new();

    let config: Config = serde_yml::from_reader(file).unwrap_or_default();

    for context in config.contexts.iter() {
        for namespace in context.namespaces.iter() {
            for pod in namespace.pods.iter() {
                result.push(PodUI {
                    pod: Pod::new(
                        &context.context,
                        &namespace.namespace,
                        &pod.service,
                        pod.port,
                    ),
                    process: None,
                })
            }
        }
    }
    Ok(result)
}
