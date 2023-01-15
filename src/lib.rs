use std::{cell::RefCell, rc::Rc};

use html_escape::encode_text;

enum TagValue {
    Value(String),
    TagChildren(Vec<Rc<RefCell<Html>>>),
}

pub struct Html {
    tag_type: String,
    attr: Option<String>,
    value: Option<TagValue>,
}

#[derive(Clone)]
pub struct Tag(Rc<RefCell<Html>>);

impl Html {
    pub fn new(tag_type: &str) -> Self {
        Self {
            tag_type: tag_type.to_string(),
            value: None,
            attr: None,
        }
    }

    pub fn build(self) -> String {
        let mut s = String::new();

        s.push('<');
        s.push_str(&self.tag_type);
        s.push('>');

        if let Some(v) = &self.value {
            match v {
                TagValue::Value(value) => s.push_str(value),
                TagValue::TagChildren(children) => {
                    for c in children {
                        c.finish(&mut s);
                    }
                }
            }
        }

        s.push_str("</");
        s.push_str(&self.tag_type);
        s.push('>');

        s
    }
}

trait Finish {
    fn finish(&self, s: &mut String);
}

impl Finish for Rc<RefCell<Html>> {
    fn finish(&self, s: &mut String) {
        let tag_type = &self.borrow().tag_type;

        s.push('<');
        s.push_str(tag_type);
        s.push('>');

        if let Some(v) = &self.borrow().value {
            match v {
                TagValue::Value(value) => s.push_str(value),
                TagValue::TagChildren(children) => {
                    for c in children {
                        c.finish(s);
                    }
                }
            }
        }

        s.push_str("</");
        s.push_str(tag_type);
        s.push('>');
    }
}

impl Html {
    pub fn add_child<TT>(&mut self, tag_type: TT) -> Tag
    where
        TT: AsRef<str>,
    {
        let n = Rc::new(RefCell::new(Html::new(tag_type.as_ref())));

        if let Some(TagValue::TagChildren(children)) = &mut self.value {
            children.push(n.clone());
        } else {
            self.value = Some(TagValue::TagChildren(vec![n.clone()]))
        }

        Tag(n)
    }

    pub fn add_value<V>(&mut self, value: V)
    where
        V: AsRef<str>,
    {
        let value = encode_text(&value);
        self.value = Some(TagValue::Value(value.to_string()));
    }

    pub fn add_attribute(&mut self, attr: &str) {
        self.attr = Some(attr.to_string());
    }
}

impl Tag {
    pub fn add_child<TT>(&self, tag_type: TT) -> Tag
    where
        TT: AsRef<str>,
    {
        let n = Rc::new(RefCell::new(Html::new(tag_type.as_ref())));

        let mut m = self.0.borrow_mut();
        if let Some(TagValue::TagChildren(children)) = &mut m.value {
            children.push(n.clone());
        } else {
            m.value = Some(TagValue::TagChildren(vec![n.clone()]))
        }

        Tag(n)
    }

    pub fn add_value<V>(&self, value: V)
    where
        V: AsRef<str>,
    {
        let value = encode_text(&value);
        self.0.borrow_mut().value = Some(TagValue::Value(value.to_string()));
    }

    pub fn add_attribute(&self, attr: &str) {
        self.0.borrow_mut().attr = Some(attr.to_string());
    }
}
