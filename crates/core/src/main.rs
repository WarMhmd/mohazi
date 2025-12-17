mod ast;
mod vis_to_yaml;

fn main() {
    let path = "/home/alhareth/projects/mohazi/crates/core/src/tests/test_grad.vis";
    let out_path = "/home/alhareth/projects/mohazi/crates/core/src/tests/test_output.yaml";
    let mut out_file = std::fs::File::create(out_path).unwrap();
    let file_content = std::fs::read_to_string(path).unwrap();

    match vis_to_yaml::parse_vis(&file_content) {
        Ok(v) => {
            println!("{:#?}", v)
        }
        Err(e) => {
            println!("errors vector: {:?}", e);
        }
    }
}
