use std::{fmt::Display, process::{Command, exit}, io::ErrorKind};
use colored::Colorize;
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

struct SGITemplate {
    gunicorn: &'static str,
    // daphne: &'static str,
    // uvicorngunicorn: &'static str,
}



impl SGIServer {
    fn is_installed(&self) -> bool{
        let package = format!("{}", self).to_lowercase();
        let test = Command::new("python3")
        .args(["-c", format!("\"import {}\"", package).as_str()])
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
    }
    pub fn run_checks(&self) {
        match self {
            SGIServer::Daphne => {

            }
            SGIServer::Gunicorn => {
                self.is_installed();
            }
            &SGIServer::Uvicorn => {

            }
        }
    }
    pub fn create_systemd_service(
        &self,
        project_name: &String,
        working_directory: &String,
        port: u16,
        sgi_file: &String
    ) {

    }
}
