macro_rules! deps {
    () => {
        Stream!();
        Token!();
    };
}

macro_rules! test_lex_basic_string {
    () => {
        deps!();
        # [test] fn test_lex_basic_string () { let cases = [(r#""""# , str ! [[r#"
Token {
    kind: BasicString,
    span: 0..2,
}

"#]] . raw () , str ! [] . raw () ,) , (r#"""trailing"# , str ! [[r#"
Token {
    kind: BasicString,
    span: 0..2,
}

"#]] . raw () , str ! ["trailing"] . raw () ,) , (r#""content"trailing"# , str ! [[r#"
Token {
    kind: BasicString,
    span: 0..9,
}

"#]] . raw () , str ! ["trailing"] . raw () ,) , (r#""content"# , str ! [[r#"
Token {
    kind: BasicString,
    span: 0..8,
}

"#]] . raw () , str ! [] . raw () ,) , (r#""content\ntrailing"# , str ! [[r#"
Token {
    kind: BasicString,
    span: 0..18,
}

"#]] . raw () , str ! [] . raw () ,) ,] ; for (stream , expected_tokens , expected_stream) in cases { dbg ! (stream) ; let mut stream = Stream :: new (stream) ; let actual_tokens = lex_basic_string (& mut stream) ; assert_data_eq ! (actual_tokens . to_debug () , expected_tokens . raw ()) ; let stream = * stream ; assert_data_eq ! (stream , expected_stream . raw ()) ; } }
    };
}

test_lex_basic_string!()