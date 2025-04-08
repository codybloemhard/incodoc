use crate::*;

/// Merge two objects by having one absorb the other.
pub trait Absorb {
    type Other;
    /// Absorb other into self.
    fn absorb(&mut self, other: Self::Other);
}

/// Prune document tree of various unwanted elements.
pub trait PruneIncodoc {
    fn prune_errors(&mut self);
    fn prune_contentless(&mut self);
    fn is_contentless(&self) -> bool;
}

fn squash_alg_text_case<'a>(
    ltext: &mut Option<&'a mut String>,
    lmtext: &mut Option<&mut TextWithMeta>,
    lem: &mut Option<&mut Emphasis>,
    text: &'a mut String, keep: &mut Vec<bool>
) {
    match ltext {
        None => {
            *ltext = Some(text);
            keep.push(true);
        },
        Some(t) => {
            t.push_str(text);
            keep.push(false);
        },
    }
    *lmtext = None;
    *lem = None;
}

fn squash_alg_mtext_case<'a>(
    ltext: &mut Option<&mut String>,
    lmtext: &mut Option<&'a mut TextWithMeta>,
    lem: &mut Option<&mut Emphasis>,
    mtext: &'a mut TextWithMeta, keep: &mut Vec<bool>
) {
    mtext.prune_contentless();
    if mtext.is_contentless() {
        keep.push(false);
    } else {
        *ltext = None;
        if let Some(t) = lmtext {
            if mtext.tags == t.tags && mtext.props == t.props {
                t.text.push_str(&mtext.text);
                keep.push(false);
            } else {
                *lmtext = Some(mtext);
                keep.push(true);
            }
        } else {
            *lmtext = Some(mtext);
            keep.push(true);
        }
    }
    *lem = None;
}

fn squash_alg_em_case<'a>(
    ltext: &mut Option<&mut String>,
    lmtext: &mut Option<&mut TextWithMeta>,
    lem: &mut Option<&'a mut Emphasis>,
    em: &'a mut Emphasis, keep: &mut Vec<bool>
) {
    *ltext = None;
    *lmtext = None;
    if let Some(e) = lem {
        if em.props == e.props && em.tags == e.tags
            && em.strength == e.strength && em.etype == e.etype {
            e.text.push_str(&em.text);
            keep.push(false);
        } else {
            *lem = Some(em);
            keep.push(true);
        }
    } else {
        *lem = Some(em);
        keep.push(true);
    }
}

fn squash_alg_def_case(
    ltext: &mut Option<&mut String>,
    lmtext: &mut Option<&mut TextWithMeta>,
    lem: &mut Option<&mut Emphasis>,
    keep: &mut Vec<bool>
) {
    keep.push(true);
    *ltext = None;
    *lmtext = None;
    *lem = None;
}

macro_rules! impl_squash_text_mtext_em {
    ($enumname:ident) => {
        pub fn squash(&mut self) {
            let mut keep = Vec::new();
            let mut ltext: Option<&mut String> = None;
            let mut lmtext: Option<&mut TextWithMeta> = None;
            let mut lem: Option<&mut Emphasis> = None;

            // squash items into earlier item if possible
            for item in &mut self.items {
                match item {
                    $enumname::Text(text) => {
                        squash_alg_text_case(&mut ltext, &mut lmtext, &mut lem, text, &mut keep);
                    },
                    $enumname::MText(mtext) => {
                        squash_alg_mtext_case(&mut ltext, &mut lmtext, &mut lem, mtext, &mut keep);
                    },
                    $enumname::Em(em) => {
                        squash_alg_em_case(&mut ltext, &mut lmtext, &mut lem, em, &mut keep);
                    },
                    _ => {
                        squash_alg_def_case(&mut ltext, &mut lmtext, &mut lem, &mut keep);
                    },
                }
            }

            // remove parts that have been squashed
            let mut kiter = keep.into_iter();
            self.items.retain(|_| kiter.next().unwrap());

            // downgrade MText's with no meta
            for item in &mut self.items {
                if let $enumname::MText(TextWithMeta { text, tags, props }) = item {
                    if tags.is_empty() && props.is_empty() {
                        *item = $enumname::Text(std::mem::take(text));
                    }
                }
            }

            // recurse
            for item in &mut self.items {
                item.squash();
            }
        }
    }
}

