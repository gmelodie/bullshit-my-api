use String;
use std::collections::HashMap;

// =====
// object
// =====
// array
// =====
// value
// =====
// A JSON value MUST be an object, array, number, or string, or one of
// the following three literal names: false null true
//
//
// ======
// number
// ======
// whitespace
// ======
//
// ======
// string
// ======
//string = quotation-mark *char quotation-mark
//char = unescaped /
//       escape (
//           %x22 /          ; "    quotation mark  U+0022
//           %x5C /          ; \    reverse solidus U+005C
//           %x2F /          ; /    solidus         U+002F
//           %x62 /          ; b    backspace       U+0008
//           %x66 /          ; f    form feed       U+000C
//           %x6E /          ; n    line feed       U+000A
//           %x72 /          ; r    carriage return U+000D
//           %x74 /          ; t    tab             U+0009
//           %x75 4HEXDIG )  ; uXXXX                U+XXXX

//escape = %x5C              ; \

//quotation-mark = %x22      ; "

//unescaped = %x20-21 / %x23-5B / %x5D-10FFFF
//












#[derive(Debug)] // make this printable with fmt::println
pub enum JSONValue {
    Value(String),
    Object(JSONObject),
}

pub type JSONObject = HashMap<String, JSONValue>;

pub fn print_json(obj: JSONValue, indent: usize) -> String {
    match obj {
        JSONValue::Value(string) => format!("\"{}\",", string),
        JSONValue::Object(obj) => {
            let mut aux = String::new();
            let indent_str = "\t".repeat(indent);
            let indent_str1 = "\t".repeat(indent+1);
            aux += format!("{indent_str}{{\n").as_str();
            for (key, value) in obj {
                aux += format!("{indent_str1}\"{}\": {}\n", key, print_json(value, indent+1)).as_str();
            }
            aux += format!("{indent_str}}}\n").as_str();
            aux
        }
    }
}

pub fn parse(file_contents: String) -> JSONObject {
    // consume object, return
    return consume_object(file_contents);
}

fn consume_whitespace(_: String) -> String {
    // U+0009 (horizontal tab, '\t')
    // U+000A (line feed, '\n')
    // U+000D (carriage return, '\r')
    // U+0020 (space, ' ')
    return String::new();
}

fn consume_object(_: String) -> JSONObject {
    // .chars() iterator
    // .next()
    // .next_back()
    //consume {
    //consume whitespace
        //option 1
            //consume } (end)
        //option 2
            //consume string
            //consume whitespace
            //consume :
            //consume value
            //if , return to option 2 (consume another)
            //elif consume }
            //else error
        //option 3: error
    let mut obj = JSONObject::new();
    obj.insert(String::from("key1"), JSONValue::Value(String::from("aaa")));

    let mut obj2 = JSONObject::new();
    obj2.insert(String::from("key2"), JSONValue::Value(String::from("aaa")));

    obj.insert(String::from("key3"), JSONValue::Object(obj2));
    obj.insert(String::from("key4"), JSONValue::Value(String::from("bbb")));
    obj.insert(String::from("key5"), JSONValue::Value(String::from("bbb")));
    return obj;
}
