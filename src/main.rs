#![allow(unused)]
mod webserver_config;
mod app_config;
mod sgi;

use std::{path::PathBuf, process::exit};

use webserver_config::{WebServer, handle_apache, handle_nginx};
use sgi::{SGIServer};

use inquire::{validator::Validation, CustomType};
use colored::Colorize;

fn main() {
    let django_app = PathBuf::from(inquire::Text::new("Django application path: ")
    .prompt()
    .expect("Failed asking for django app path"));

    if !django_app.exists() || !django_app.is_dir() {
        eprintln!("{} {} {}", "The path".red(), django_app.to_str().unwrap().trim().green().bold(), "does not exist, or isn't a directory.".red());
        exit(1);
    }

    


    let wsgi_server = inquire::Select::new("Please select SGI server: ", vec![
        SGIServer::Gunicorn,
        SGIServer::Daphne,
        SGIServer::Uvicorn
    ]).prompt().expect("Failed prompting for wsgi server.");

    wsgi_server.run_checks();

    let want_systemd_service = inquire::Confirm::new("Create systemd service?")
    .with_default(true)
    .prompt().expect("Failed prompting for systemd service");

    if want_systemd_service {
        let port = CustomType::<u16>::new("Please enter port number to run service on:")
        .with_default(8000)
        .with_formatter(&|i| format!(":{:.2}", i))
        .with_error_message("Please type a valid port number")
        .with_help_message("Type the amount in US dollars using a decimal point as a separator")
        .prompt();

        // wsgi_server.create_systemd_service(

        // );
    }





    let web_server = inquire::Select::new("Please select web server", vec![
        WebServer::NGINX,
        WebServer::Apache,
        WebServer::None,
        
    ])
    .prompt()
    .expect("Error during asking webserver type.");

    
    if web_server != WebServer::None {
        let web_server_dir = 
        PathBuf::from(
        inquire::Text::new("Web server configuration directory")
        .with_default(match web_server {
            WebServer::Apache => "/etc/apache2",
            WebServer::NGINX => "/etc/nginx",
            _ => panic!("None selected even though it shouldn't be possible")
        }).prompt()
        .expect("Error during asking for webserver config dir."));

        if !web_server_dir.exists() {
            eprintln!("{} {} {}", "Web server configuration directory".red(),  web_server_dir.to_str().unwrap().trim().green(), "does not exist!".red());
            exit(1);
        }


        match web_server {
            WebServer::NGINX => {handle_nginx(&web_server_dir)},
            WebServer::Apache => {handle_apache(&web_server_dir)},
            WebServer::None => {panic!()}
        }
       
    }
    




}
