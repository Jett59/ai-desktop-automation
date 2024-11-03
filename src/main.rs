use ai::AiContext;
use ui::Ui;

mod ai;
mod python;
mod ui;

fn main() {
    env_logger::init();

    let ui = Ui::new();
    println!("{}", ui.root_name());
    let ai_context = AiContext::new();
    println!("{}", ai_context.text_query("What is the meaning of life?"));
}
