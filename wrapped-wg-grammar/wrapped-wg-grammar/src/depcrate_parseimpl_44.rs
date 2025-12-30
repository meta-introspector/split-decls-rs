// Generated macro for impl_44 (impl)
macro_rules! Depcrate_parseimpl_44 {
() => {
// Module: crate::parse
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'a , 'i , I : :: gll :: runtime :: Input > TOKEN_TREE < 'a , 'i , I > { fn from_sppf (parser : & 'a :: gll :: runtime :: Parser < 'i , _P , _C , I > , _node : ParseNode < 'i , _P > , _r : traverse ! (typeof (ParseNode <'i , _P >) { 0 _0 : P ! (& [:: gll :: proc_macro :: FlatTokenPat :: Ident (None) ,]) => ?, 1 _1 : P ! (& [:: gll :: proc_macro :: FlatTokenPat :: Punct { ch : None , joint : None } ,]) => ?, 2 _2 : P ! (& [:: gll :: proc_macro :: FlatTokenPat :: Literal ,]) => ?, 3 _3 : P ! (((& [:: gll :: proc_macro :: FlatTokenPat :: Delim ('(') ,] (TOKEN_TREE *)) & [:: gll :: proc_macro :: FlatTokenPat :: Delim (')') ,])) => ((?, ?) , ?) , 4 _4 : P ! (((& [:: gll :: proc_macro :: FlatTokenPat :: Delim ('[') ,] (TOKEN_TREE *)) & [:: gll :: proc_macro :: FlatTokenPat :: Delim (']') ,])) => ((?, ?) , ?) , 5 _5 : P ! (((& [:: gll :: proc_macro :: FlatTokenPat :: Delim ('{') ,] (TOKEN_TREE *)) & [:: gll :: proc_macro :: FlatTokenPat :: Delim ('}') ,])) => ((?, ?) , ?) , }) ,) -> Self { TOKEN_TREE { _marker : PhantomData , } } }
};
}
