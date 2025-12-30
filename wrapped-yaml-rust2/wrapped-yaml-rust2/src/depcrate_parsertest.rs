// Generated macro for test (module)
macro_rules! Depcrate_parsertest {
() => {
// Module: crate::parser
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: { Event , Parser } ; use crate :: YamlLoader ; # [test] fn test_peek_eq_parse () { let s = "
a0 bb: val
a1: &x
    b1: 4
    b2: d
a2: 4
a3: [1, 2, 3]
a4:
    - [a1, a2]
    - 2
a5: *x
" ; let mut p = Parser :: new_from_str (s) ; while { let event_peek = p . peek () . unwrap () . clone () ; let event = p . next_token () . unwrap () ; assert_eq ! (event , event_peek) ; event . 0 != Event :: StreamEnd } { } } # [test] fn test_keep_tags_across_multiple_documents () { let text = r#"
%YAML 1.1
%TAG !t! tag:test,2024:
--- !t!1 &1
foo: "bar"
--- !t!2 &2
baz: "qux"
"# ; let mut parser = Parser :: new_from_str (text) . keep_tags (true) ; let result = YamlLoader :: load_from_parser (& mut parser) ; assert ! (result . is_ok ()) ; let docs = result . unwrap () ; assert_eq ! (docs . len () , 2) ; let yaml = & docs [0] ; assert_eq ! (yaml ["foo"] . as_str () , Some ("bar")) ; let yaml = & docs [1] ; assert_eq ! (yaml ["baz"] . as_str () , Some ("qux")) ; let mut parser = Parser :: new_from_str (text) . keep_tags (false) ; let result = YamlLoader :: load_from_parser (& mut parser) ; assert ! (result . is_err ()) ; } }
};
}
