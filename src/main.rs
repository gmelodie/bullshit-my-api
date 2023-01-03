mod parser;

use std::io::Read;

fn print_top_level(obj: &parser::JSONObject) {
    let mut i = 0;
    let mut aux = String::from("{\n");
    for (key, value) in obj {
        aux += format!("\t{}: {}", key, value).as_str();
        if i < obj.len()-1 {
            aux += ",";
        }
        aux += "\n";
        i += 1;
    }
    aux += "}";
    println!("{}", aux);
}


fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let file = std::fs::File::open(path);

    let file_string = match file {
        Ok(mut file_contents) => {
            let mut file_string = String::new();
            file_contents.read_to_string(&mut file_string).unwrap();
            file_string
        }
        Err(error) => {
            println!("failed to open file: {}", error);
            return;
        }
    };
    print_top_level(&parser::parse(file_string));
}
