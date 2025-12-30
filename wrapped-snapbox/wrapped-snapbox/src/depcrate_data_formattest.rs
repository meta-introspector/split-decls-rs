// Generated macro for test (module)
macro_rules! Depcrate_data_formattest {
() => {
// Module: crate::data::format
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn combos () { # [cfg (feature = "json")] let json = DataFormat :: Json ; # [cfg (not (feature = "json"))] let json = DataFormat :: Text ; # [cfg (feature = "json")] let jsonl = DataFormat :: JsonLines ; # [cfg (not (feature = "json"))] let jsonl = DataFormat :: Text ; # [cfg (feature = "term-svg")] let term_svg = DataFormat :: TermSvg ; # [cfg (not (feature = "term-svg"))] let term_svg = DataFormat :: Text ; let cases = [("foo" , DataFormat :: Text) , (".foo" , DataFormat :: Text) , ("foo.txt" , DataFormat :: Text) , (".foo.txt" , DataFormat :: Text) , ("foo.stdout.txt" , DataFormat :: Text) , ("foo.json" , json) , ("foo.stdout.json" , json) , (".foo.json" , json) , ("foo.jsonl" , jsonl) , ("foo.stdout.jsonl" , jsonl) , (".foo.jsonl" , jsonl) , ("foo.term.svg" , term_svg) , ("foo.stdout.term.svg" , term_svg) , (".foo.term.svg" , term_svg) ,] ; for (input , output) in cases { let input = std :: path :: Path :: new (input) ; assert_eq ! (DataFormat :: from (input) , output , "for `{}`" , input . display ()) ; } } }
};
}
