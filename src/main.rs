use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    let mut list = ItemList::new();
    list.add_items(vec![
        "abc".to_string(),
        "abc\nabc".to_string(),
        "123\nabc".to_string(),
        "def\nabc".to_string(),
        "xyz".to_string(),
    ]);
    // expected: five items total with four total unique attributes
    println!("{:#?}", list);
}

#[derive(Debug)]
struct TemplateLibrary {
    set: HashSet<AttributeTemplate>,
}

impl TemplateLibrary {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }

    pub fn insert(&mut self, template: AttributeTemplate) {
        self.set.insert(template);
    }

    pub fn get(&self, id: &str) -> &AttributeTemplate {
        self.set
            .get(&AttributeTemplate { id: id.to_string() })
            .unwrap()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct AttributeTemplate {
    id: String,
}

impl AttributeTemplate {
    pub fn parse(string: &str) -> Self {
        Self {
            id: string.to_string(),
        }
    }
}

#[derive(Debug)]
struct ItemList<'a> {
    list: Vec<Item<'a>>,
    library: TemplateLibrary,
}

impl<'a> ItemList<'a> {
    pub fn new() -> Self {
        Self {
            list: vec![],
            library: TemplateLibrary::new(),
        }
    }

    pub fn add_items(&mut self, item_strs: Vec<String>) {
        for item_str in item_strs.into_iter() {
            let item = Item::parse(item_str, &mut self.library);
            self.list.push(item);
        }
    }
}

#[derive(Debug)]
struct Item<'a> {
    attributes: Vec<Attribute<'a>>,
}

impl<'a> Item<'a> {
    pub fn parse(item_str: String, lib: &'a mut TemplateLibrary) -> Self {
        let mut attributes = vec![];
        for attr_str in item_str.split("\n") {
            let template = AttributeTemplate::parse(attr_str);
            lib.insert(template);
            let attribute = Attribute::new(lib.get(attr_str));
            attributes.push(attribute);
        }
        Self {
            attributes: attributes,
        }
    }
}

#[derive(Debug)]
struct Attribute<'a> {
    template: &'a AttributeTemplate,
}

impl<'a> Attribute<'a> {
    pub fn new(template: &'a AttributeTemplate) -> Self {
        Self { template: template }
    }
}
