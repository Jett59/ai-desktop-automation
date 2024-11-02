use python::{ensure_package_exists, run_script};
use ui::Ui;

mod ai;
mod python;
mod ui;

fn main() {
    env_logger::init();

    let ui = Ui::new();
    println!("{}", ui.root_name());
    ensure_package_exists("requests");
    println!("{}", run_script(r#"
print("Hello, world!")

def factorial(n):
  return 1 if n < 2 else n*factorial(n-1)

print(factorial(7))
"#.to_string()));
}
