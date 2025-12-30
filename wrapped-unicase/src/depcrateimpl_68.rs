// Generated macro for impl_68 (impl)
macro_rules! Depcrateimpl_68 {
() => {
// Module: crate
// Provides: {"impl_68"}
// Dependencies: {}
impl < S1 : AsRef < str > , S2 : AsRef < str > > PartialEq < UniCase < S2 > > for UniCase < S1 > { # [inline] fn eq (& self , other : & UniCase < S2 >) -> bool { match (& self . 0 , & other . 0) { (& Encoding :: Ascii (ref x) , & Encoding :: Ascii (ref y)) => x == y , (& Encoding :: Unicode (ref x) , & Encoding :: Unicode (ref y)) => x == y , (& Encoding :: Ascii (ref x) , & Encoding :: Unicode (ref y)) => & Unicode (x . as_ref ()) == y , (& Encoding :: Unicode (ref x) , & Encoding :: Ascii (ref y)) => x == & Unicode (y . as_ref ()) , } } }
};
}
