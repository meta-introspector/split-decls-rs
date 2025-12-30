// Generated macro for nonzero_integers (macro)
macro_rules! Depcrate_ser_implsnonzero_integers {
() => {
// Module: crate::ser::impls
// Provides: {"nonzero_integers"}
// Dependencies: {}
macro_rules ! nonzero_integers { ($ ($ T : ident ,) +) => { $ (impl Serialize for num ::$ T { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . get () . serialize (serializer) } }) + } }
};
}
