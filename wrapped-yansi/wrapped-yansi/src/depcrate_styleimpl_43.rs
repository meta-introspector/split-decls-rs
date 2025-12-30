// Generated macro for impl_43 (impl)
macro_rules! Depcrate_styleimpl_43 {
() => {
// Module: crate::style
// Provides: {"impl_43"}
// Dependencies: {}
impl core :: hash :: Hash for Style { fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { let Style { foreground , background , attributes , quirks : _ , condition : _ , } = self ; foreground . hash (state) ; background . hash (state) ; attributes . hash (state) ; } }
};
}
