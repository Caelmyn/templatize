use std::fs;

#[inline]
fn exp_path(path: &str) -> String {
    format!("tests/expected/{}.exp", path)
}

#[inline]
fn inp_path(path: &str) -> String {
    format!("tests/inputs/{}.json", path)
}

#[inline]
fn load_expected(path: &str) -> String {
    fs::read_to_string(exp_path(path)).expect("Failed to load expect file").trim().to_string()
}

/* ---------- */

#[test]
fn from_string() {
    static TEST_STR: &str = "{ \"int\": 1, \"str\": \"string\", \"bool\": false, \"null\": null }";
    let s = format!("{:?}", templatize::fields_from_str(TEST_STR).expect("Failed to generate fields"));
    let expected = load_expected("basic_field");

    assert_eq!(s, expected)
}

#[test]
fn from_file() {
    let s = format!("{:?}", templatize::fields_from_file(&inp_path("basic_file")).expect("Failed to generate fields"));
    let exp = load_expected("basic_field");

    assert_eq!(s, exp)
}

#[test]
fn from_struct() {
    #[derive(serde::Serialize)]
    struct Test {
        int: i8,
        str: &'static str,
        bool: bool,
        null: Option<()>
    }

    static TEST_OBJ: Test = Test {
        int: 1,
        str: "string",
        bool: false,
        null: None
    };

    let s = format!("{:?}", templatize::fields_from_struct(&TEST_OBJ).expect("Failed to generate fields"));
    let exp = load_expected("basic_field");

    assert_eq!(s, exp);
}

#[test]
fn field_with_object() {
    let s = format!("{:?}", templatize::fields_from_file(&inp_path("object_file")).expect("Failed to generate fields"));
    let exp = load_expected("object_fields");

    assert_eq!(s, exp);
}
