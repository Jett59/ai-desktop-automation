use windows::Win32::{
    System::Com::{CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_INPROC_SERVER},
    UI::Accessibility::{CUIAutomation, IUIAutomation, TreeScope_Descendants},
};

fn main() {
    unsafe {
        CoInitialize(None).unwrap();
        let guid = Box::new(CUIAutomation);
        let ui_automation: IUIAutomation =
            CoCreateInstance(Box::into_raw(guid), None, CLSCTX_INPROC_SERVER).unwrap();
        let root = ui_automation.GetRootElement().unwrap();
        let children = root
            .FindAll(
                TreeScope_Descendants,
                Some(&ui_automation.CreateTrueCondition().unwrap()),
            )
            .unwrap();
        for i in 0..children.Length().unwrap() {
            let child = children.GetElement(i).unwrap();
            if child.CurrentIsControlElement().unwrap().as_bool() {
                let name = child.CurrentName().unwrap();
                println!("{}", name);
                println!("{:?}", child.CurrentControlType().unwrap());
            }
        }
        CoUninitialize();
    }
}
