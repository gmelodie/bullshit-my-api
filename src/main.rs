mod parser;

use std::io::Read;

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
    let obj = parser::parse(file_string);
    println!("{}", parser::print_json(obj, 0).as_str());
}
