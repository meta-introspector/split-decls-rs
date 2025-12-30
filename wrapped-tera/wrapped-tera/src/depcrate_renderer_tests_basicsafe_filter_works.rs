// Generated macro for safe_filter_works (function)
macro_rules! Depcrate_renderer_tests_basicsafe_filter_works {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"safe_filter_works"}
// Dependencies: {}
# [test] fn safe_filter_works () { struct Safe ; impl crate :: Filter for Safe { fn filter (& self , value : & Value , _args : & HashMap < String , Value >) -> Result < Value > { Ok (Value :: String (format ! ("<div>{}</div>" , value . as_str () . unwrap ()))) } fn is_safe (& self) -> bool { true } } let mut tera = Tera :: default () ; tera . register_filter ("safe_filter" , Safe) ; tera . add_raw_template ("test.html" , r#"{{ "Hello" | safe_filter }}"#) . unwrap () ; let res = tera . render ("test.html" , & Context :: new ()) ; assert_eq ! (res . unwrap () , "<div>Hello</div>") ; }
};
}
