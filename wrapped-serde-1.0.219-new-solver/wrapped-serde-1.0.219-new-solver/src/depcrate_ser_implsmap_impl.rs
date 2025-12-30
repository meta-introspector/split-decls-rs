// Generated macro for map_impl (macro)
macro_rules! Depcrate_ser_implsmap_impl {
() => {
// Module: crate::ser::impls
// Provides: {"map_impl"}
// Dependencies: {}
# [cfg (no_relaxed_trait_bounds)] macro_rules ! map_impl { ($ (# [$ attr : meta]) * $ ty : ident < K $ (: $ kbound1 : ident $ (+ $ kbound2 : ident) *) *, V $ (, $ typaram : ident : $ bound : ident) *>) => { $ (# [$ attr]) * impl < K , V $ (, $ typaram) *> Serialize for $ ty < K , V $ (, $ typaram) *> where K : Serialize $ (+ $ kbound1 $ (+ $ kbound2) *) *, V : Serialize , $ ($ typaram : $ bound ,) * { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_map (self) } } } }
};
}
