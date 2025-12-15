mod tests;
mod vis_to_yaml;

fn main() {
    let input_path = "testinput.vis";
    vis_to_yaml::read_vis_file(input_path);
}
