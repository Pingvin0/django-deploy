#![allow(unused)]
mod webserver_config;
mod app_config;
mod sgi;
mod utils;

use std::{path::PathBuf, process::{exit, Command}, fs::canonicalize};

use webserver_config::{WebServer};
use sgi::{SGIServer};

use inquire::{validator::Validation, CustomType};
use colored::Colorize;

use fs_extra::dir::{self, CopyOptions};

use crate::utils::print_install_failure;

fn main() {
    sudo::escalate_if_needed();

    let mut django_app_dir = PathBuf::from(
        inquire::Text::new("Django application path:")
        .with_validator(inquire::validator::MinLengthValidator::new(1))
        .prompt()
        .expect("Failed prompting for django app path.")
    );

    if !django_app_dir.exists() || !django_app_dir.is_dir() {
        eprintln!(
            "{} {} {}",
            "The path".red(),
            django_app_dir.to_str().unwrap().trim().green().bold(),
            "does not exist, or isn't a directory.".red()
        );
        exit(1);
    }

    let django_app = inquire::Text::new("Django project name:")
    .with_validator(inquire::validator::MinLengthValidator::new(1))
    .prompt()
    .expect("Failed prompting for django app name.");

    let mut django_proj_settings_dir = django_app_dir.join(PathBuf::from(&django_app));
    
    
    if !django_proj_settings_dir.exists() || !django_proj_settings_dir.is_dir() {
        eprintln!("{} {} {}", "The path".red(), django_proj_settings_dir.to_str().unwrap().trim().green().bold(), "does not exist, or isn't a directory.".red());
        exit(1);
    }
    
    // If sgirunner user does not exist
    if !Command::new("id")
    .arg("sgirunner")
    .output().unwrap().status.success() {
        println!("{}", "sgirunner user does not exist. Creating...".cyan());
        let add_user = Command::new("useradd")
        .args(["-r", "-M", "-s", "/sbin/nologin", "sgirunner"])
        .output();

        if let Err(e) = add_user {
            eprintln!("{} {}", "Failed creating sgirunner user. Error:".red(), e.to_string().bold().red());
            exit(1);
        }

        let cmd = add_user.unwrap();
        if !cmd.status.success() {
            eprintln!("Failed creating sgirunner user.");
            print_install_failure(&cmd);
            exit(1);
        }



        println!("{}", "Created sgirunner user".green());
    }

    

    if inquire::Confirm::new("Copy app to /django-apps?")
    .with_default(true).prompt().expect("Failed prompting for app copy.") {
        let django_apps = PathBuf::from("/django-apps");
        if !django_apps.exists() {
            let create = std::fs::create_dir(&django_apps);
            if let Err(e) = create {
                eprintln!("{} {}", "Failed creating /django-apps. Error:".red(), e.to_string().bold().red());
            }
        }
        // Separate if, cause if the folder was created above we want to
        // check that as well.
        if django_apps.exists() {
            let new_app_dir = django_apps.join(&django_app);
            if new_app_dir.exists() {
                eprintln!("{}", "Target app directory already exists!".red());
                exit(1);
            }
            
            std::fs::create_dir(&new_app_dir);

            let copy = dir::copy(&django_app_dir, new_app_dir, &CopyOptions::new());

            if let Err(e) = copy {
                eprintln!("{} {}", "Failed copying app dir to /django-apps. Error:".red(), e.to_string().bold().red());
            } else {
                django_app_dir = django_apps.join(&django_app);
                django_proj_settings_dir = django_app_dir.join(&django_app);


                let chown = Command::new("chown")
                .args(["-R", "sgirunner:sgirunner", "/django-apps"])
                .output();

                if let Err(e) = chown{
                    eprintln!("{} {}", "Failed setting ownership for new app folder. Error:".red(), e.to_string().bold().red());
                }

                let chmod = Command::new("chmod")
                .args(["770", "-R", "/django-apps"])
                .output();

                if let Err(e) = chmod{
                    eprintln!("{} {}", "Failed setting permissions for new app folder. Error:".red(), e.to_string().bold().red());
                }

            }
        }
    }
    


    let wsgi_server = inquire::Select::new("Please select SGI server:", vec![
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
        .prompt()
        .expect("Failed prompting for port number.");

        wsgi_server.create_systemd_service(
            &django_app,

            django_app_dir
            .canonicalize().unwrap()
            .to_str().unwrap(),

            port,
            inquire::Select::new(
                "Please select which SGI file you want to use:",
                vec![
                    ".wsgi",
                    ".asgi"
                ]
            ).prompt().expect("Failed to prompt for SGI file.")
        );
    }





    let web_server = inquire::Select::new("Please select web server", vec![
        WebServer::NGINX,
        WebServer::Apache,
        WebServer::None,
        
    ])
    .prompt()
    .expect("Error during prompting webserver type.");
    web_server.run_checks();

    
    if web_server != WebServer::None {
        let web_server_dir = 
        PathBuf::from(
        inquire::Text::new("Web server configuration directory")
        .with_default(match web_server {
            WebServer::Apache => "/etc/apache2",
            WebServer::NGINX => "/etc/nginx",
            _ => panic!("None selected even though it shouldn't be possible")
        }).prompt()
        .expect("Error during prompting for webserver config dir."));

        if !web_server_dir.exists() {
            eprintln!("{} {} {}", "Web server configuration directory".red(),  web_server_dir.to_str().unwrap().trim().green(), "does not exist!".red());
            exit(1);
        }

        

        web_server.create_config(
            &web_server_dir
        );
       
    }
    




}
