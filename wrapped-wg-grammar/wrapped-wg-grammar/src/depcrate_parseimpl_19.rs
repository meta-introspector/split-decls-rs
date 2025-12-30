// Generated macro for impl_19 (impl)
macro_rules! Depcrate_parseimpl_19 {
() => {
// Module: crate::parse
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a , 'i , I : :: gll :: runtime :: Input , T > Iterator for Handle < 'a , 'i , I , [T] > { type Item = Result < Handle < 'a , 'i , I , T > , Ambiguity < Self > > ; fn next (& mut self) -> Option < Self :: Item > { match self . all_list_heads () { ListHead :: Cons (mut iter) => { let (elem , rest) = iter . next () . unwrap () ; let original = * self ; * self = rest ; if iter . next () . is_none () { Some (Ok (elem)) } else { match self . node . kind . shape () { ParseNodeShape :: Opt (_) => { self . node . range = Range (original . node . range . split_at (0) . 0) ; } _ => unreachable ! () , } match self . one_list_head () { ListHead :: Nil => { } _ => unreachable ! () , } Some (Err (Ambiguity (original))) } } ListHead :: Nil => None , } } }
};
}
