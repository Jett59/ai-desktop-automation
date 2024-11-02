use python::ensure_package_exists;
use ui::Ui;

mod ai;
mod python;
mod ui;

fn main() {
    env_logger::init();

    let ui = Ui::new();
    println!("{}", ui.root_name());
    ensure_package_exists("requests");
}
