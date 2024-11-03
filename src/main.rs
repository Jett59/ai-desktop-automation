use ai::AiContext;
use ui::Ui;

mod ai;
mod python;
mod ui;

fn main() {
    env_logger::init();

    let ui = Ui::new();
    for child in ui.root().children() {
        println!("{}", child.text_value());
    }
    let ai_context = AiContext::new();
    println!("{}", ai_context.text_query("What is the meaning of life?"));
}
