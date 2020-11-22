use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

#[derive(Debug)]
pub enum  Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data: HashMap<&'buf str, Value<'buf>> = HashMap::new();
        for sub_string in s.split('&') {
            let mut key = sub_string;
            let mut value = "";
            if let Some(i) = sub_string.find('=') {
                key = &sub_string[..i];
                value = &sub_string[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(val) => {
                        *existing = Value::Multiple(vec![val, value]);
                    }
                    Value::Multiple(vec) => vec.push(value)
                })
                .or_insert(Value::Single(value));
        }

        Self { data }
    }
}
