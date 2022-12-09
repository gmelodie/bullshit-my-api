use std::io::Read;

fn main() {
    let myfile = std::fs::File::open("./myfile");

    let file_string = match myfile {
        Ok(mut file_contents) => {
            let mut file_string = String::new();
            file_contents.read_to_string(&mut file_string).unwrap();
            file_string
        }
        Err(error) => {
            println!("Hoorray here's an error {}", error);
            return;
        }
    };
    print!("File opened!\n");
    print!("File: {}", file_string);
}
