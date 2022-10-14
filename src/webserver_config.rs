use std::{fmt::Display, fs, path::PathBuf, process::exit};

#[derive(PartialEq)]
pub enum WebServer {
    Apache,
    NGINX,
    None
}

mod nginx;


impl Display for WebServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", match self {
            WebServer::Apache => "Apache",
            WebServer::NGINX => "NGINX",
            WebServer::None => "None (Don't configure)"
        });
    }
}

pub fn handle_nginx(web_server_dir: &PathBuf) {
    let static_files = inquire::Confirm::new("Configure staticfiles?")
    .with_default(true)
    .prompt()
    .expect("Failed asking for staticfile config.");

}

pub fn handle_apache(web_server_dir: &PathBuf) {

}