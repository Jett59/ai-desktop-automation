use crate::{python, ui::{Ui, UiNode}};

pub struct AiContext {}

impl AiContext {
    pub fn new() -> Self {
        // The API key should be in the GOOGLE_API_KEY environment variable
        std::env::var("GOOGLE_API_KEY").expect("No GOOGLE_API_KEY found");
        python::ensure_package_exists("google-generativeai");

        Self {}
    }

    pub fn text_query(&self, text: &str) -> String {
        python::run_script(
            r#"
import os
import google.generativeai as genai

GOOGLE_API_KEY=os.environ.get('GOOGLE_API_KEY')
genai.configure(api_key=GOOGLE_API_KEY)
model = genai.GenerativeModel('gemini-1.5-flash')
result = model.generate_content('{query}')
print(result.text)
        "#
            .replace("{query}", &text.replace("'", "\\'")),
        )
    }
}

struct UiInfo<'a> {
    current_focus: UiNode<'a>,
    ancestors: Vec<UiNode<'a>>,
    windows: Vec<UiNode<'a>>,
}

impl<'a> UiInfo<'a> {
    fn new(ui: &'a Ui) -> Self {
        Self {
            current_focus: ui.current_focus(),
            ancestors: ui.current_focus().ancestors(),
            windows: ui.root().children(),
        }
    }
}
