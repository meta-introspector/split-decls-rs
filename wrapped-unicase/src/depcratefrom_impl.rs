// Generated macro for from_impl (macro)
macro_rules! Depcratefrom_impl {
() => {
// Module: crate
// Provides: {"from_impl"}
// Dependencies: {}
macro_rules ! from_impl { ($ from : ty => $ to : ty ; $ by : ident) => (impl <'a > From <$ from > for UniCase <$ to > { fn from (s : $ from) -> Self { UniCase :: unicode (s .$ by ()) } }) ; ($ from : ty => $ to : ty) => (from_impl ! ($ from => $ to ; into) ;) }
};
}
