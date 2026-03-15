use crate::*;

/// Prune document tree of various unwanted elements.
pub trait PruneIncodoc {
    fn prune_errors(&mut self);
    fn prune_contentless(&mut self);
    fn is_contentless(&self) -> bool;
}

impl<T> PruneIncodoc for Vec<T> where T: PruneIncodoc {
    fn prune_errors(&mut self) {
        for item in self {
            item.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        for item in self {
            item.prune_contentless();
        }
    }

    fn is_contentless(&self) -> bool {
        for item in self {
            if !item.is_contentless() {
                return false;
            }
        }
        true
    }
}

impl PruneIncodoc for Doc {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        self.navs.prune_errors();
        self.items.prune_errors();
    }

    fn prune_contentless(&mut self) {
        self.tags.prune_contentless();
        self.props.prune_contentless();
        self.navs.prune_contentless();
        self.navs.retain(|item| !item.is_contentless());
        self.items.prune_contentless();
        self.items.retain(|item| !item.is_contentless());
    }

    fn is_contentless(&self) -> bool {
        self.items.is_contentless() && self.navs.is_contentless()
    }
}

impl PruneIncodoc for DocItem {
    fn prune_errors(&mut self) {
        match self {
            DocItem::Paragraph(par) => par.prune_errors(),
            DocItem::Section(section) => section.prune_errors(),
        }
    }

    fn prune_contentless(&mut self) {
        match self {
            DocItem::Paragraph(par) => par.prune_contentless(),
            DocItem::Section(section) => section.prune_contentless(),
        }
    }

    fn is_contentless(&self) -> bool {
        match self {
            DocItem::Paragraph(par) => par.is_contentless(),
            DocItem::Section(section) => section.is_contentless(),
        }
    }
}

impl PruneIncodoc for String {
    fn prune_errors(&mut self) { }

    fn prune_contentless(&mut self) {
        let trimmed = self.trim();
        if trimmed.is_empty() {
            *self = trimmed.to_string();
        }
    }

    fn is_contentless(&self) -> bool {
        self.is_empty()
    }
}

impl PruneIncodoc for Tags {
    fn prune_errors(&mut self) { }

    fn prune_contentless(&mut self) {
        self.retain(|t|
            !t.is_empty() && !t.chars().all(char::is_whitespace)
        );
    }

    fn is_contentless(&self) -> bool {
        self.is_empty()
    }
}

impl PruneIncodoc for Props {
    fn prune_errors(&mut self) {
        self.retain(|_, v| !v.is_error());
    }

    fn prune_contentless(&mut self) {
        for pval in self.values_mut() {
            pval.prune_contentless();
        }
        self.retain(|k, v|
            !k.is_empty() && !v.is_contentless() && !k.chars().all(char::is_whitespace)
        );
    }

    fn is_contentless(&self) -> bool {
        self.is_empty()
    }
}

impl PruneIncodoc for PropVal {
    fn prune_errors(&mut self) {  }

    fn prune_contentless(&mut self) {
        match self {
            Self::String(string) |
            Self::Text(string) => string.prune_contentless(),
            _ => { },
        }
    }

    fn is_contentless(&self) -> bool {
        match self {
            Self::String(string) |
            Self::Text(string) => string.is_empty(),
            _ => false,
        }
    }
}

impl PruneIncodoc for Section {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        self.items.prune_errors();
    }

    fn prune_contentless(&mut self) {
        self.heading.prune_contentless();
        self.items.prune_contentless();
        self.items.retain(|item| !item.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.heading.is_contentless() && self.items.is_contentless()
    }
}

impl PruneIncodoc for SectionItem {
    fn prune_errors(&mut self) {
        match self {
            Self::Paragraph(par) => par.prune_errors(),
            Self::Section(section) => section.prune_errors(),
        }
    }

    fn prune_contentless(&mut self) {
        match self {
            Self::Paragraph(par) => par.prune_contentless(),
            Self::Section(section) => section.prune_contentless(),
        }
    }

    fn is_contentless(&self) -> bool {
        match self {
            Self::Paragraph(par) => par.is_contentless(),
            Self::Section(section) => section.is_contentless(),
        }
    }
}

