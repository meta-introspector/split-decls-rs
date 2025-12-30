// Generated macro for SynToken (enum)
macro_rules! DepcrateSynToken {
() => {
// Module: crate
// Provides: {"SynToken"}
// Dependencies: {}
# [derive (Debug)] enum SynToken < S > { Ordinary (SyntaxToken) , Punct { token : SyntaxToken , offset : usize } , Leaf (tt :: Leaf < S >) , }
};
}
