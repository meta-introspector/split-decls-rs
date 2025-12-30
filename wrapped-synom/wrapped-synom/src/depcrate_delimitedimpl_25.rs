// Generated macro for impl_25 (impl)
macro_rules! Depcrate_delimitedimpl_25 {
() => {
// Module: crate::delimited
// Provides: {"impl_25"}
// Dependencies: {}
impl < T , D > From < Vec < T > > for Delimited < T , D > where D : Default , { fn from (v : Vec < T >) -> Self { let len = v . len () ; Delimited { inner : v . into_iter () . enumerate () . map (| (i , item) | { (item , if i + 1 == len { None } else { Some (D :: default ()) }) }) . collect () , } } }
};
}
