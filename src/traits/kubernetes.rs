#![allow(dead_code)]
use std::error::Error;
use std::process::Child;

pub trait Kubernetes {
    type Success;
    fn get_namespaces() -> Result<Vec<String>, Box<dyn Error>>;
    fn get_services() -> Result<Vec<String>, Box<dyn Error>>;
    fn forward_connection(pod: &Pod) -> Result<Child, Box<dyn Error>>;
}
#[derive(Clone, Debug)]
pub struct Pod {
    context: String,
    namespace: String,
    service: String,
    port: i32,
}

impl Pod {
    pub(crate) fn new(context: &str, namespace: &str, service: &str, port: i32) -> Self {
        Self {
            context: context.to_string(),
            namespace: namespace.to_string(),
            service: service.to_string(),
            port,
        }
    }
    pub fn get_context(&self) -> String {
        self.context.clone()
    }
    pub fn set_context(&mut self, context: &str) {
        self.context = context.to_string();
    }
    pub fn set_namespace(&mut self, namespace: &str) {
        self.namespace = namespace.to_string();
    }
    pub fn set_service(&mut self, service: &str) {
        self.service = service.to_string();
    }

    pub fn get_service(&self) -> String {
        self.service.clone()
    }
    pub fn set_port(&mut self, port: i32) {
        self.port = port;
    }

    pub fn get_namespace(&self) -> String {
        self.namespace.clone()
    }
    pub fn get_port(&self) -> i32 {
        self.port.clone()
    }
}
