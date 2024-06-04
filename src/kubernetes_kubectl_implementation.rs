use std::{
    error::Error,
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
};

use crate::traits::kubernetes::{Kubernetes, Pod};
const HEADER: usize = 1;

pub struct KubernetesImpl {}

impl Kubernetes for KubernetesImpl {
    fn get_namespaces() -> Result<Vec<String>, Box<dyn Error>> {
        Ok(vec![String::from("d4i")]) // we only have a namespace valid so far
    }

    fn get_services() -> Result<Vec<String>, Box<dyn Error>> {
        let mut command = Command::new("kubectl");
        command.arg("--namespace=d4i").arg("get").arg("services");

        let mut child = command.stdout(Stdio::piped()).spawn().expect("");
        let stdout = child.stdout.take().expect("failed to open stdout");

        let reader = BufReader::new(stdout);
        let mut names: Vec<String> = Vec::new();

        for line in reader.lines().skip(HEADER) {
            let line = line.expect("failed to read line");
            let cols: Vec<&str> = line.split_whitespace().collect();
            if !cols.is_empty() {
                names.push(cols[0].to_string());
            }
        }
        let _ = child.kill();
        let _ = child.wait();
        Ok(names)
    }

    fn forward_connection(pod: &Pod) -> Result<Child, Box<dyn Error>> {
        let pod_clone = pod.clone();

        let output = Command::new("kubectl")
            .arg(format!("--context={}", pod_clone.get_context()))
            .arg(format!("--namespace={}", pod_clone.get_namespace()))
            .arg("port-forward")
            .arg(format!("svc/{}", pod_clone.get_service())) // neet to create the method
            .arg(format!("{}:80", pod_clone.get_port()))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("");

        Ok(output)
    }

    type Success = Child;
}
