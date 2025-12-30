// Generated macro for big_struct_n (function)
macro_rules! Depcrate_bench_fixturebig_struct_n {
() => {
// Module: crate::bench_fixture
// Provides: {"big_struct_n"}
// Dependencies: {}
pub fn big_struct_n (n : u32) -> String { let mut buf = "pub struct RegisterBlock {" . to_owned () ; for i in 0 .. n { format_to ! (buf , "  /// Doc comment for {}.\n" , i) ; format_to ! (buf , "  pub s{}: S{},\n" , i , i) ; } buf . push_str ("}\n\n") ; for i in 0 .. n { format_to ! (buf , "

#[repr(transparent)]
struct S{} {{
    field: u32,
}}" , i) ; } buf }
};
}
