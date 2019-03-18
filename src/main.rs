/// This function reurns middle names of a full  name
fn middle_name(full_name: &str) -> &str {

    full_name.split_whitespace().nth(1).unwrap()
}
/// This function prints the middle name of pawan
fn main() {
    let result: &str;

        let name: String = String::from("Pawan Singh Bhist");
        result = middle_name(&name);

    println!("{}",result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn middle_name_isvalid(){
        assert_eq!("kumar",middle_name("Alok kumar jha"))
    }
    #[test]
    #[should_panic]
    fn name_not_valid() {
        middle_name("Pawan");
    }

    #[test]
    fn middle_name_is_not_valid(){
        assert_ne!("singh",middle_name("Alok kumar jha"))
    }
}