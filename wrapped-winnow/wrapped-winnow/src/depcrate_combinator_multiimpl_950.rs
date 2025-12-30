// Generated macro for impl_950 (impl)
macro_rules! Depcrate_combinator_multiimpl_950 {
() => {
// Module: crate::combinator::multi
// Provides: {"impl_950"}
// Dependencies: {}
impl < P , I , O , C , E > Parser < I , C , E > for Repeat < P , I , O , C , E > where P : Parser < I , O , E > , I : Stream , C : Accumulate < O > , E : ParserError < I > , { # [inline (always)] fn parse_next (& mut self , i : & mut I) -> Result < C , E > { let Range { start_inclusive , end_inclusive , } = self . occurrences ; trace ("repeat" , move | i : & mut I | { match (start_inclusive , end_inclusive) { (0 , None) => fold_repeat0_ (& mut self . parser , & mut | | C :: initial (None) , & mut | mut acc , o | { acc . accumulate (o) ; acc } , i ,) , (1 , None) => fold_repeat1_ (& mut self . parser , & mut | | C :: initial (None) , & mut | mut acc , o | { acc . accumulate (o) ; acc } , i ,) , (min , end) if Some (min) == end => fold_repeat_n_ (min , & mut self . parser , & mut | | C :: initial (Some (min)) , & mut | mut acc , o | { acc . accumulate (o) ; acc } , i ,) , (min , end) => fold_repeat_m_n_ (min , end . unwrap_or (usize :: MAX) , & mut self . parser , & mut | | C :: initial (Some (min)) , & mut | mut acc , o | { acc . accumulate (o) ; acc } , i ,) , } }) . parse_next (i) } }
};
}
