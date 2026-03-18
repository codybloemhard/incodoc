use crate::*;

impl Doc {
    /// Collect a mutable reference to all links in the document.
    pub fn links_mut(&mut self, include_navs: bool) -> Vec<&mut Link> {
        let mut res = Vec::new();
        links_mut_doc(self, &mut res, include_navs);
        res
    }
}

/// Collect a mutable reference to all links in the document.
pub fn links_mut_doc<'a>(doc: &'a mut Doc, res: &mut Vec<&'a mut Link>, include_navs: bool) {
    if include_navs {
        links_mut_navs(&mut doc.navs, res);
    }
    for item in &mut doc.items {
        match item {
            DocItem::Section(section) => links_mut_section(section, res),
            DocItem::Paragraph(par) => links_mut_par(par, res),
        }
    }
}

/// Collect a mutable reference to all links in the navigation.
pub fn links_mut_navs<'a>(navs: &'a mut Vec<Nav>, res: &mut Vec<&'a mut Link>) {
    for nav in navs {
        for link in &mut nav.links {
            res.push(link);
        }
        links_mut_navs(&mut nav.subs, res);
    }
}

/// Collect a mutable reference to all links in the section.
pub fn links_mut_section<'a>(section: &'a mut Section, res: &mut Vec<&'a mut Link>) {
    for item in &mut section.items {
        match item {
            SectionItem::Paragraph(par) => links_mut_par(par, res),
            SectionItem::Section(section) => links_mut_section(section, res),
        }
    }
}

/// Collect a mutable reference to all links in the paragraph.
pub fn links_mut_par<'a>(par: &'a mut Paragraph, res: &mut Vec<&'a mut Link>) {
    for item in &mut par.items {
        match item {
            ParagraphItem::Link(link) => res.push(link),
            ParagraphItem::List(list) => links_mut_list(list, res),
            ParagraphItem::Table(table) => links_mut_table(table, res),
            _ => { },
        }
    }
}

/// Collect a mutable reference to all links in the list.
pub fn links_mut_list<'a>(list: &'a mut List, res: &mut Vec<&'a mut Link>) {
    for item in &mut list.items {
        links_mut_par(item, res);
    }
}

/// Collect a mutable reference to all links in the table.
pub fn links_mut_table<'a>(table: &'a mut Table, res: &mut Vec<&'a mut Link>) {
    for row in &mut table.rows {
        for item in &mut row.items {
            links_mut_par(item, res);
        }
    }
}

