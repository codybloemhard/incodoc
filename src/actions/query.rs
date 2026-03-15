use crate::*;

impl Doc {
    pub fn first_heading(&self) -> Option<&Heading> {
        for item in &self.items {
            if let DocItem::Section(section) = item {
                return Some(&section.heading);
            }
        }
        None
    }

    pub fn first_biggest_heading(&self) -> Option<&Heading> {
        let mut res = None;
        let mut best = 256;
        for item in &self.items {
            if let DocItem::Section(section) = item {
                let level = section.heading.level as u32;
                if best > level {
                    best = level;
                    res = Some(&section.heading);
                }
            }
        }
        res
    }
}
