// Generated macro for nested_limited (function)
macro_rules! Depcrate_dernested_limited {
() => {
// Module: crate::der
// Provides: {"nested_limited"}
// Dependencies: {}
pub (crate) fn nested_limited < 'a , R > (input : & mut untrusted :: Reader < 'a > , tag : Tag , error : Error , decoder : impl FnOnce (& mut untrusted :: Reader < 'a >) -> Result < R , Error > , size_limit : usize ,) -> Result < R , Error > { match expect_tag_and_get_value_limited (input , tag , size_limit) { Ok (value) => value . read_all (error , decoder) , Err (_) => Err (error) , } }
};
}
