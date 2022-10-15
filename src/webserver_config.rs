use std::{fmt::{Display, format}, fs::{self, File}, path::PathBuf, process::{exit, Command}};

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
    fn test_config(&self) -> bool {
        let cmd = Command::new(match self {
            Self::Apache => "apachectl",
            Self::NGINX => "nginx",
            _ => panic!("test_config on WebServer::None")
        })
        .arg(match self {
            Self::Apache => "configtest",
            Self::NGINX => "-t",
            _ => panic!("test_config on WebServer::None")
        })
        .output();

        if let Err(e) = cmd {
            return false;
        }

        let cmd = cmd.unwrap();

        if !cmd.status.success() {
            eprintln!("{}", "Web server config is invalid!".red());
            print_install_failure(&cmd);

            return false;
        }

        return true;
    }

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
                "apache2",
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
            self.install();
        }
        println!("{}", "Testing web server configuration before adding new config file.".blue());
        if self.test_config() {

        } else {
            eprintln!("{}", "Web server config is faulty.".bright_red().bold());
            if !inquire::Confirm::new("Proceed?").with_default(true).prompt().expect("Failed prompting for proceed") {
                eprintln!("{}", "Exiting process...".red());
                exit(1);
            }
        }
    }

    pub fn create_config(
        &self,
        config_dir: &PathBuf
    ) {
        if self == &Self::None {return;}

        println!("{}", "Creating web server config file.".cyan());

        let domain = inquire::Text::new("Enter website domain:")
        .with_validator(inquire::validator::MinLengthValidator::new(3))
        .prompt().expect("Failed prompting for domain.");

        let ac_domain = inquire::Text::new("Enter frontend domain (for Access-Control-Allow-Origin)")
        .with_validator(inquire::validator::MinLengthValidator::new(3))
        .prompt().expect("Failed prompting for AC domain.");

        let prefetch = inquire::Confirm::new("Include Access-Control-Allow-Origin in prefetch req? (Recommended)")
        .with_default(true).prompt().expect("Failed prompting for prefetch cfg");

        let cfg_file = config_dir.join("sites-available/").join(format!("django-{}.conf", domain));

        if cfg_file.exists() {
            eprintln!("{}", "Web server config file already exists! Skipping step.".red());
            return;
        }

        let file = File::create(&cfg_file);

    }
}

