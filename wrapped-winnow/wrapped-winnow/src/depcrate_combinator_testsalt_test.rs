// Generated macro for alt_test (function)
macro_rules! Depcrate_combinator_testsalt_test {
() => {
// Module: crate::combinator::tests
// Provides: {"alt_test"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [test] fn alt_test () { # [cfg (feature = "alloc")] use crate :: { alloc :: { fmt :: Debug , string :: String } , error :: ParserError , } ; # [cfg (feature = "alloc")] # [derive (Debug , Clone , Eq , PartialEq)] struct ErrorStr (String) ; # [cfg (feature = "alloc")] impl From < u32 > for ErrorStr { fn from (i : u32) -> Self { ErrorStr (format ! ("custom error code: {i}")) } } # [cfg (feature = "alloc")] impl < 'a > From < & 'a str > for ErrorStr { fn from (i : & 'a str) -> Self { ErrorStr (format ! ("custom error message: {i}")) } } # [cfg (feature = "alloc")] impl < I : Stream + Debug > ParserError < I > for ErrorStr { type Inner = Self ; fn from_input (input : & I) -> Self { ErrorStr (format ! ("custom error message: ({input:?})")) } fn append (self , input : & I , _ : & < I as Stream > :: Checkpoint) -> Self { ErrorStr (format ! ("custom error message: ({input:?}) - {self:?}")) } fn into_inner (self) -> Result < Self :: Inner , Self > { Ok (self) } } fn work < 'i > (input : & mut & 'i [u8]) -> ModalResult < & 'i [u8] , ErrorStr > { Ok (input . finish ()) } # [allow (unused_variables)] fn dont_work < 'i > (input : & mut & 'i [u8]) -> ModalResult < & 'i [u8] , ErrorStr > { Err (ErrMode :: Backtrack (ErrorStr ("abcd" . to_owned ()))) } fn work2 < 'i > (_input : & mut & 'i [u8]) -> ModalResult < & 'i [u8] , ErrorStr > { Ok (& b"" [..]) } fn alt1 < 'i > (i : & mut & 'i [u8]) -> ModalResult < & 'i [u8] , ErrorStr > { alt ((dont_work , dont_work)) . parse_next (i) } fn alt2 < 'i > (i : & mut & 'i [u8]) -> ModalResult < & 'i [u8] , ErrorStr > { alt ((dont_work , work)) . parse_next (i) } fn alt3 < 'i > (i : & mut & 'i [u8]) -> ModalResult < & 'i [u8] , ErrorStr > { alt ((dont_work , dont_work , work2 , dont_work)) . parse_next (i) } let a = & b"abcd" [..] ; assert_eq ! (alt1 . parse_peek (a) , Err (ErrMode :: Backtrack (ErrorStr ("custom error message: ([97, 98, 99, 100]) - ErrorStr(\"abcd\")" . to_owned ())))) ; assert_eq ! (alt2 . parse_peek (a) , Ok ((& b"" [..] , a))) ; assert_eq ! (alt3 . parse_peek (a) , Ok ((a , & b"" [..]))) ; fn alt4 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , & 'i [u8] > { alt (("abcd" , "efgh")) . parse_next (i) } let b = & b"efgh" [..] ; assert_parse ! (alt4 . parse_peek (a) , str ! [[r#"
Ok(
    (
        [],
        [
            97,
            98,
            99,
            100,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (alt4 . parse_peek (b) , str ! [[r#"
Ok(
    (
        [],
        [
            101,
            102,
            103,
            104,
        ],
    ),
)

"#]] . raw ()) ; }
};
}