macro_rules! impl_squash_text_em {
    ($enumname:ident) => {
        pub fn squash(&mut self) {
            let mut keep = Vec::new();
            let mut ltext: Option<&mut String> = None;
            let mut lem: Option<&mut Emphasis> = None;

            // squash items into earlier item if possible
            for item in &mut self.items {
                match item {
                    $enumname::String(text) => {
                        squash_alg_text_case(&mut ltext, &mut None, &mut lem, text, &mut keep);
                    },
                    $enumname::Em(em) => {
                        squash_alg_em_case(&mut ltext, &mut None, &mut lem, em, &mut keep);
                    },
                }
            }

            // remove parts that have been squashed
            let mut kiter = keep.into_iter();
            self.items.retain(|_| kiter.next().unwrap());
        }
    }
}

impl Doc {
    impl_squash_text_mtext_em!(DocItem);
}

impl PruneIncodoc for Doc {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        self.items.retain(|i| !matches!(i, DocItem::Code(Err(_))));
        for item in &mut self.items {
            item.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        self.tags.prune_contentless();
        self.props.prune_contentless();
        for item in &mut self.items {
            item.prune_contentless();
        }
        self.items.retain(|item| !item.is_contentless());
    }

    fn is_contentless(&self) -> bool {
        self.items.is_empty()
    }
}

impl DocItem {
    pub fn squash(&mut self) {
        match self {
            DocItem::Link(link) => link.squash(),
            DocItem::List(list) => list.squash(),
            DocItem::Paragraph(par) => par.squash(),
            _ => { },
        }
    }
}

impl PruneIncodoc for DocItem {
    fn prune_errors(&mut self) {
        match self {
            DocItem::MText(mtext) => mtext.prune_errors(),
            DocItem::Em(em) => em.prune_errors(),
            DocItem::Code(Ok(code)) => code.prune_errors(),
            DocItem::Link(link) => link.prune_errors(),
            DocItem::Nav(nav) => nav.prune_errors(),
            DocItem::List(list) => list.prune_errors(),
            DocItem::Paragraph(par) => par.prune_errors(),
            DocItem::Section(section) => section.prune_errors(),
            _ => { },
        }
    }

    fn prune_contentless(&mut self) {
        match self {
            DocItem::Text(text) => text.prune_contentless(),
            DocItem::MText(mtext) => mtext.prune_contentless(),
            DocItem::Em(em) => em.prune_contentless(),
            DocItem::Code(Ok(code)) => code.prune_contentless(),
            DocItem::Code(Err(_)) => { },
            DocItem::Link(link) => link.prune_contentless(),
            DocItem::Nav(nav) => nav.prune_contentless(),
            DocItem::List(list) => list.prune_contentless(),
            DocItem::Paragraph(par) => par.prune_contentless(),
            DocItem::Section(section) => section.prune_contentless(),
        }
    }

