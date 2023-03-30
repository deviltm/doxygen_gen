use regex::Regex;
pub fn name_regex() -> Regex {
    Regex::new(r"^typedef\s+(enum|struct)\s+(\w)\s*\{").unwrap()
}

#[test]
fn test_struct_name() {
    let input = "typedef struct Employee {";
    let re = name_regex();
    let captures = re.captures(input).unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "struct");
    assert_eq!(captures.get(2).unwrap().as_str(), "Employee");
}

#[test]
fn test_enum_name() {
    let input = "typedef enum Employee {";
    let re = name_regex();
    let captures = re.captures(input).unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "enum");
    assert_eq!(captures.get(2).unwrap().as_str(), "Employee");
}
