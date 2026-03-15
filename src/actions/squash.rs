use crate::*;
use crate::actions::prune::PruneIncodoc;

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
                    $enumname::Text(text) => {
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
    pub fn squash(&mut self) {
        for item in &mut self.items {
            item.squash();
        }
    }
}

impl DocItem {
    pub fn squash(&mut self) {
        match self {
            Self::Paragraph(par) => par.squash(),
            Self::Section(section) => section.squash(),
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

impl SectionItem {
    pub fn squash(&mut self) {
        match self {
            Self::Paragraph(par) => par.squash(),
            Self::Section(section) => section.squash(),
        }
    }
}

impl Heading {
    impl_squash_text_em!(EmOrText);
}

impl Paragraph {
    impl_squash_text_mtext_em!(ParagraphItem);
}

impl ParagraphItem {
    pub fn squash(&mut self) {
        match self {
            Self::Link(link) => link.squash(),
            Self::List(list) => list.squash(),
            Self::Table(table) => table.squash(),
            _ => { },
        }
    }
}

impl List {
    pub fn squash(&mut self) {
        for item in &mut self.items {
            item.squash();
        }
    }
}

impl Table {
    pub fn squash(&mut self) {
        for row in &mut self.rows {
            row.squash();
        }
    }
}

impl TableRow {
    pub fn squash(&mut self) {
        for item in &mut self.items {
            item.squash();
        }
    }
}

impl Link {
    impl_squash_text_em!(EmOrText);
}

