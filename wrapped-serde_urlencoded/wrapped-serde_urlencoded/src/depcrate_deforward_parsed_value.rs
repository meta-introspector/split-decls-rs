// Generated macro for forward_parsed_value (macro)
macro_rules! Depcrate_deforward_parsed_value {
() => {
// Module: crate::de
// Provides: {"forward_parsed_value"}
// Dependencies: {}
macro_rules ! forward_parsed_value { ($ ($ ty : ident => $ method : ident ,) *) => { $ (fn $ method < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor <'de > { match self . 0 . parse ::<$ ty > () { Ok (val) => val . into_deserializer () .$ method (visitor) , Err (e) => Err (de :: Error :: custom (e)) } }) * } }
};
}
