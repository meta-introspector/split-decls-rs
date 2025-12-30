// Generated macro for tests (module)
macro_rules! Depcrate_contexttests {
() => {
// Module: crate::context
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use serde_json :: json ; use std :: collections :: HashMap ; # [test] fn test_dotted_pointer () { let data = r#"{
            "foo": {
                "bar": {
                    "goo": {
                        "moo": {
                            "cows": [
                                {
                                    "name": "betsy",
                                    "age" : 2,
                                    "temperament": "calm"
                                },
                                {
                                    "name": "elsie",
                                    "age": 3,
                                    "temperament": "calm"
                                },
                                {
                                    "name": "veal",
                                    "age": 1,
                                    "temperament": "ornery"
                                }
                            ]
                        }
                    }
                },
                "http://example.com/": {
                    "goo": {
                        "moo": {
                            "cows": [
                                {
                                    "name": "betsy",
                                    "age" : 2,
                                    "temperament": "calm"
                                },
                                {
                                    "name": "elsie",
                                    "age": 3,
                                    "temperament": "calm"
                                },
                                {
                                    "name": "veal",
                                    "age": 1,
                                    "temperament": "ornery"
                                }
                            ]
                        }
                    }
                }
            }
            }"# ; let value = serde_json :: from_str (data) . unwrap () ; assert_eq ! (dotted_pointer (& value , "") , Some (& value)) ; assert_eq ! (dotted_pointer (& value , "foo") , value . pointer ("/foo")) ; assert_eq ! (dotted_pointer (& value , "foo.bar.goo") , value . pointer ("/foo/bar/goo")) ; assert_eq ! (dotted_pointer (& value , "skrr") , value . pointer ("/skrr")) ; assert_eq ! (dotted_pointer (& value , r#"foo["bar"].baz"#) , value . pointer (r#"/foo["bar"]/baz"#)) ; assert_eq ! (dotted_pointer (& value , r#"foo["bar"].baz["qux"].blub"#) , value . pointer (r#"/foo["bar"]/baz["qux"]/blub"#)) ; } # [test] fn can_extend_context () { let mut target = Context :: new () ; target . insert ("a" , & 1) ; target . insert ("b" , & 2) ; let mut source = Context :: new () ; source . insert ("b" , & 3) ; source . insert ("c" , & 4) ; target . extend (source) ; assert_eq ! (* target . data . get ("a") . unwrap () , to_value (1) . unwrap ()) ; assert_eq ! (* target . data . get ("b") . unwrap () , to_value (3) . unwrap ()) ; assert_eq ! (* target . data . get ("c") . unwrap () , to_value (4) . unwrap ()) ; } # [test] fn can_create_context_from_value () { let obj = json ! ({ "name" : "bob" , "age" : 25 }) ; let context_from_value = Context :: from_value (obj) . unwrap () ; let mut context = Context :: new () ; context . insert ("name" , "bob") ; context . insert ("age" , & 25) ; assert_eq ! (context_from_value , context) ; } # [test] fn can_create_context_from_impl_serialize () { let mut map = HashMap :: new () ; map . insert ("name" , "bob") ; map . insert ("last_name" , "something") ; let context_from_serialize = Context :: from_serialize (& map) . unwrap () ; let mut context = Context :: new () ; context . insert ("name" , "bob") ; context . insert ("last_name" , "something") ; assert_eq ! (context_from_serialize , context) ; } # [test] fn can_remove_a_key () { let mut context = Context :: new () ; context . insert ("name" , "foo") ; context . insert ("bio" , "Hi, I'm foo.") ; let mut expected = Context :: new () ; expected . insert ("name" , "foo") ; assert_eq ! (context . remove ("bio") , Some (to_value ("Hi, I'm foo.") . unwrap ())) ; assert_eq ! (context . get ("bio") , None) ; assert_eq ! (context , expected) ; } # [test] fn remove_return_none_with_unknown_index () { let mut context = Context :: new () ; assert_eq ! (context . remove ("unknown") , None) ; } }
};
}
