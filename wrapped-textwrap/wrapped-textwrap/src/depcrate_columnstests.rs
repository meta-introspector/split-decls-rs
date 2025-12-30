// Generated macro for tests (module)
macro_rules! Depcrate_columnstests {
() => {
// Module: crate::columns
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn wrap_columns_empty_text () { assert_eq ! (wrap_columns ("" , 1 , 10 , "| " , "" , " |") , vec ! ["|        |"]) ; } # [test] fn wrap_columns_single_column () { assert_eq ! (wrap_columns ("Foo" , 3 , 30 , "| " , " | " , " |") , vec ! ["| Foo    |        |          |"]) ; } # [test] fn wrap_columns_uneven_columns () { assert_eq ! (wrap_columns ("Foo Bar Baz Quux" , 4 , 21 , "|" , "|" , "|") , vec ! ["|Foo |Bar |Baz |Quux|"]) ; assert_eq ! (wrap_columns ("Foo Bar Baz Quux" , 4 , 24 , "|" , "|" , "|") , vec ! ["|Foo |Bar |Baz |Quux   |"]) ; assert_eq ! (wrap_columns ("Foo Bar Baz Quux" , 4 , 25 , "|" , "|" , "|") , vec ! ["|Foo  |Bar  |Baz  |Quux |"]) ; } # [test] # [cfg (feature = "unicode-width")] fn wrap_columns_with_emojis () { assert_eq ! (wrap_columns ("Words and a few emojis 😍 wrapped in ⓶ columns" , 2 , 30 , "✨ " , " ⚽ " , " 👀") , vec ! ["✨ Words      ⚽ wrapped in 👀" , "✨ and a few  ⚽ ⓶ columns  👀" , "✨ emojis 😍  ⚽            👀"]) ; } # [test] fn wrap_columns_big_gaps () { assert_eq ! (wrap_columns ("xyz" , 2 , 10 , "----> " , " !!! " , " <----") , vec ! ["----> x !!! z <----" , "----> y !!!   <----"]) ; } # [test] # [should_panic] fn wrap_columns_panic_with_zero_columns () { wrap_columns ("" , 0 , 10 , "" , "" , "") ; } }
};
}
