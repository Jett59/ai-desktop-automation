use std::sync::atomic::{AtomicU32, Ordering};

use windows::{
    core::Interface,
    Win32::{
        System::Com::{CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_INPROC_SERVER}, UI::Accessibility::{
            CUIAutomation, IUIAutomation, IUIAutomationElement, IUIAutomationTextPattern,
            IUIAutomationValuePattern, TreeScope_Children, TreeScope_Descendants,
            UIA_ButtonControlTypeId, UIA_CheckBoxControlTypeId, UIA_ComboBoxControlTypeId,
            UIA_DataGridControlTypeId, UIA_DataItemControlTypeId, UIA_DocumentControlTypeId,
            UIA_EditControlTypeId, UIA_GroupControlTypeId, UIA_HeaderControlTypeId,
            UIA_HeaderItemControlTypeId, UIA_HyperlinkControlTypeId, UIA_ImageControlTypeId,
            UIA_ListControlTypeId, UIA_ListItemControlTypeId, UIA_MenuBarControlTypeId,
            UIA_MenuControlTypeId, UIA_MenuItemControlTypeId, UIA_PaneControlTypeId,
            UIA_ProgressBarControlTypeId, UIA_RadioButtonControlTypeId, UIA_ScrollBarControlTypeId,
            UIA_SeparatorControlTypeId, UIA_SliderControlTypeId, UIA_SpinnerControlTypeId,
            UIA_SplitButtonControlTypeId, UIA_StatusBarControlTypeId, UIA_TabControlTypeId,
            UIA_TabItemControlTypeId, UIA_TableControlTypeId, UIA_TextControlTypeId,
            UIA_TextPatternId, UIA_TitleBarControlTypeId, UIA_ToolBarControlTypeId,
            UIA_ToolTipControlTypeId, UIA_TreeControlTypeId, UIA_TreeItemControlTypeId,
            UIA_ValuePatternId, UIA_WindowControlTypeId, UIA_CONTROLTYPE_ID,
        }
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
            node_type: get_node_type(unsafe { self.root.CurrentControlType().unwrap() }),
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Window,
    WindowTitle,
    TextLabel,
    TextBox,
    Button,
    CheckBox,
    ComboBox,
    DataGrid,
    DataItem,
    Document,
    Group,
    Header,
    HeaderItem,
    Hyperlink,
    Image,
    List,
    ListItem,
    MenuBar,
    Menu,
    MenuItem,
    Pane,
    ProgressBar,
    RadioButton,
    ScrollBar,
    Separator,
    Slider,
    Spinner,
    SplitButton,
    StatusBar,
    Tab,
    TabItem,
    Table,
    ToolBar,
    ToolTip,
    TreeView,
    TreeItem,
    Unknown,
}

fn get_node_type(uia_id: UIA_CONTROLTYPE_ID) -> NodeType {
    #[allow(non_upper_case_globals)]
    match uia_id {
        UIA_ButtonControlTypeId => NodeType::Button,
        UIA_CheckBoxControlTypeId => NodeType::CheckBox,
        UIA_ComboBoxControlTypeId => NodeType::ComboBox,
        UIA_DataGridControlTypeId => NodeType::DataGrid,
        UIA_DataItemControlTypeId => NodeType::DataItem,
        UIA_DocumentControlTypeId => NodeType::Document,
        UIA_EditControlTypeId => NodeType::TextBox,
        UIA_GroupControlTypeId => NodeType::Group,
        UIA_HeaderControlTypeId => NodeType::Header,
        UIA_HeaderItemControlTypeId => NodeType::HeaderItem,
        UIA_HyperlinkControlTypeId => NodeType::Hyperlink,
        UIA_ImageControlTypeId => NodeType::Image,
        UIA_ListControlTypeId => NodeType::List,
        UIA_ListItemControlTypeId => NodeType::ListItem,
        UIA_MenuBarControlTypeId => NodeType::MenuBar,
        UIA_MenuControlTypeId => NodeType::Menu,
        UIA_MenuItemControlTypeId => NodeType::MenuItem,
        UIA_PaneControlTypeId => NodeType::Pane,
        UIA_ProgressBarControlTypeId => NodeType::ProgressBar,
        UIA_RadioButtonControlTypeId => NodeType::RadioButton,
        UIA_ScrollBarControlTypeId => NodeType::ScrollBar,
        UIA_SeparatorControlTypeId => NodeType::Separator,
        UIA_SliderControlTypeId => NodeType::Slider,
        UIA_SpinnerControlTypeId => NodeType::Spinner,
        UIA_SplitButtonControlTypeId => NodeType::SplitButton,
        UIA_StatusBarControlTypeId => NodeType::StatusBar,
        UIA_TabControlTypeId => NodeType::Tab,
        UIA_TabItemControlTypeId => NodeType::TabItem,
        UIA_TableControlTypeId => NodeType::Table,
        UIA_TextControlTypeId => NodeType::TextLabel,
        UIA_TitleBarControlTypeId => NodeType::WindowTitle,
        UIA_ToolBarControlTypeId => NodeType::ToolBar,
        UIA_ToolTipControlTypeId => NodeType::ToolTip,
        UIA_TreeControlTypeId => NodeType::TreeView,
        UIA_TreeItemControlTypeId => NodeType::TreeItem,
        UIA_WindowControlTypeId => NodeType::Window,
        _ => NodeType::Unknown,
    }
}

pub struct UiNode<'a> {
    ui: &'a Ui,
    inner: IUIAutomationElement,
    node_type: NodeType,
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
                let node_type = get_node_type(child.CurrentControlType().unwrap());
                result.push(UiNode {
                    ui: self.ui,
                    inner: child,
                    node_type,
                });
            }
            result
        }
    }

    pub fn name(&self) -> String {
        unsafe { self.inner.CurrentName().unwrap().to_string() }
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
                self.name()
            }
        }
    }

    /// Finds the text value of all content components from all descendants of this component.
    pub fn text_content(&self) -> String {
        let mut result = String::new();
        unsafe {
            let descendants = self
                .inner
                .FindAll(
                    TreeScope_Descendants,
                    Some(&self.ui.uia_handle.CreateTrueCondition().unwrap()),
                )
                .unwrap();
            for i in 0..descendants.Length().unwrap() {
                let descendant = descendants.GetElement(i).unwrap();
                if descendant.CurrentIsContentElement().unwrap().as_bool() {
                    let descendant_type = get_node_type(descendant.CurrentControlType().unwrap());
                    let text_content = UiNode {
                        ui: self.ui,
                        inner: descendant,
                        node_type: descendant_type,
                    }
                    .text_value();
                    if !text_content.is_empty() {
                        result = result + "\n" + &text_content;
                    }
                }
            }
        }
        result
    }

    pub fn node_type(&self) -> NodeType {
        self.node_type
    }
}
