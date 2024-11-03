use std::sync::atomic::{AtomicU32, Ordering};

use windows::{
    core::Interface,
    Win32::{
        System::Com::{CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_INPROC_SERVER},
        UI::Accessibility::{
            CUIAutomation, IUIAutomation, IUIAutomationElement, IUIAutomationTextPattern,
            IUIAutomationValuePattern, TreeScope_Children, UIA_TextPatternId, UIA_ValuePatternId,
        },
    },
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

    pub fn root(&self) -> UiNode {
        UiNode {
            ui: self,
            inner: self.root.clone(),
        }
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

pub struct UiNode<'a> {
    ui: &'a Ui,
    inner: IUIAutomationElement,
}

impl UiNode<'_> {
    pub fn children(&self) -> Vec<UiNode> {
        unsafe {
            let children = self
                .inner
                .FindAll(
                    TreeScope_Children,
                    Some(&self.ui.uia_handle.CreateTrueCondition().unwrap()),
                )
                .unwrap();

            let mut result = Vec::with_capacity(children.Length().unwrap() as usize);
            for i in 0..children.Length().unwrap() {
                let child = children.GetElement(i).unwrap();
                result.push(UiNode {
                    ui: self.ui,
                    inner: child,
                });
            }
            result
        }
    }

    pub fn text_value(&self) -> String {
        unsafe {
            // ref: https://stackoverflow.com/questions/23850176/c-sharp-system-windows-automation-get-element-text
            if let Ok(value_pattern) = self.inner.GetCurrentPattern(UIA_ValuePatternId) {
                let value: IUIAutomationValuePattern = value_pattern.cast().unwrap();
                value.CurrentValue().unwrap().to_string()
            } else if let Ok(text_pattern) = self.inner.GetCurrentPattern(UIA_TextPatternId) {
                let text: IUIAutomationTextPattern = text_pattern.cast().unwrap();
                let string = text
                    .DocumentRange()
                    .unwrap()
                    .GetText(-1)
                    .unwrap()
                    .to_string();
                if let Some(without_suffix) = string.strip_suffix('\r') {
                    without_suffix.to_string()
                } else {
                    string
                }
            } else {
                self.inner.CurrentName().unwrap().to_string()
            }
        }
    }
}
