use std::{fmt::{Display, format}, fs, path::PathBuf, process::{exit, Command}};

use colored::Colorize;
use inquire::ui::StyleSheet;

use super::utils::print_install_failure;

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

impl WebServer {
    fn is_installed(&self) -> bool{
        let cmd = Command::new(match self {
            Self::NGINX => "nginx",
            Self::Apache => "apache2",
            Self::None => panic!("is_installed check on None webserver.")
        })
        .arg("-h")
        .output();



        if let Err(e) = cmd {
            return false;
        }

        return cmd.unwrap().status.success();
    }
    
    fn install(&self) {
        
    }

    pub fn run_checks(&self) {
        let mut blue_text = inquire::ui::RenderConfig::empty();
        blue_text.prompt = StyleSheet::new()
        .with_fg(inquire::ui::Color::LightBlue);

        blue_text.answer = StyleSheet::new()
        .with_fg(inquire::ui::Color::LightGreen);

        if self == &WebServer::None {return}

        let installed = self.is_installed();

        if !installed && inquire::Confirm::new(
            &format!("It appears {} is not installed! Install it with apt?", self)
        ).with_render_config(blue_text).with_default(true).prompt().expect("Failed prompting for install of webserver.") {
            println!("{}", "Installing...".cyan());
            let update = Command::new("apt").arg("update").output();
            if let Err(e) = update {
                eprintln!(
                    "{} {}",
                    "Failed installing webserver! Error:".red(),
                    e.to_string().red().bold()
                );
                exit(1);
            }

            let install = Command::new("apt")
            .args(["install", "-y"])
            .args(match self {
                Self::Apache => vec![
                    "apache2e",
                    "apache2-utils"
                ],
                Self::NGINX => vec![
                    "nginx-full"
                ],
                Self::None => panic!("None during webserver install.")
            })
            .output();

            if let Err(e) = install {
                eprintln!(
                    "{} {}",
                    "Failed installing webserver! Error:".red(),
                    e.to_string().red().bold()
                );
                exit(1);
            }

            let install = install.unwrap();

            if !install.status.success() {
                eprintln!("{}", "Failed installation of webserver. Process output: ".red());
                print_install_failure(&install);
                eprintln!("{}", "Please install the selected webserver and try again!".bright_red().bold());
                exit(1);
            }
            

            println!("{}", "Installation of webserver was successful!".green());
        }
    }

    pub fn create_config(&self) {
        if self == &Self::None {return;}
    }
}

