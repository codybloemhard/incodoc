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

/// A recursive table of contents.
#[derive(Clone, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TableOfContentsItem {
    /// Title of this item.
    title: String,
    /// Link to the items destination in the document.
    link: String,
    /// What type of content this item refers to.
    item_type: TableOfContentsItemType,
    /// Sub items in this table.
    children: Vec<TableOfContentsItem>,
}

/// Describes the type of content an item in a table of contents refers to.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TableOfContentsItemType {
    Document,
    Section,
    Paragraph,
    Nav,
    Quote,
    FootnoteDefinition,
    List,
    Table,
    CodeBlock,
    Link,
    Emphasis,
    MText,
}

/// Defines the behaviour of the filter when generating a table of contents.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TableOfContentsFilterType {
    /// Stop when a type is not in the filter, don't look any further.
    HardStop,
    /// Include a vertex when its children need including, even when its type is absent from the
    /// filter.
    IncludeWithChildren,
}

/// Generate a table of contents from a part of a document.
pub trait GetTableOfContents {
    /// If a filter is supplied, an item must be of a type present in the filter to get included.
    fn get_table_of_contents(
        &self,
        filter: &Option<(HashSet<TableOfContentsItemType>, TableOfContentsFilterType)>,
    ) -> Option<TableOfContentsItem>;
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
    pub fn squash(&mut self) {
        for item in &mut self.items {
            item.squash();
        }
    }
}

impl PruneIncodoc for Doc {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
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
        if let DocItem::Paragraph(par) = self {
            par.squash();
        }
    }
}

impl PruneIncodoc for DocItem {
    fn prune_errors(&mut self) {
        match self {
            DocItem::Nav(nav) => nav.prune_errors(),
            DocItem::Paragraph(par) => par.prune_errors(),
            DocItem::Section(section) => section.prune_errors(),
        }
    }

    fn prune_contentless(&mut self) {
        match self {
            DocItem::Nav(nav) => nav.prune_contentless(),
            DocItem::Paragraph(par) => par.prune_contentless(),
            DocItem::Section(section) => section.prune_contentless(),
        }
    }

