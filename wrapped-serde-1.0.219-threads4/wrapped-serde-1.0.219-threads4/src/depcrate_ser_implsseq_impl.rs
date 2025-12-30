// Generated macro for seq_impl (macro)
macro_rules! Depcrate_ser_implsseq_impl {
() => {
// Module: crate::ser::impls
// Provides: {"seq_impl"}
// Dependencies: {}
# [cfg (no_relaxed_trait_bounds)] macro_rules ! seq_impl { ($ (# [$ attr : meta]) * $ ty : ident < T $ (: $ tbound1 : ident $ (+ $ tbound2 : ident) *) * $ (, $ typaram : ident : $ bound : ident) *>) => { $ (# [$ attr]) * impl < T $ (, $ typaram) *> Serialize for $ ty < T $ (, $ typaram) *> where T : Serialize $ (+ $ tbound1 $ (+ $ tbound2) *) *, $ ($ typaram : $ bound ,) * { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_seq (self) } } } }
};
}
