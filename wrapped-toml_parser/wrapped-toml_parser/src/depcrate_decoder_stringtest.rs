// Generated macro for test (module)
macro_rules! Depcrate_decoder_stringtest {
() => {
// Module: crate::decoder::string
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "std")] mod test { use super :: * ; use crate :: decoder :: Encoding ; use alloc :: borrow :: Cow ; use snapbox :: assert_data_eq ; use snapbox :: prelude :: * ; use snapbox :: str ; # [test] fn literal_string () { let cases = [(r"'C:\Users\nodejs\templates'" , str ! [[r#"C:\Users\nodejs\templates"#]] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , (r"'\\ServerX\admin$\system32\'" , str ! [[r#"\\ServerX\admin$\system32\"#]] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , (r#"'Tom "Dubs" Preston-Werner'"# , str ! [[r#"Tom "Dubs" Preston-Werner"#]] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , (r"'<\i\c*\s*>'" , str ! [[r#"<\i\c*\s*>"#]] . raw () , str ! [[r#"
[]

"#]] . raw () ,) ,] ; for (input , expected , expected_error) in cases { let mut error = Vec :: new () ; let mut actual = Cow :: Borrowed ("") ; decode_literal_string (Raw :: new_unchecked (input , Some (Encoding :: LiteralString) , Default :: default ()) , & mut actual , & mut error ,) ; assert_data_eq ! (actual . as_ref () , expected) ; assert_data_eq ! (error . to_debug () , expected_error) ; } } # [test] fn ml_literal_string () { let cases = [(r"'''I [dw]on't need \d{2} apples'''" , str ! [[r#"I [dw]on't need \d{2} apples"#]] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , (r#"''''one_quote''''"# , str ! ["'one_quote'"] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , (r#"'''
The first newline is
trimmed in raw strings.
   All other whitespace
   is preserved.
'''"# , str ! [[r#"
The first newline is
trimmed in raw strings.
   All other whitespace
   is preserved.

"#]] . raw () , str ! [[r#"
[]

"#]] . raw () ,) ,] ; for (input , expected , expected_error) in cases { let mut error = Vec :: new () ; let mut actual = Cow :: Borrowed ("") ; decode_ml_literal_string (Raw :: new_unchecked (input , Some (Encoding :: MlLiteralString) , Default :: default ()) , & mut actual , & mut error ,) ; assert_data_eq ! (actual . as_ref () , expected) ; assert_data_eq ! (error . to_debug () , expected_error) ; } } # [test] fn basic_string () { let cases = [(r#""""# , str ! [""] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , (r#""content\"trailing""# , str ! [[r#"content"trailing"#]] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , (r#""content\""# , str ! [[r#"content\"#]] . raw () , str ! [[r#"
[
    ParseError {
        context: Some(
            0..10,
        ),
        description: "missing escaped value",
        expected: Some(
            [
                Literal(
                    "b",
                ),
                Literal(
                    "f",
                ),
                Literal(
                    "n",
                ),
                Literal(
                    "r",
                ),
                Literal(
                    "\\",
                ),
                Literal(
                    "\"",
                ),
                Literal(
                    "u",
                ),
                Literal(
                    "U",
                ),
            ],
        ),
        unexpected: Some(
            9..9,
        ),
    },
]

"#]] . raw () ,) , (r#""content
trailing""# , str ! [[r#"
content
trailing
"#]] . raw () , str ! [[r#"
[
    ParseError {
        context: Some(
            0..18,
        ),
        description: "invalid basic string",
        expected: Some(
            [
                Description(
                    "non-double-quote visible characters",
                ),
                Literal(
                    "\\",
                ),
            ],
        ),
        unexpected: Some(
            8..9,
        ),
    },
]

"#]] . raw () ,) , (r#""I'm a string. \"You can quote me\". Name\tJos\u00E9\nLocation\tSF. \U0002070E""# , str ! [[r#"
I'm a string. "You can quote me". Name	José
Location	SF. 𠜎
"#]] . raw () , str ! [[r#"
[]

"#]] . raw () ,) ,] ; for (input , expected , expected_error) in cases { let mut error = Vec :: new () ; let mut actual = Cow :: Borrowed ("") ; decode_basic_string (Raw :: new_unchecked (input , Some (Encoding :: BasicString) , Default :: default ()) , & mut actual , & mut error ,) ; assert_data_eq ! (actual . as_ref () , expected) ; assert_data_eq ! (error . to_debug () , expected_error) ; } } # [test] fn ml_basic_string () { let cases = [(r#""""
Roses are red
Violets are blue""""# , str ! [[r#"
Roses are red
Violets are blue
"#]] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , (r#"""" \""" """"# , str ! [[r#" """ "#]] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , (r#"""" \\""""# , str ! [[r#" \"#]] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , (r#""""
The quick brown \


  fox jumps over \
    the lazy dog.""""# , str ! ["The quick brown fox jumps over the lazy dog."] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , (r#""""\
       The quick brown \
       fox jumps over \
       the lazy dog.\
       """"# , str ! ["The quick brown fox jumps over the lazy dog."] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , (r#""""\
       """"# , str ! [""] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , (r#""""
\
  \
""""# , str ! [""] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , (r#""""  """# , str ! [[r#"  """#]] . raw () , str ! [[r#"
[
    ParseError {
        context: Some(
            0..7,
        ),
        description: "invalid multi-line basic string",
        expected: Some(
            [
                Literal(
                    "\"",
                ),
            ],
        ),
        unexpected: Some(
            7..7,
        ),
    },
]

"#]] . raw () ,) , (r#""""  \""""# , str ! [[r#"  \"#]] . raw () , str ! [[r#"
[
    ParseError {
        context: Some(
            0..9,
        ),
        description: "missing escaped value",
        expected: Some(
            [
                Literal(
                    "b",
                ),
                Literal(
                    "f",
                ),
                Literal(
                    "n",
                ),
                Literal(
                    "r",
                ),
                Literal(
                    "\\",
                ),
                Literal(
                    "\"",
                ),
                Literal(
                    "u",
                ),
                Literal(
                    "U",
                ),
            ],
        ),
        unexpected: Some(
            6..6,
        ),
    },
]

"#]] . raw () ,) ,] ; for (input , expected , expected_error) in cases { let mut error = Vec :: new () ; let mut actual = Cow :: Borrowed ("") ; decode_ml_basic_string (Raw :: new_unchecked (input , Some (Encoding :: MlBasicString) , Default :: default ()) , & mut actual , & mut error ,) ; assert_data_eq ! (actual . as_ref () , expected) ; assert_data_eq ! (error . to_debug () , expected_error) ; } } # [test] fn unquoted_keys () { let cases = [("a" , str ! ["a"] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , ("hello" , str ! ["hello"] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , ("-" , str ! ["-"] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , ("_" , str ! ["_"] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , ("-hello-world-" , str ! ["-hello-world-"] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , ("_hello_world_" , str ! ["_hello_world_"] . raw () , str ! [[r#"
[]

"#]] . raw () ,) , ("" , str ! [""] . raw () , str ! [[r#"
[
    ParseError {
        context: Some(
            0..0,
        ),
        description: "unquoted keys cannot be empty",
        expected: Some(
            [
                Description(
                    "letters",
                ),
                Description(
                    "numbers",
                ),
                Literal(
                    "-",
                ),
                Literal(
                    "_",
                ),
            ],
        ),
        unexpected: Some(
            0..0,
        ),
    },
]

"#]] . raw () ,) ,] ; for (input , expected , expected_error) in cases { let mut error = Vec :: new () ; let mut actual = Cow :: Borrowed ("") ; decode_unquoted_key (Raw :: new_unchecked (input , None , Default :: default ()) , & mut actual , & mut error ,) ; assert_data_eq ! (actual . as_ref () , expected) ; assert_data_eq ! (error . to_debug () , expected_error) ; } } }
};
}
