mod vis_to_yaml;

fn main() {
    let rules = "    min_length: 5\n    error: \"Password must be at least 5 characters long\"\n    max_length: 10\n    error: \"Password must be at most 10 characters long\"";
    let result = vis_to_yaml::handle_custom_rule_format(rules.to_string());
    println!("{}", result);
}
