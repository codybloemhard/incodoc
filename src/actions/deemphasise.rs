use crate::{ EmOrText, Emphasis };

/// Obtain a single plain string from emphasised content.
pub trait DeEmphasise {
    fn deemphasise(&self) -> String;
}

impl DeEmphasise for Emphasis {
    fn deemphasise(&self) -> String {
        self.text.clone()
    }
}

impl DeEmphasise for Vec<EmOrText> {
    fn deemphasise(&self) -> String {
        let mut res = String::new();
        for item in self {
            match item {
                EmOrText::Text(string) => res.push_str(string),
                EmOrText::Em(emphasis) => res.push_str(&emphasis.text),
            }
        }
        res
    }
}

