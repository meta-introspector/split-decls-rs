// Generated macro for tuple_parser (macro)
macro_rules! Depcratetuple_parser {
() => {
// Module: crate
// Provides: {"tuple_parser"}
// Dependencies: {}
# [doc = " Internal parser, do not use directly."] # [doc (hidden)] # [macro_export] macro_rules ! tuple_parser { ($ i : expr , ($ ($ parsed : tt) ,*) , $ e : ident , $ ($ rest : tt) *) => { tuple_parser ! ($ i , ($ ($ parsed) ,*) , call ! ($ e) , $ ($ rest) *) } ; ($ i : expr , () , $ submac : ident ! ($ ($ args : tt) *) , $ ($ rest : tt) *) => { match $ submac ! ($ i , $ ($ args) *) { :: std :: result :: Result :: Err (err) => :: std :: result :: Result :: Err (err) , :: std :: result :: Result :: Ok ((i , o)) => tuple_parser ! (i , (o) , $ ($ rest) *) , } } ; ($ i : expr , ($ ($ parsed : tt) *) , $ submac : ident ! ($ ($ args : tt) *) , $ ($ rest : tt) *) => { match $ submac ! ($ i , $ ($ args) *) { :: std :: result :: Result :: Err (err) => :: std :: result :: Result :: Err (err) , :: std :: result :: Result :: Ok ((i , o)) => tuple_parser ! (i , ($ ($ parsed) * , o) , $ ($ rest) *) , } } ; ($ i : expr , ($ ($ parsed : tt) ,*) , $ e : ident) => { tuple_parser ! ($ i , ($ ($ parsed) ,*) , call ! ($ e)) } ; ($ i : expr , () , $ submac : ident ! ($ ($ args : tt) *)) => { $ submac ! ($ i , $ ($ args) *) } ; ($ i : expr , ($ ($ parsed : expr) ,*) , $ submac : ident ! ($ ($ args : tt) *)) => { match $ submac ! ($ i , $ ($ args) *) { :: std :: result :: Result :: Err (err) => :: std :: result :: Result :: Err (err) , :: std :: result :: Result :: Ok ((i , o)) => :: std :: result :: Result :: Ok ((i , ($ ($ parsed) ,*, o))) , } } ; ($ i : expr , ($ ($ parsed : expr) ,*)) => { :: std :: result :: Result :: Ok (($ i , ($ ($ parsed) ,*))) } ; }
};
}