    fn is_contentless(&self) -> bool {
        match self {
            DocItem::Text(text) => text.is_empty(),
            DocItem::MText(mtext) => mtext.is_contentless(),
            DocItem::Em(em) => em.is_contentless(),
            DocItem::Code(Ok(code)) => code.is_contentless(),
            DocItem::Code(Err(_)) => true,
            DocItem::Link(link) => link.is_contentless(),
            DocItem::Nav(nav) => nav.is_contentless(),
            DocItem::List(list) => list.is_contentless(),
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

impl Absorb for Tags {
    type Other = Option<Self>;
    fn absorb(&mut self, other: Self::Other) {
        if let Some(o) = other {
            for v in o {
                self.insert(v);
            }
        }
    }
}

impl PruneIncodoc for Tags {
    fn prune_errors(&mut self) { }

    fn prune_contentless(&mut self) {
        self.retain(|t|
            !t.is_empty() &&
            !t.chars().all(|c| c.is_whitespace())
        );
    }

    fn is_contentless(&self) -> bool {
        self.is_empty()
    }
}

impl Absorb for Props {
    type Other = Self;
    fn absorb(&mut self, other: Self::Other) {
        for prop in other {
            insert_prop(self, prop)
        }
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
            !k.is_empty() && !v.is_contentless() &&
            !k.chars().all(|c| c.is_whitespace())
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
            Self::String(string) => string.prune_contentless(),
            Self::Text(string) => string.prune_contentless(),
            _ => { },
        }
    }

    fn is_contentless(&self) -> bool {
        match self {
            Self::String(string) => string.is_empty(),
            Self::Text(string) => string.is_empty(),
            _ => false,
        }
    }
}

impl Section {
    pub fn squash(&mut self) {
        for item in &mut self.items {
            item.squash();
        }
    }
}

impl PruneIncodoc for Section {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        for item in &mut self.items {
            item.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        self.heading.prune_contentless();
        for item in &mut self.items {
            item.prune_contentless();
        }
        self.items.retain(|item| !item.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.heading.is_contentless() && self.items.is_empty()
    }
}

impl SectionItem {
    pub fn squash(&mut self) {
        match self {
            Self::Paragraph(par) => par.squash(),
            Self::Section(section) => section.squash(),
        }
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

impl Heading {
    impl_squash_text_em!(HeadingItem);
}

impl PruneIncodoc for Heading {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        for item in &mut self.items {
            item.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        for item in &mut self.items {
            item.prune_contentless();
        }
        self.items.retain(|item| !item.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.items.is_empty()
    }
}

impl PruneIncodoc for HeadingItem {
    fn prune_errors(&mut self) {
        if let Self::Em(em) = self {
            em.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        match self {
            Self::String(string) => string.prune_contentless(),
            Self::Em(em) => em.prune_contentless(),
        }
    }

    fn is_contentless(&self) -> bool {
        match self {
            Self::String(string) => string.is_empty(),
            Self::Em(em) => em.is_contentless(),
        }
    }
}

impl Paragraph {
    impl_squash_text_mtext_em!(ParagraphItem);
}

impl PruneIncodoc for Paragraph {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        for item in &mut self.items {
            item.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        for item in &mut self.items {
            item.prune_contentless();
        }
        self.items.retain(|item| !item.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.items.is_empty()
    }
}

impl ParagraphItem {
    pub fn squash(&mut self) {
        match self {
            Self::Link(link) => link.squash(),
            Self::List(list) => list.squash(),
            _ => { },
        }
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

impl List {
    pub fn squash(&mut self) {
        for item in &mut self.items {
            item.squash();
        }
    }
}

impl PruneIncodoc for List {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        for par in &mut self.items {
            par.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        for par in &mut self.items {
            par.prune_contentless();
        }
        self.items.retain(|par| !par.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.items.is_empty()
    }
}

impl PruneIncodoc for Nav {
    fn prune_errors(&mut self) {
        for snav in self {
            snav.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        for snav in self.iter_mut() {
            snav.prune_contentless();
        }
        self.retain(|snav| !snav.is_contentless());
    }

    fn is_contentless(&self) -> bool {
        self.is_empty() || self.iter().all(|s| s.is_contentless())
    }
}

impl PruneIncodoc for SNav {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        for link in &mut self.links {
            link.prune_errors();
        }
        for sub in &mut self.subs {
            sub.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        self.description.prune_contentless();
        for link in &mut self.links {
            link.prune_contentless();
        }
        self.links.retain(|link| !link.is_contentless());
        for sub in &mut self.subs {
            sub.prune_contentless();
        }
        self.subs.retain(|sub| !sub.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.subs.is_empty() && self.links.is_empty()
    }
}

impl Link {
    impl_squash_text_em!(LinkItem);
}

impl PruneIncodoc for Link {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        for item in &mut self.items {
            item.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        self.url.prune_contentless();
        for item in &mut self.items {
            item.prune_contentless();
        }
        self.items.retain(|item| !item.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.url.is_empty() && self.items.is_empty()
    }
}

impl PruneIncodoc for LinkItem {
    fn prune_errors(&mut self) {
        if let Self::Em(em) = self {
            em.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        match self {
            Self::String(string) => string.prune_contentless(),
            Self::Em(em) => em.prune_contentless(),
        }
    }

    fn is_contentless(&self) -> bool {
        match self {
            Self::String(string) => string.is_empty(),
            Self::Em(em) => em.is_contentless(),
        }
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

