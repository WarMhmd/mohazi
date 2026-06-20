mod ast;
mod parser;

fn main() {
    let path = "./crates/core_lib/src/tests/testing.mhz";
    let file_content = std::fs::read_to_string(path).unwrap();

    match parser::parse_mhz(&file_content) {
        Ok(v) => {
            println!("{:#?}", v)
        }
        Err(e) => {
            println!("errors vector: {:?}", e);
        }
    }
}
