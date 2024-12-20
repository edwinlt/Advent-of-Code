use std::collections::HashMap;

pub struct Trie {
    is_str_end: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Self {
        Self{is_str_end: false, children: HashMap::new()}
    }

    pub fn insert(&mut self, s: &str) {
        let mut node = self;
        for c in s.chars() {
            node = node.children.entry(c)
                       .or_insert_with(Self::new);
        }
        node.is_str_end = true;
    }

    #[allow(dead_code)]
    pub fn contains(&self, s: &str) -> bool {
        let mut node = self;
        for c in s.chars() {
            let Some(next) = node.get(c) else {
                return false;
            };
            node = next;
        }
        node.is_str_end
    }

    pub fn get(&self, c: char) -> Option<&Trie> {
        self.children.get(&c)
    }

    pub fn is_end(&self) -> bool {
        self.is_str_end
    }
}

impl<'a> FromIterator<&'a str> for Trie {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut trie = Self::new();
        for s in iter {
            trie.insert(s);
        }
        trie
    }
}
