use String;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)] // make this printable with fmt::println
pub enum JSONValue {
    Value(String),
    Object(JSONObject),
}

pub type JSONObject = HashMap<String, JSONValue>;

// value: "hello"
// obj: {\n\t"key": val\n}\n
fn print_json_object(val_obj: &JSONObject) -> String {
    let mut aux = String::new();
    let mut i = 0;
    for (key, value) in val_obj {
        aux += format!("{{\n\t\"{}\": {}\n}}", key, value).as_str();
        if i < val_obj.len()-1 {
            aux += ",";
        }
        i += 1;
    }
    return aux.to_string();
}

impl fmt::Display for JSONValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JSONValue::Value(string) => write!(f, "\"{}\"", string),
            JSONValue::Object(val_obj) => write!(f, "{}",  print_json_object(val_obj)),
        }
    }
}

pub fn parse(file_contents: String) -> JSONObject {
    return consume_object(file_contents);
}

// fn consume_object(json_contents: String) -> JSONObject {
fn consume_object(_: String) -> JSONObject {
    let mut obj = JSONObject::new();
    obj.insert(String::from("key1"), JSONValue::Value(String::from("aaa")));

    let mut obj2 = JSONObject::new();
    obj2.insert(String::from("key2"), JSONValue::Value(String::from("aaa")));

    obj.insert(String::from("key3"), JSONValue::Object(obj2));
    obj.insert(String::from("key4"), JSONValue::Value(String::from("bbb")));
    obj.insert(String::from("key5"), JSONValue::Value(String::from("bbb")));

    return obj;
}
