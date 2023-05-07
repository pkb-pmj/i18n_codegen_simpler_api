i18n_codegen::i18n!("tests/locales");

#[test]
fn it_works() {
    assert_eq!("Hello, World!", Locale::En.hello());
    assert_eq!("Hej, Verden!", Locale::Da.hello());
}

#[test]
fn it_works_with_interpolations() {
    assert_eq!("Hello Bob", Locale::En.greeting("Bob"));
    assert_eq!("Hej Bob", Locale::Da.greeting("Bob"));
}

#[test]
fn it_works_when_some_locales_are_missing_interpolations() {
    assert_eq!("en foo word", Locale::En.missing_interpolation_da("word"),);
    assert_eq!("da foo", Locale::Da.missing_interpolation_da("word"));

    assert_eq!("en foo", Locale::En.missing_interpolation_en("word"));
    assert_eq!("da foo word", Locale::Da.missing_interpolation_en("word"),);
}

#[test]
fn it_works_when_placeholders_are_rust_keywords() {
    assert_eq!("yo dawg", Locale::En.rust_keyword("dawg"));
    assert_eq!("hva så hund", Locale::Da.rust_keyword("hund"));
}

#[test]
fn it_works_for_duplicate_placeholders() {
    assert_eq!(
        "Hey Bob. Is your name Bob?",
        Locale::En.duplicate_placeholders("Bob")
    );

    assert_eq!(
        "Hej Bob. Er dit navn Bob?",
        Locale::Da.duplicate_placeholders("Bob")
    );
}

#[test]
fn it_works_for_multiple_placeholders() {
    assert_eq!("en one two", Locale::En.two_placeholders("one", "two"));

    assert_eq!("da one two", Locale::Da.two_placeholders("one", "two"));
}

#[test]
fn it_supports_strings_with_newlines() {
    assert_eq!("Hello\nWorld!", Locale::En.hello_newline());
    assert_eq!("Hej\nVerden!", Locale::Da.hello_newline());
}
