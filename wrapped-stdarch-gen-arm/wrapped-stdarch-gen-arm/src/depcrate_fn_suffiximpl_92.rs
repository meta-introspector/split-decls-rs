// Generated macro for impl_92 (impl)
macro_rules! Depcrate_fn_suffiximpl_92 {
() => {
// Module: crate::fn_suffix
// Provides: {"impl_92"}
// Dependencies: {}
impl fmt :: Display for SuffixKind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { SuffixKind :: Normal => write ! (f , "normal") , SuffixKind :: NoQ => write ! (f , "NoQ") , SuffixKind :: NSuffix => write ! (f , "NSuffix") , SuffixKind :: NoQNSuffix => write ! (f , "NoQNSuffix") , SuffixKind :: DupNox => write ! (f , "DupNox") , SuffixKind :: Dup => write ! (f , "Dup" ,) , SuffixKind :: Lane => write ! (f , "Lane") , SuffixKind :: LaneNoX => write ! (f , "LaneNoX") , SuffixKind :: LaneQNoX => write ! (f , "LaneQNoX") , SuffixKind :: Base => write ! (f , "Base") , SuffixKind :: Rot270 => write ! (f , "Rot270" ,) , SuffixKind :: Rot270Lane => write ! (f , "Rot270Lane") , SuffixKind :: Rot270LaneQ => write ! (f , "Rot270LaneQ") , SuffixKind :: Rot90 => write ! (f , "Rot90" ,) , SuffixKind :: Rot90Lane => write ! (f , "Rot90Lane") , SuffixKind :: Rot90LaneQ => write ! (f , "Rot90LaneQ") , SuffixKind :: Rot180 => write ! (f , "Rot180" ,) , SuffixKind :: Rot180Lane => write ! (f , "Rot180Lane") , SuffixKind :: Rot180LaneQ => write ! (f , "Rot180LaneQ") , SuffixKind :: Unsigned => write ! (f , "Unsigned") , SuffixKind :: Tuple => write ! (f , "Tuple") , SuffixKind :: NoX => write ! (f , "NoX") , SuffixKind :: BaseByteSize => write ! (f , "BaseByteSize") , } } }
};
}
