use std::{
    env,
    fs::{self, canonicalize},
    path::{absolute, PathBuf},
    process::Command,
};

use log::info;

pub fn ensure_package_exists(name: &str) {
    enter_virtual_environment();

    info!("Installing {} using pip", name);
    let succeeded = Command::new(virtual_environment_executable("pip"))
        .arg("install")
        .arg(name)
        .status()
        .map(|status| status.success())
        .expect("Failed to run pip");
    if !succeeded {
        panic!("Failed to install {}", name);
    }
}

fn enter_virtual_environment() {
    // If we are already in one (VIRTUAL_ENV is set), just return
    if env::var("VIRTUAL_ENV").is_ok() {
        info!("Already in virtual environment.");
        set_up_environment(&env::var("VIRTUAL_ENV").unwrap());
        return;
    }
    // Check if ./venv exists
    if fs::metadata("./venv").is_ok_and(|data| data.is_dir()) {
        info!("Found virtual environment at ./venv");
        set_up_environment("./venv");
        return;
    }

    // Otherwise we have to create it
    let python_executable = find_python_executable();
    info!("Creating virtual environment at ./venv");
    let created_venv = Command::new(python_executable)
        .arg("-m")
        .arg("venv")
        .arg("venv")
        .status()
        .expect("Failed to run python")
        .success();
    if !created_venv {
        panic!("Failed to create virtual environment");
    }

    set_up_environment("./venv");
}

fn set_up_environment(root: &str) {
    env::set_var("VIRTUAL_ENV", canonicalize(root).unwrap());
    env::set_var(
        "PYTHONPATH",
        canonicalize(root.to_string() + "/lib/site-packages").unwrap(),
    );
}

fn find_python_executable() -> &'static str {
    // We try python3, python, py (in that order)
    let options = ["python", "python3", "py"];
    for option in options {
        info!("Attempting to run {}", option);
        let Ok(succeeded) = Command::new(option)
            .arg("--version")
            .status()
            .map(|status| status.success())
        else {
            continue;
        };
        if succeeded {
            return option;
        }
    }
    panic!("No python found (tried {:?})", options);
}

fn virtual_environment_executable(name: &str) -> PathBuf {
    let virtual_environment_path = env::var("VIRTUAL_ENV").expect("Not in a virtual environment");
    if fs::exists(absolute(virtual_environment_path.clone() + "/bin").unwrap()).unwrap_or(false) {
        absolute(virtual_environment_path + "/bin/" + name).unwrap()
    } else if fs::exists(absolute(virtual_environment_path.clone() + "\\bin").unwrap())
        .unwrap_or(false)
    {
        absolute(virtual_environment_path + "\\bin\\" + name).unwrap()
    } else if fs::exists(absolute(virtual_environment_path.clone() + "\\scripts").unwrap())
        .unwrap_or(false)
    {
        absolute(virtual_environment_path + "\\scripts\\" + name).unwrap()
    } else {
        panic!("Failed to find virtual environment bin directory");
    }
}
