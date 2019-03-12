
fn middle_name(full_name: &str) -> &str {
    full_name.split_whitespace().nth(1).unwrap()
}

fn main() {
    let result: &str;
    {
        let name: String = String::from("Pawan Singh Bisht");
        result = middle_name(&name);
    }
    assert_eq!(result, "Singh");
}