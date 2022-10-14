use std::{fmt::Display, process::{Command, exit}, io::{ErrorKind, BufWriter, Write}, path::PathBuf, fs::File};
use colored::Colorize;
use inquire::ui::StyleSheet;

use crate::utils::print_install_failure;
pub enum SGIServer {
    Gunicorn,
    Daphne,
    Uvicorn
}

impl Display for SGIServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", match self {
            SGIServer::Gunicorn => "Gunicorn",
            SGIServer::Daphne => "Daphne",
            SGIServer::Uvicorn => "Uvicorn"
        })
    }
}

pub struct SGITemplate {
    gunicorn: &'static str,
    // daphne: &'static str,
    // uvicorngunicorn: &'static str,
}

pub static SGI_TEMPLATES: SGITemplate = SGITemplate {
    gunicorn: include_str!("templates/sgi/gunicorn.service"),
    // daphne: include_str!("templates/sgi/daphne.service"),
    // uvicorn: include_str!("templates/sgi/uvicorn.service"),
};



impl SGIServer {
    fn is_installed(&self) -> bool{
        let package = format!("{}", self).to_lowercase();
        let test = Command::new("python3")
        .args(["-c", format!("import {}", package.trim()).as_str()])
        .output();


        
        
        if let Err(e) = test {
            eprintln!("{} {}", "Failed checking for package presence.\nError:".red(), e.to_string().bright_red().bold());

            if e.kind() == ErrorKind::NotFound {
                eprintln!("{}", "Do you have python3 installed? `python3` may not have been in $PATH.".blue());
            }
            exit(1);
        }

        let test = test.unwrap();
        
        
        
        return test.status.success();
    }
    fn install(&self) {
        let package = format!("{}", self).to_lowercase();
        let install = Command::new("python3")
        .args(["-m", "pip", "install", package.as_str()])
        .output();

        if let Err(e) = install {
            eprintln!("{}", "Failed installing package. Process output:".red());
            // eprintln!("")
            exit(1);
        }
        let install = install.unwrap();

        if !install.status.success() {
            eprintln!("{}", "Failed installing SGI server. Please try again.");
            print_install_failure(&install);

            exit(1);
        }

        eprintln!("{} {}", "Successfully installed".bright_blue(), package);
    }
    pub fn run_checks(&self) {
        let is_installed = self.is_installed();

        let mut blue_text = inquire::ui::RenderConfig::empty();
        blue_text.prompt = StyleSheet::new()
        .with_fg(inquire::ui::Color::LightBlue);

        blue_text.answer = StyleSheet::new()
        .with_fg(inquire::ui::Color::LightGreen);

        if !is_installed {
            println!(
                "{} {} {}",
                "It seems".red(), self.to_string().green().bold(),
                "is not installed on your system.".red()
            );
            let install = inquire::Confirm::new("Would you like to install it?").with_default(true)
            .with_render_config(blue_text)
            .prompt()
            .expect("Failed prompting for install of SGI server.");

            if !install {
                eprintln!("{}", "Please install the SGI server you chose, and start over!".red());
                exit(1);
            }

            self.install();
        }
    }
    pub fn create_systemd_service(
        &self,
        project_name: &String,
        working_directory: &str,
        port: u16,
        sgi_file: &str
    ) {
        let file = PathBuf::from(
            format!("/etc/systemd/system/django-{}-{}.service", self.to_string().to_lowercase(), project_name)
        );
        dbg!(&file);
        if file.exists() {
            eprintln!(
                "{} {} {}",
                "The file".red(),
                file.to_str().unwrap().red().bold(),
                "already exists.".red(),
            );
            eprintln!("{}", "Skipping service creation...".red());
            return;
        }

        let service_file = File::create(&file).expect("Failed creating systemd service file.");
        
        match self {
            
            Self::Gunicorn => {
               
               let mut writer = BufWriter::new(service_file);
               writer.write(
                SGI_TEMPLATES.gunicorn
                .replace("{working_directory}", working_directory)
                .replace("{port}", &format!("{port}"))
                .replace("{project_name}", &project_name)
                .replace("{sgi_file}", sgi_file)
                .as_bytes()
               );
            }
            Self::Daphne => {}
            Self::Uvicorn => {}
        }

        if inquire::Confirm::new("Enable service?")
        .with_default(true)
        .prompt()
        .expect("Failed prompting for enable service") {

            let cmd = Command::new("systemctl")
            .args(["enable", file.file_name().unwrap().to_str().unwrap()])
            .output();

            if let Err(e) = cmd {
                eprintln!("{} {}", "Failed executing systemctl enable. Error:".red(), e.to_string().red().bold());
                eprintln!("{}", "Skipping step...".blue());
                return;
            }

            let cmd = cmd.unwrap();

            if !cmd.status.success() {
                eprintln!("{}", "Failed enabling systemd service. Error:".red());
                print_install_failure(&cmd);
                return;
            }
            
        }




    }
}