impl PruneIncodoc for Heading {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        self.items.prune_errors();
    }

    fn prune_contentless(&mut self) {
        self.items.prune_contentless();
        self.items.retain(|item| !item.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.items.is_contentless()
    }
}

impl PruneIncodoc for EmOrText {
    fn prune_errors(&mut self) {
        if let Self::Em(em) = self {
            em.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        match self {
            Self::Text(string) => string.prune_contentless(),
            Self::Em(em) => em.prune_contentless(),
        }
    }

    fn is_contentless(&self) -> bool {
        match self {
            Self::Text(string) => string.is_empty(),
            Self::Em(em) => em.is_contentless(),
        }
    }
}

impl PruneIncodoc for Paragraph {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        self.items.prune_errors();
    }

    fn prune_contentless(&mut self) {
        self.items.prune_contentless();
        self.items.retain(|item| !item.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.items.is_contentless()
    }
}

impl PruneIncodoc for ParagraphItem {
    fn prune_errors(&mut self) {
        match self {
            Self::MText(mtext) => mtext.prune_errors(),
            Self::Em(em) => em.prune_errors(),
            Self::Code(Ok(code)) => code.prune_errors(),
            Self::Link(link) => link.prune_errors(),
            Self::List(list) => list.prune_errors(),
            _ => (),
        }
    }

    fn prune_contentless(&mut self) {
        match self {
            Self::Text(text) => text.prune_contentless(),
            Self::MText(mtext) => mtext.prune_contentless(),
            Self::Em(em) => em.prune_contentless(),
            Self::Code(Ok(code)) => code.prune_contentless(),
            Self::Code(Err(_)) => { },
            Self::Link(link) => link.prune_contentless(),
            Self::List(list) => list.prune_contentless(),
            Self::Table(table) => table.prune_contentless(),
        }
    }

    fn is_contentless(&self) -> bool {
        match self {
            Self::Text(text) => text.is_empty(),
            Self::MText(mtext) => mtext.is_contentless(),
            Self::Em(em) => em.is_contentless(),
            Self::Code(Ok(code)) => code.is_contentless(),
            Self::Code(Err(_)) => true,
            Self::Link(link) => link.is_contentless(),
            Self::List(list) => list.is_contentless(),
            Self::Table(table) => table.is_contentless(),
        }
    }
}

impl PruneIncodoc for Emphasis {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
    }

    fn prune_contentless(&mut self) {
        self.text.prune_contentless();
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.text.is_empty()
    }
}

impl PruneIncodoc for List {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        self.items.prune_errors();
    }

    fn prune_contentless(&mut self) {
        self.items.prune_contentless();
        self.items.retain(|par| !par.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.items.is_contentless()
    }
}

impl PruneIncodoc for Table {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        self.rows.prune_errors();
    }

    fn prune_contentless(&mut self) {
        self.rows.prune_contentless();
        self.rows.retain(|row| !row.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.rows.is_contentless()
    }
}

impl PruneIncodoc for TableRow {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        self.items.prune_errors();
    }

    fn prune_contentless(&mut self) {
        self.items.prune_contentless();
        self.items.retain(|par| !par.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.items.is_contentless()
    }
}

impl PruneIncodoc for Nav {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        self.links.prune_errors();
        self.subs.prune_errors();
    }

    fn prune_contentless(&mut self) {
        self.description.prune_contentless();
        self.links.prune_contentless();
        self.links.retain(|link| !link.is_contentless());
        self.subs.prune_contentless();
        self.subs.retain(|sub| !sub.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.subs.is_contentless() && self.links.is_contentless()
    }
}

impl PruneIncodoc for Link {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        self.items.prune_errors();
    }

    fn prune_contentless(&mut self) {
        self.url.prune_contentless();
        self.items.prune_contentless();
        self.items.retain(|item| !item.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.url.is_contentless() && self.items.is_contentless()
    }
}

impl PruneIncodoc for CodeBlock {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
    }

    fn prune_contentless(&mut self) {
        self.language.prune_contentless();
        self.code.prune_contentless();
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.code.is_empty()
    }
}

impl PruneIncodoc for TextWithMeta {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
    }

    fn prune_contentless(&mut self) {
        self.text.prune_contentless();
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.text.is_empty()
    }
}

