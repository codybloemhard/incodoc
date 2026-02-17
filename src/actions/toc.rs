use crate::*;

/// A recursive table of contents.
#[derive(Clone, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TableOfContentsItem {
    /// Title of this item.
    pub title: String,
    /// Link to the items destination in the document.
    pub link: String,
    /// What type of content this item refers to.
    pub item_type: TableOfContentsItemType,
    /// Sub items in this table.
    pub children: Vec<TableOfContentsItem>,
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

pub trait InsertTableOfContentsSectionIDs {
    fn insert_table_of_contents_section_ids(&mut self);
}

fn push_toci(children: &mut Vec<TableOfContentsItem>, res: Option<TableOfContentsItem>) {
    if let Some(item) = res {
        children.push(item);
    }
}

fn heading_title_and_id(heading: &Heading, title: &mut String, id: &mut String, id_is_link: bool) {
    if id_is_link {
        id.push('#');
    }
    for item in &heading.items {
        match item {
            HeadingItem::String(string) => {
                title.push_str(string);
                id.push_str(&string.to_lowercase().replace(" ", "-"));
            },
            HeadingItem::Em(em) => {
                title.push_str(&em.text);
                id.push_str(&em.text.to_lowercase().replace(" ", "-"));
            },
        }
    }
}

fn id_to_link(id: &str) -> String {
    let mut res = String::from("#");
    res.push_str(id);
    res
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
        if let Some((filter, ftype)) = filter
            && !filter.contains(&item_type)
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
            && !filter.contains(&item_type)
            && *ftype == TableOfContentsFilterType::IncludeWithChildren
        {
            return None;
        }
        heading_title_and_id(&self.heading, &mut title, &mut link, true);
        if let Some(PropVal::String(id)) = self.props.get("id") {
            link = id_to_link(id);
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
            && !filter.contains(&TableOfContentsItemType::Paragraph)
            && *ftype == TableOfContentsFilterType::IncludeWithChildren
        {
            return None;
        }
        if let Some(PropVal::String(id)) = self.props.get("id") {
            Some(TableOfContentsItem {
                title: id.to_string(),
                link: id_to_link(id),
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
        if let Some((filter, _)) = filter
            && !filter.contains(&TableOfContentsItemType::Emphasis)
        {
            return None;
        }
        if let Some(PropVal::String(id)) = self.props.get("id") {
            Some(TableOfContentsItem {
                title: self.text.to_string(),
                link: id_to_link(id),
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
        let mut children = Vec::new();
        for par in &self.items {
            push_toci(&mut children, par.get_table_of_contents(filter));
        }
        if children.is_empty()
            && let Some((filter, ftype)) = filter
            && !filter.contains(&TableOfContentsItemType::List)
            && *ftype == TableOfContentsFilterType::IncludeWithChildren
        {
            return None;
        }
        if let Some(PropVal::String(id)) = self.props.get("id") {
            Some(TableOfContentsItem {
                title: id.to_string(),
                link: id_to_link(id),
                item_type: TableOfContentsItemType::List,
                children,
            })
        } else if !children.is_empty() {
            Some(TableOfContentsItem {
                title: "list".to_string(),
                link: "".to_string(),
                item_type: TableOfContentsItemType::List,
                children,
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
        if let Some((filter, _)) = filter && !filter.contains(&TableOfContentsItemType::Nav) {
            return None;
        }
        if let Some(PropVal::String(id)) = self.props.get("id") {
            Some(TableOfContentsItem {
                title: self.description.to_string(),
                link: id_to_link(id),
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
        if let Some((filter, _)) = filter && !filter.contains(&TableOfContentsItemType::Link) {
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
                link: id_to_link(id),
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
        if let Some((filter, _)) = filter
            && !filter.contains(&TableOfContentsItemType::CodeBlock)
        {
            return None;
        }
        if let Ok(code_block) = self {
            if let Some(PropVal::String(id)) = code_block.props.get("id") {
                Some(TableOfContentsItem {
                    title: id.to_string(),
                    link: id_to_link(id),
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
        let mut children = Vec::new();
        for row in &self.rows {
            for par in &row.items {
                push_toci(&mut children, par.get_table_of_contents(filter));
            }
        }
        if children.is_empty()
            && let Some((filter, ftype)) = filter
            && !filter.contains(&TableOfContentsItemType::Table)
            && *ftype == TableOfContentsFilterType::IncludeWithChildren
        {
            return None;
        }
        if let Some(PropVal::String(id)) = self.props.get("id") {
            Some(TableOfContentsItem {
                title: id.to_string(),
                link: id_to_link(id),
                item_type: TableOfContentsItemType::Table,
                children,
            })
        } else if !children.is_empty() {
            Some(TableOfContentsItem {
                title: "table".to_string(),
                link: "".to_string(),
                item_type: TableOfContentsItemType::Table,
                children,
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
        if let Some((filter, _)) = filter && !filter.contains(&TableOfContentsItemType::MText) {
            return None;
        }
        if let Some(PropVal::String(id)) = self.props.get("id") {
            Some(TableOfContentsItem {
                title: id.to_string(),
                link: id_to_link(id),
                item_type: TableOfContentsItemType::MText,
                children: vec![],
            })
        } else {
            None
        }
    }
}

impl InsertTableOfContentsSectionIDs for Doc {
    fn insert_table_of_contents_section_ids(&mut self) {
        for item in &mut self.items {
            if let DocItem::Section(section) = item {
                section.insert_table_of_contents_section_ids();
            }
        }
    }
}

impl InsertTableOfContentsSectionIDs for Section {
    fn insert_table_of_contents_section_ids(&mut self) {
        let (mut title, mut link) = (String::new(), String::new());
        if !self.props.contains_key("id") {
            heading_title_and_id(&self.heading, &mut title, &mut link, false);
            self.props.insert("id".to_string(), PropVal::String(link));
        }
        for item in &mut self.items {
            if let SectionItem::Section(section) = item {
                section.insert_table_of_contents_section_ids();
            }
        }
    }
}

