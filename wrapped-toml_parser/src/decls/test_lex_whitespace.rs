macro_rules! deps {
    () => {
        Token!();
        Stream!();
    };
}

macro_rules! test_lex_whitespace {
    () => {
        deps!();
        # [test] fn test_lex_whitespace () { let cases = [(" " , str ! [[r#"
Token {
    kind: Whitespace,
    span: 0..1,
}

"#]] . raw () , str ! [] . raw () ,) , (" \t  \t  \t " , str ! [[r#"
Token {
    kind: Whitespace,
    span: 0..9,
}

"#]] . raw () , str ! [] . raw () ,) , (" \n" , str ! [[r#"
Token {
    kind: Whitespace,
    span: 0..1,
}

"#]] . raw () , str ! [[r#"


"#]] . raw () ,) , (" #" , str ! [[r#"
Token {
    kind: Whitespace,
    span: 0..1,
}

"#]] . raw () , str ! ["#"] . raw () ,) , (" a" , str ! [[r#"
Token {
    kind: Whitespace,
    span: 0..1,
}

"#]] . raw () , str ! ["a"] . raw () ,) ,] ; for (stream , expected_tokens , expected_stream) in cases { dbg ! (stream) ; let mut stream = Stream :: new (stream) ; let actual_tokens = lex_whitespace (& mut stream) ; assert_data_eq ! (actual_tokens . to_debug () , expected_tokens . raw ()) ; let stream = * stream ; assert_data_eq ! (stream , expected_stream . raw ()) ; } }
    };
}

test_lex_whitespace!()