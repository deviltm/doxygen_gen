use regex::Regex;
pub fn name_regex() -> Regex {
    Regex::new(r"typedef\s+(enum|struct)\s+(\w+)\s*\{").unwrap()
}

#[test]
fn test_struct_name_weird_whitespaces() {
    let input = "           typedef            struct      employee_struct              {      ";
    let re = name_regex();
    assert_eq!(re.is_match(input),true);
    let captures = re.captures(input).unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "struct");
    assert_eq!(captures.get(2).unwrap().as_str(), "employee_struct");
}
#[test]
fn test_struct_name() {
    let input = "typedef struct employee_struct {";
    let re = name_regex();
    assert_eq!(re.is_match(input),true);
    let captures = re.captures(input).unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "struct");
    assert_eq!(captures.get(2).unwrap().as_str(), "employee_struct");
}

#[test]
fn test_enum_name() {
    let input = "typedef enum employee_struct {";
    let re = name_regex();
    assert_eq!(re.is_match(input),true);
    let captures = re.captures(input).unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "enum");
    assert_eq!(captures.get(2).unwrap().as_str(), "employee_struct");
}
