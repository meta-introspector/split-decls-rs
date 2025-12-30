// Generated macro for impl_21 (impl)
macro_rules! Depcrate_parseimpl_21 {
() => {
// Module: crate::parse
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a , 'i , I : :: gll :: runtime :: Input , T > Handle < 'a , 'i , I , [T] > { fn one_list_head (self) -> ListHead < Result < (Handle < 'a , 'i , I , T > , Handle < 'a , 'i , I , [T] >) , Ambiguity < Self > > > { match self . all_list_heads () { ListHead :: Cons (mut iter) => { let first = iter . next () . unwrap () ; if iter . next () . is_none () { ListHead :: Cons (Ok (first)) } else { ListHead :: Cons (Err (Ambiguity (self))) } } ListHead :: Nil => ListHead :: Nil , } } fn all_list_heads (mut self) -> ListHead < impl Iterator < Item = (Handle < 'a , 'i , I , T > , Handle < 'a , 'i , I , [T] >) > > { if let ParseNodeShape :: Opt (_) = self . node . kind . shape () { if let Some (opt_child) = self . node . unpack_opt () { self . node = opt_child ; } else { return ListHead :: Nil ; } } ListHead :: Cons (self . parser . sppf . all_splits (self . node) . flat_map (move | (elem , rest) | { if let ParseNodeShape :: Split (..) = rest . kind . shape () { Some (self . parser . sppf . all_splits (rest)) . into_iter () . flatten () . chain (None) } else { None . into_iter () . flatten () . chain (Some ((elem , rest))) } }) . map (move | (elem , rest) | { (Handle { node : elem , parser : self . parser , _marker : PhantomData , } , Handle { node : rest , .. self }) })) } }
};
}
