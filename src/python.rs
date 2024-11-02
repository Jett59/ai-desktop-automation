use std::{
    env,
    fs::{self, canonicalize},
    io::Write,
    path::{absolute, PathBuf},
    process::{Command, Stdio},
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

pub fn run_script(text: String) -> String {
    enter_virtual_environment();

    let mut command = Command::new(virtual_environment_executable("python"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    // This needs to be run from a separate thread to prevent deadlocks.
    // If we tried to write it all on this thread, the program might (and probably would) try to write to stdout before everything had been written.
    // This would require us to read from it, which we would not do until we had finished writing.
    let mut stdin = command.stdin.take().expect("Failed to get stdin handle");
    std::thread::spawn(move || {
        stdin.write_all(text.as_bytes()).expect("Failed to write to stdin");
    });

    let output = command
        .wait_with_output()
        .expect("Failed to get output of command");
    String::from_utf8(output.stdout).expect("Python generated non-UTF8 output")
}