    fn is_contentless(&self) -> bool {
        match self {
            DocItem::Nav(nav) => nav.is_contentless(),
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
            !t.is_empty() && !t.chars().all(char::is_whitespace)
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
            insert_prop(self, prop);
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

impl PruneIncodoc for Table {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        for row in &mut self.rows {
            row.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        for row in &mut self.rows {
            row.prune_contentless();
        }
        self.rows.retain(|row| !row.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn is_contentless(&self) -> bool {
        self.rows.is_empty()
    }
}

impl PruneIncodoc for TableRow {
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

fn push_toci(children: &mut Vec<TableOfContentsItem>, res: Option<TableOfContentsItem>) {
    if let Some(item) = res {
        children.push(item);
    }
}

impl GetTableOfContents for Doc {
    fn get_table_of_contents(
        &self,
        filter: &Option<(HashSet<TableOfContentsItemType>, TableOfContentsFilterType)>,
    ) -> Option<TableOfContentsItem> {
        if let Some((filter, ftype)) = filter
            && !filter.contains(&TableOfContentsItemType::Document)
            && *ftype == TableOfContentsFilterType::HardStop
        {
            return None;
        }
        let mut children = Vec::new();
        for item in &self.items {
            match item {
                DocItem::Nav(nav) => push_toci(&mut children, nav.get_table_of_contents(filter)),
                DocItem::Paragraph(par) => push_toci(
                    &mut children,
                    par.get_table_of_contents(filter)
                ),
                DocItem::Section(section) => push_toci(
                    &mut children,
                    section.get_table_of_contents(filter)
                ),
            }
        }
        if children.is_empty()
            && let Some((filter, ftype)) = filter
            && !filter.contains(&TableOfContentsItemType::Document)
            && *ftype == TableOfContentsFilterType::IncludeWithChildren
        {
            return None;
        }
        Some(TableOfContentsItem {
            title: "Table of Contents".to_string(),
            link: ".".to_string(),
            item_type: TableOfContentsItemType::Document,
            children,
        })
    }
}

impl GetTableOfContents for Section {
    fn get_table_of_contents(
        &self,
        filter: &Option<(HashSet<TableOfContentsItemType>, TableOfContentsFilterType)>,
    ) -> Option<TableOfContentsItem> {
        if let Some((filter, ftype)) = filter
            && !filter.contains(&TableOfContentsItemType::Section)
            && *ftype == TableOfContentsFilterType::HardStop
        {
            return None;
        }
        let mut children = Vec::new();
        for item in &self.items {
            match item {
                SectionItem::Paragraph(par) => push_toci(
                    &mut children,
                    par.get_table_of_contents(filter)
                ),
                SectionItem::Section(section) => push_toci(
                    &mut children,
                    section.get_table_of_contents(filter)
                ),
            }
        }
        if children.is_empty()
            && let Some((filter, ftype)) = filter
            && !filter.contains(&TableOfContentsItemType::Section)
            && *ftype == TableOfContentsFilterType::IncludeWithChildren
        {
            return None;
        }
        let mut title = String::new();
        let mut link = String::new();
        let item_type = if self.tags.contains("footnote-def") {
            title += "Footnote definition: ";
            TableOfContentsItemType::FootnoteDefinition
        } else if self.tags.contains("blockquote") || self.tags.contains("blockquote-typed") {
            title += "Quote: ";
            TableOfContentsItemType::Quote
        } else {
            TableOfContentsItemType::Section
        };
        for item in &self.heading.items {
            match item {
                HeadingItem::String(string) => {
                    title += string;
                    link += &string.to_lowercase().replace(" ", "-");
                },
                HeadingItem::Em(em) => {
                    title += &em.text;
                    link += &em.text.to_lowercase().replace(" ", "-");
                },
            }
        }
        if title.ends_with(": ") {
            title.pop();
            title.pop();
        }
        Some(TableOfContentsItem {
            title,
            link,
            item_type,
            children,
        })
    }
}

impl GetTableOfContents for Paragraph {
    fn get_table_of_contents(
        &self,
        filter: &Option<(HashSet<TableOfContentsItemType>, TableOfContentsFilterType)>,
    ) -> Option<TableOfContentsItem> {
        if let Some((filter, ftype)) = filter
            && !filter.contains(&TableOfContentsItemType::Paragraph)
            && *ftype == TableOfContentsFilterType::HardStop
        {
            return None;
        }
        let mut children = Vec::new();
        for item in &self.items {
            match item {
                ParagraphItem::Text(_) => { },
                ParagraphItem::MText(mtext) => push_toci(
                    &mut children,
                    mtext.get_table_of_contents(filter)
                ),
                ParagraphItem::Em(em) => push_toci(&mut children, em.get_table_of_contents(filter)),
                ParagraphItem::Code(code_result) => push_toci(
                    &mut children,
                    code_result.get_table_of_contents(filter)
                ),
                ParagraphItem::Link(link) => push_toci(
                    &mut children,
                    link.get_table_of_contents(filter)
                ),
                ParagraphItem::List(list) => push_toci(
                    &mut children,
                    list.get_table_of_contents(filter)
                ),
                ParagraphItem::Table(table) => push_toci(
                    &mut children,
                    table.get_table_of_contents(filter)
                ),
            }
        }
        if children.is_empty()
            && let Some((filter, ftype)) = filter
            && !filter.contains(&TableOfContentsItemType::Section)
            && *ftype == TableOfContentsFilterType::IncludeWithChildren
        {
            return None;
        }
        if let Some(PropVal::String(id)) = self.props.get("id") {
            Some(TableOfContentsItem {
                title: id.to_string(),
                link: id.to_string(),
                item_type: TableOfContentsItemType::Paragraph,
                children,
            })
        } else if !children.is_empty() {
            Some(TableOfContentsItem {
                title: "paragraph".to_string(),
                link: "".to_string(),
                item_type: TableOfContentsItemType::Paragraph,
                children,
            })
        } else {
            None
        }
    }
}

impl GetTableOfContents for Emphasis {
    fn get_table_of_contents(
        &self,
        filter: &Option<(HashSet<TableOfContentsItemType>, TableOfContentsFilterType)>,
    ) -> Option<TableOfContentsItem> {
        if let Some((filter, ftype)) = filter
            && !filter.contains(&TableOfContentsItemType::Emphasis)
            && *ftype == TableOfContentsFilterType::HardStop
        {
            return None;
        }
        if let Some(PropVal::String(id)) = self.props.get("id") {
            Some(TableOfContentsItem {
                title: self.text.to_string(),
                link: id.to_string(),
                item_type: TableOfContentsItemType::Emphasis,
                children: vec![],
            })
        } else {
            None
        }
    }
}

impl GetTableOfContents for List {
    fn get_table_of_contents(
        &self,
        filter: &Option<(HashSet<TableOfContentsItemType>, TableOfContentsFilterType)>,
    ) -> Option<TableOfContentsItem> {
        if let Some((filter, ftype)) = filter
            && !filter.contains(&TableOfContentsItemType::List)
            && *ftype == TableOfContentsFilterType::HardStop
        {
            return None;
        }
        if let Some(PropVal::String(id)) = self.props.get("id") {
            Some(TableOfContentsItem {
                title: id.to_string(),
                link: id.to_string(),
                item_type: TableOfContentsItemType::List,
                children: vec![],
            })
        } else {
            None
        }
    }
}

impl GetTableOfContents for Nav {
    fn get_table_of_contents(
        &self,
        filter: &Option<(HashSet<TableOfContentsItemType>, TableOfContentsFilterType)>,
    ) -> Option<TableOfContentsItem> {
        if let Some((filter, ftype)) = filter
            && !filter.contains(&TableOfContentsItemType::Nav)
            && *ftype == TableOfContentsFilterType::HardStop
        {
            return None;
        }
        if let Some(PropVal::String(id)) = self.props.get("id") {
            Some(TableOfContentsItem {
                title: self.description.to_string(),
                link: id.to_string(),
                item_type: TableOfContentsItemType::Nav,
                children: vec![],
            })
        } else {
            None
        }
    }
}

impl GetTableOfContents for Link {
    fn get_table_of_contents(
        &self,
        filter: &Option<(HashSet<TableOfContentsItemType>, TableOfContentsFilterType)>,
    ) -> Option<TableOfContentsItem> {
        if let Some((filter, ftype)) = filter
            && !filter.contains(&TableOfContentsItemType::Link)
            && *ftype == TableOfContentsFilterType::HardStop
        {
            return None;
        }
        if let Some(PropVal::String(id)) = self.props.get("id") {
            let mut title = String::new();
            for item in &self.items {
                match item {
                    LinkItem::String(string) => title += string,
                    LinkItem::Em(em) => title += &em.text,
                }
            }
            Some(TableOfContentsItem {
                title,
                link: id.to_string(),
                item_type: TableOfContentsItemType::Link,
                children: vec![],
            })
        } else {
            None
        }
    }
}

impl GetTableOfContents for Result<CodeBlock, CodeIdentError> {
    fn get_table_of_contents(
        &self,
        filter: &Option<(HashSet<TableOfContentsItemType>, TableOfContentsFilterType)>,
    ) -> Option<TableOfContentsItem> {
        if let Some((filter, ftype)) = filter
            && !filter.contains(&TableOfContentsItemType::CodeBlock)
            && *ftype == TableOfContentsFilterType::HardStop
        {
            return None;
        }
        if let Ok(code_block) = self {
            if let Some(PropVal::String(id)) = code_block.props.get("id") {
                Some(TableOfContentsItem {
                    title: id.to_string(),
                    link: id.to_string(),
                    item_type: TableOfContentsItemType::CodeBlock,
                    children: vec![],
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl GetTableOfContents for Table {
    fn get_table_of_contents(
        &self,
        filter: &Option<(HashSet<TableOfContentsItemType>, TableOfContentsFilterType)>,
    ) -> Option<TableOfContentsItem> {
        if let Some((filter, ftype)) = filter
            && !filter.contains(&TableOfContentsItemType::Table)
            && *ftype == TableOfContentsFilterType::HardStop
        {
            return None;
        }
        if let Some(PropVal::String(id)) = self.props.get("id") {
            Some(TableOfContentsItem {
                title: id.to_string(),
                link: id.to_string(),
                item_type: TableOfContentsItemType::Table,
                children: vec![],
            })
        } else {
            None
        }
    }
}

impl GetTableOfContents for TextWithMeta {
    fn get_table_of_contents(
        &self,
        filter: &Option<(HashSet<TableOfContentsItemType>, TableOfContentsFilterType)>,
    ) -> Option<TableOfContentsItem> {
        if let Some((filter, ftype)) = filter
            && !filter.contains(&TableOfContentsItemType::MText)
            && *ftype == TableOfContentsFilterType::HardStop
        {
            return None;
        }
        if let Some(PropVal::String(id)) = self.props.get("id") {
            Some(TableOfContentsItem {
                title: id.to_string(),
                link: id.to_string(),
                item_type: TableOfContentsItemType::MText,
                children: vec![],
            })
        } else {
            None
        }
    }
}

