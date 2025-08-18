use crate::*;

use std::fmt::Write;

fn spaces_out(spaces: usize, output: &mut String) {
    for _ in 0..spaces {
        output.push(' ');
    }
}

fn str_out(string: &str, spaces: usize, output: &mut String) {
    spaces_out(spaces, output);
    output.push_str(string);
}

fn string_out(string: &str, spaces: usize, output: &mut String) {
    spaces_out(spaces, output);
    output.push('"');
    output.push_str(string);
    output.push('"');
}

fn text_out(string: &str, spaces: usize, output: &mut String) {
    spaces_out(spaces, output);
    output.push('\'');
    output.push_str(string);
    output.push('\'');
}

fn text_item_out(text: &str, spaces: usize, output: &mut String) {
    text_out(text, spaces, output);
    output.push_str(",\n");
}

fn code_text_out(code: &str, spaces: usize, output: &mut String) {
    str_out("'\n", spaces, output);
    spaces_out(spaces, output);
    for c in code.chars() {
        output.push(c);
        if c == '\n' {
            spaces_out(spaces, output);
        }
    }
    output.push('\n');
    str_out("',\n", spaces, output);
}

fn mtext_out(mtext: &TextWithMeta, spaces: usize, output: &mut String) {
    text_out(&mtext.text, spaces, output);
    output.push_str(" {\n");
    tags_out(&mtext.tags, spaces + 4, output);
    props_out(&mtext.props, spaces + 4, output);
    str_out("},\n", spaces, output);
}

fn date_out(date: Date, output: &mut String) {
    output.push_str(&date.year.to_string());
    output.push('/');
    let _ = write!(output, "{:0>2}", date.month);
    output.push('/');
    let _ = write!(output, "{:0>2}", date.day);
}

fn tags_out(tags: &Tags, spaces: usize, output: &mut String) {
    if tags.is_empty() { return; }
    str_out("tags {\n", spaces, output);
    for tag in tags {
        string_out(tag, spaces + 4, output);
        output.push_str(",\n");
    }
    str_out("},\n", spaces, output);
}

fn props_out(props: &Props, spaces: usize, output: &mut String) {
    if props.is_empty() { return; }
    str_out("props {\n", spaces, output);
    for kv in props {
        let not_err = kv_out(kv, spaces + 4, output);
        if not_err { output.push_str(",\n"); }
    }
    str_out("},\n", spaces, output);
}

fn kv_out((k, v): (&String, &PropVal), spaces: usize, output: &mut String) -> bool {
    if matches!(v, PropVal::Error(_)) {
        return false;
    }
    str_out("(", spaces, output);
    string_out(k, 0, output);
    output.push_str(", ");
    match v {
        PropVal::String(string) => string_out(string, 0, output),
        PropVal::Text(text) => text_out(text, 0, output),
        PropVal::Int(int) => output.push_str(&int.to_string()),
        PropVal::Date(date) => date_out(*date, output),
        PropVal::Error(_) => (),
    }
    output.push(')');
    true
}

fn emphasis_out(em: &Emphasis, spaces: usize, output: &mut String) {
    str_out("em {\n", spaces, output);
    let strength = match em.strength {
        EmStrength::Light => "l",
        EmStrength::Medium => "m",
        EmStrength::Strong => "s",
    };
    let etype = match em.etype {
        EmType::Emphasis => 'e',
        EmType::Deemphasis => 'd',
    };
    str_out(strength, spaces + 4, output);
    output.push(etype);
    output.push_str(",\n");
    string_out(&em.text, spaces + 4, output);
    output.push_str(",\n");
    tags_out(&em.tags, spaces + 4, output);
    props_out(&em.props, spaces + 4, output);
    str_out("},\n", spaces, output);
}

fn code_out(code: &CodeBlock, spaces: usize, output: &mut String) {
    str_out("code {\n", spaces, output);
    string_out(&code.language, spaces + 4, output);
    output.push_str(",\n");
    let mode = match code.mode {
        CodeModeHint::Show => "show",
        CodeModeHint::Runnable => "runnable",
        CodeModeHint::Run => "run",
        CodeModeHint::Replace => "replace",
    };
    string_out(mode, spaces + 4, output);
    output.push_str(",\n");
    code_text_out(&code.code, spaces + 4, output);
    tags_out(&code.tags, spaces + 4, output);
    props_out(&code.props, spaces + 4, output);
    str_out("},\n", spaces, output);
}

