use regex::Regex;
pub fn name_regex() -> Regex {
    Regex::new(r"typedef\s+(enum|struct)\s+(\w+)\s*\{").unwrap()
}
pub fn field_regex() -> Regex {
    Regex::new(r"\s*(.+?);?\s*//!<\s*(.+?)$").unwrap()
}

#[test]
fn test_struct_name_weird_whitespaces() {
    let input = "           typedef            struct      employee_struct              {      ";
    let re = name_regex();
    assert_eq!(re.is_match(input), true);
    let captures = re.captures(input).unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "struct");
    assert_eq!(captures.get(2).unwrap().as_str(), "employee_struct");
}
#[test]
fn test_struct_name() {
    let input = "typedef struct employee_struct {";
    let re = name_regex();
    assert_eq!(re.is_match(input), true);
    let captures = re.captures(input).unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "struct");
    assert_eq!(captures.get(2).unwrap().as_str(), "employee_struct");
}

#[test]
fn test_enum_name() {
    let input = "typedef enum employee_struct {";
    let re = name_regex();
    assert_eq!(re.is_match(input), true);
    let captures = re.captures(input).unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "enum");
    assert_eq!(captures.get(2).unwrap().as_str(), "employee_struct");
}

#[test] fn test_field_enum(){
    let input = "TEST_test = 1, //!< description description";
    let re = field_regex();
    assert_eq!(re.is_match(input), true);
    let captures = re.captures(input).unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "TEST_test = 1,");
    assert_eq!(captures.get(2).unwrap().as_str(), "description description");
}

#[test]
fn test_field_enum_basic(){
    let input = "TEST, //!< description";
    let re = field_regex();
    assert_eq!(re.is_match(input), true);
    let captures = re.captures(input).unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "TEST,");
    assert_eq!(captures.get(2).unwrap().as_str(), "description");
}

#[test] fn test_field_struct(){
    let input = "test_test Test : 6; //!< description description";
    let re = field_regex();
    assert_eq!(re.is_match(input), true);
    let captures = re.captures(input).unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "test_test Test : 6");
    assert_eq!(captures.get(2).unwrap().as_str(), "description description");
}

#[test]
fn test_field_enum_struct(){
    let input = "int TEST; //!< description";
    let re = field_regex();
    assert_eq!(re.is_match(input), true);
    let captures = re.captures(input).unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "int TEST");
    assert_eq!(captures.get(2).unwrap().as_str(), "description");
}
