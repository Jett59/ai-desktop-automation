use ai::AiContext;
use ui::Ui;

mod ai;
mod python;
mod ui;

fn main() {
    env_logger::init();

    let ui = Ui::new();
    for child in ui.root().children() {
        println!("{:?}\n{}", child.node_type(), child.text_content());
    }
    let ai_context = AiContext::new();
    println!("{}", ai_context.text_query("What is the meaning of life?"));
}
