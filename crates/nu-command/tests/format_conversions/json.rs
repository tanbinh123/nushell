use nu_test_support::fs::Stub::FileWithContentToBeTrimmed;
use nu_test_support::playground::Playground;
use nu_test_support::{nu, pipeline};

#[test]
fn table_to_json_text_and_from_json_text_back_into_table() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            open sgml_description.json
            | to json
            | from json
            | get glossary.GlossDiv.GlossList.GlossEntry.GlossSee
        "#
    ));

    assert_eq!(actual.out, "markup");
}

#[test]
fn from_json_text_to_table() {
    Playground::setup("filter_from_json_test_1", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "katz.txt",
            r#"
                {
                    "katz": [
                        {"name":   "Yehuda", "rusty_luck": 1},
                        {"name": "Jonathan", "rusty_luck": 1},
                        {"name":   "Andres", "rusty_luck": 1},
                        {"name":"GorbyPuff", "rusty_luck": 1}
                    ]
                }
            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(),
            "open katz.txt | from json | get katz | get rusty_luck | length "
        );

        assert_eq!(actual.out, "4");
    })
}

#[test]
fn from_json_text_recognizing_objects_independently_to_table() {
    Playground::setup("filter_from_json_test_2", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "katz.txt",
            r#"
                {"name":   "Yehuda", "rusty_luck": 1}
                {"name": "Jonathan", "rusty_luck": 1}
                {"name":   "Andres", "rusty_luck": 1}
                {"name":"GorbyPuff", "rusty_luck": 3}
            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                open katz.txt
                | from json -o
                | where name == "GorbyPuff"
                | get rusty_luck.0
            "#
        ));

        assert_eq!(actual.out, "3");
    })
}

#[test]
fn table_to_json_text() {
    Playground::setup("filter_to_json_test", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "sample.txt",
            r#"
                JonAndrehudaTZ,3
                GorbyPuff,100
            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                open sample.txt
                | lines
                | split column "," name luck
                | select name
                | to json
                | from json
                | get 0
                | get name
            "#
        ));

        assert_eq!(actual.out, "JonAndrehudaTZ");
    })
}

#[test]
fn top_level_values_from_json() {
    for (value, type_name) in [("null", "nothing"), ("true", "bool"), ("false", "bool")] {
        let actual = nu!(r#""{}" | from json | to json"#, value);
        assert_eq!(actual.out, value);
        let actual = nu!(r#""{}" | from json | describe"#, value);
        assert_eq!(actual.out, type_name);
    }
}

#[test]
fn ranges_to_json_as_array() {
    let value = r#"[  1,  2,  3]"#;
    let actual = nu!(r#"1..3 | to json"#);
    assert_eq!(actual.out, value);
}

#[test]
fn unbounded_from_in_range_fails() {
    let actual = nu!(r#"1.. | to json"#);
    assert!(actual.err.contains("Cannot create range"));
}

#[test]
fn inf_in_range_fails() {
    let actual = nu!(r#"inf..5 | to json"#);
    assert!(actual.err.contains("Cannot create range"));
    let actual = nu!(r#"5..inf | to json"#);
    assert!(actual.err.contains("Cannot create range"));
    let actual = nu!(r#"-inf..inf | to json"#);
    assert!(actual.err.contains("Cannot create range"));
}