fn link_out(link: &Link, spaces: usize, output: &mut String) {
    str_out("link {\n", spaces, output);
    string_out(&link.url, spaces + 4, output);
    output.push_str(",\n");
    for item in &link.items {
        match item {
            LinkItem::String(string) => {
                string_out(string, spaces + 4, output);
                output.push_str(",\n");
            },
            LinkItem::Em(em) => {
                emphasis_out(em, spaces + 4, output);
            },
        }
    }
    tags_out(&link.tags, spaces + 4, output);
    props_out(&link.props, spaces + 4, output);
    str_out("},\n", spaces, output);
}

fn heading_out(head: &Heading, spaces: usize, plevel: usize, output: &mut String) -> usize {
    str_out("head {\n", spaces, output);
    str_out(&(head.level as usize - plevel).to_string(), spaces + 4, output);
    output.push_str(",\n");
    for item in &head.items {
        match item {
            HeadingItem::String(string) => {
                string_out(string, spaces + 4, output);
                output.push_str(",\n");
            },
            HeadingItem::Em(em) => {
                emphasis_out(em, spaces + 4, output);
            },
        }
    }
    tags_out(&head.tags, spaces + 4, output);
    props_out(&head.props, spaces + 4, output);
    str_out("},\n", spaces, output);
    head.level as usize
}

fn nav_out(nav: &[SNav], spaces: usize, output: &mut String) {
    str_out("nav {\n", spaces, output);
    for snav in nav {
        snav_out(snav, spaces + 4, output);
    }
    str_out("},\n", spaces, output);
}

fn snav_out(snav: &SNav, spaces: usize, output: &mut String) {
    str_out("snav {\n", spaces, output);
    string_out(&snav.description, spaces + 4, output);
    output.push_str(",\n");
    for link in &snav.links {
        link_out(link, spaces + 4, output);
    }
    for sub in &snav.subs {
        snav_out(sub, spaces + 4, output);
    }
    tags_out(&snav.tags, spaces, output);
    props_out(&snav.props, spaces, output);
    str_out("},\n", spaces, output);
}

fn par_items_out(items: &[ParagraphItem], spaces: usize, output: &mut String) {
    for item in items {
        match item {
            ParagraphItem::Text(text) => text_item_out(text, spaces, output),
            ParagraphItem::MText(mtext) => mtext_out(mtext, spaces, output),
            ParagraphItem::Em(em) => emphasis_out(em, spaces, output),
            ParagraphItem::Link(link) => link_out(link, spaces, output),
            ParagraphItem::Code(Ok(code)) => code_out(code, spaces, output),
            ParagraphItem::List(list) => list_out(list, spaces, output),
            ParagraphItem::Code(_) => { },
        }
    }
}

fn list_out(list: &List, spaces: usize, output: &mut String) {
    str_out("list {\n", spaces, output);
    spaces_out(spaces + 4, output);
    let ltype = match list.ltype {
        ListType::Distinct => "dl",
        ListType::Identical => "il",
        ListType::Checked => "cl",
    };
    output.push_str(ltype);
    output.push_str(",\n");
    for par in &list.items {
        paragraph_out(par, spaces + 4, output);
    }
    tags_out(&list.tags, spaces + 4, output);
    props_out(&list.props, spaces + 4, output);
    str_out("},\n", spaces, output);
}

fn paragraph_out(par: &Paragraph, spaces: usize, output: &mut String) {
    str_out("par {\n", spaces, output);
    par_items_out(&par.items, spaces + 4, output);
    tags_out(&par.tags, spaces + 4, output);
    props_out(&par.props, spaces + 4, output);
    str_out("},\n", spaces, output);
}

fn section_out(section: &Section, spaces: usize, plevel: usize, output: &mut String) {
    str_out("section {\n", spaces, output);
    let plevel = 1 + heading_out(&section.heading, spaces + 4, plevel, output);
    for item in &section.items {
        match item {
            SectionItem::Paragraph(par) => paragraph_out(par, spaces + 4, output),
            SectionItem::Section(section) => section_out(section, spaces + 4, plevel, output),
        }
    }
    tags_out(&section.tags, spaces + 4, output);
    props_out(&section.props, spaces + 4, output);
    str_out("},\n", spaces, output);
}

/// Unparse: take abstract documents structure and produce a string that is an incodoc.
/// Any output of this should be able to be parsed by this crate.
pub fn doc_out(doc: &Doc, output: &mut String) {
    tags_out(&doc.tags, 0, output);
    props_out(&doc.props, 0, output);
    for item in &doc.items {
        match item {
            DocItem::Nav(nav) => nav_out(nav, 0, output),
            DocItem::Paragraph(par) => paragraph_out(par, 0, output),
            DocItem::Section(section) => section_out(section, 0, 0, output),
        }
    }
}

