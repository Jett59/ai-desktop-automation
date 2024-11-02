use std::sync::atomic::{AtomicU32, Ordering};

use windows::Win32::{
    System::Com::{CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_INPROC_SERVER},
    UI::Accessibility::{CUIAutomation, IUIAutomation, IUIAutomationElement},
};

pub struct Ui {
    uia_handle: IUIAutomation,
    root: IUIAutomationElement,
}

// We must initialize COM when the first instance is created and uninitialize it when the last one is destroyed.
static INSTANCE_COUNT: AtomicU32 = AtomicU32::new(0);

impl Ui {
    pub fn new() -> Self {
        unsafe {
            if INSTANCE_COUNT.fetch_add(1, Ordering::Relaxed) == 0 {
                CoInitialize(None).unwrap();
            }
            let guid = Box::new(CUIAutomation);
            let uia_handle: IUIAutomation =
                CoCreateInstance(Box::into_raw(guid), None, CLSCTX_INPROC_SERVER).unwrap();
            let root = uia_handle.GetRootElement().unwrap();
            Self { uia_handle, root }
        }
    }

    pub fn root_name(&self) -> String {
        unsafe { self.root.CurrentName().unwrap().to_string() }
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        if INSTANCE_COUNT.fetch_sub(1, Ordering::Relaxed) == 1 {
            unsafe {
                CoUninitialize();
            }
        }
    }
}
