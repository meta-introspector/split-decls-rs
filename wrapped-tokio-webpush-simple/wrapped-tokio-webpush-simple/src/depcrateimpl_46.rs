// Generated macro for impl_46 (impl)
macro_rules! Depcrateimpl_46 {
() => {
// Module: crate
// Provides: {"impl_46"}
// Dependencies: {}
impl < S1 , S2 > Future for StreamNext < S1 , S2 > where S1 : Stream , S2 : Stream < Error = S1 :: Error > , { type Item = Option < (Either < S1 :: Item , S2 :: Item > , S1 , S2) > ; type Error = S1 :: Error ; fn poll (& mut self) -> Poll < Self :: Item , Self :: Error > { let item = { let left = self . left . as_mut () . unwrap () ; let right = self . right . as_mut () . unwrap () ; match left . poll () ? { Async :: Ready (None) => return Ok (Async :: Ready (None)) , Async :: Ready (Some (item)) => Either :: A (item) , Async :: NotReady => { match right . poll () ? { Async :: Ready (None) => return Ok (Async :: Ready (None)) , Async :: Ready (Some (item)) => Either :: B (item) , Async :: NotReady => return Ok (Async :: NotReady) , } } } } ; Ok (Async :: Ready (Some ((item , self . left . take () . unwrap () , self . right . take () . unwrap ())))) } }
};
}
