// Generated macro for parsing (module)
macro_rules! Depcrate_delimitedparsing {
() => {
// Module: crate::delimited
// Provides: {"parsing"}
// Dependencies: {}
# [cfg (feature = "parsing")] mod parsing { use super :: Delimited ; use { PResult , Cursor , Synom , parse_error } ; impl < T , D > Delimited < T , D > where T : Synom , D : Synom , { pub fn parse_separated (input : Cursor) -> PResult < Self > { Self :: parse (input , T :: parse , false) } pub fn parse_separated_nonempty (input : Cursor) -> PResult < Self > { Self :: parse_separated_nonempty_with (input , T :: parse) } pub fn parse_terminated (input : Cursor) -> PResult < Self > { Self :: parse_terminated_with (input , T :: parse) } } impl < T , D > Delimited < T , D > where D : Synom , { pub fn parse_separated_nonempty_with (input : Cursor , parse : fn (Cursor) -> PResult < T >) -> PResult < Self > { match Self :: parse (input , parse , false) { Ok ((_ , ref b)) if b . is_empty () => parse_error () , other => other , } } pub fn parse_terminated_with (input : Cursor , parse : fn (Cursor) -> PResult < T >) -> PResult < Self > { Self :: parse (input , parse , true) } fn parse (mut input : Cursor , parse : fn (Cursor) -> PResult < T > , terminated : bool) -> PResult < Self > { let mut res = Delimited :: new () ; match parse (input) { Err (_) => Ok ((input , res)) , Ok ((i , o)) => { if i == input { return parse_error () ; } input = i ; res . push_first (o) ; while let Ok ((i2 , s)) = D :: parse (input) { if i2 == input { break ; } if let Ok ((i3 , o3)) = parse (i2) { if i3 == i2 { break ; } res . push_next (o3 , s) ; input = i3 ; } else { break ; } } if terminated { if let Ok ((after , sep)) = D :: parse (input) { res . push_trailing (sep) ; input = after ; } } Ok ((input , res)) } } } } }
};
}
