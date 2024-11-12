use ai::AiContext;
use ui::Ui;

mod ai;
mod python;
mod ui;

fn main() {
    env_logger::init();

    let ui = Ui::new();
    let focused_element = ui.current_focus();
    println!(
        "{} ({:?})\n{}",
        focused_element.name(),
        focused_element.node_type(),
        focused_element.text_content()
    );
    let ai_context = AiContext::new();
    println!("{}", ai_context.text_query("What is the meaning of life?"));
}
