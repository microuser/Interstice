
//#extern crate serde;
//#[macro_use]
//#extern crate serde_derive;
//#extern crate serde_json;

pub mod receptacle;


fn get_filename_of_first_argument<'a>() -> String {
    std::env::args().nth(1).expect("First agument must be a filename")
}    

fn get_content_of_first_argument<'a>() -> String{
    std::fs::read_to_string(
        get_filename_of_first_argument()
        ).expect("first argument must be a path that exists with valid JSON")
}

fn main() {
    
    let file_content = get_content_of_first_argument();
    println!("{}", file_content);

}
