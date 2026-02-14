mod ast;
mod vis_parser;

fn main() {
    let path = "./tests/testing.vis";
    let file_content = std::fs::read_to_string(path).unwrap();

    match vis_parser::parse_vis(&file_content) {
        Ok(v) => {
            println!("{:#?}", v)
        }
        Err(e) => {
            println!("errors vector: {:?}", e);
        }
    }
}
