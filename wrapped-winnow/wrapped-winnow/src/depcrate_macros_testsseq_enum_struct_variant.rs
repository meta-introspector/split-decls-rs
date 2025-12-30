// Generated macro for seq_enum_struct_variant (function)
macro_rules! Depcrate_macros_testsseq_enum_struct_variant {
() => {
// Module: crate::macros::tests
// Provides: {"seq_enum_struct_variant"}
// Dependencies: {}
# [test] fn seq_enum_struct_variant () { # [derive (Debug , PartialEq , Eq)] enum Expr { Add { lhs : u32 , rhs : u32 } , Mul (u32 , u32) , } fn add < 'i > (input : & mut & 'i [u8]) -> TestResult < & 'i [u8] , Expr > { seq ! { Expr :: Add { lhs : dec_uint ::< _ , u32 , _ >, _ : b" + " , rhs : dec_uint ::< _ , u32 , _ >, } } . parse_next (input) } fn mul < 'i > (input : & mut & 'i [u8]) -> TestResult < & 'i [u8] , Expr > { seq ! (Expr :: Mul (dec_uint ::< _ , u32 , _ >, _ : b" * " , dec_uint ::< _ , u32 , _ >,)) . parse_next (input) } assert_parse ! (add . parse_peek (& b"1 + 2" [..]) , str ! [[r#"
Ok(
    (
        [],
        Add {
            lhs: 1,
            rhs: 2,
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (mul . parse_peek (& b"3 * 4" [..]) , str ! [[r#"
Ok(
    (
        [],
        Mul(
            3,
            4,
        ),
    ),
)

"#]] . raw ()) ; }
};
}
