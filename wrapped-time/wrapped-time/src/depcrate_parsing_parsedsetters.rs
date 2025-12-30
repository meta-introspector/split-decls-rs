// Generated macro for setters (macro)
macro_rules! Depcrate_parsing_parsedsetters {
() => {
// Module: crate::parsing::parsed
// Provides: {"setters"}
// Dependencies: {}
# [doc = " Generate setters based on the builders."] macro_rules ! setters { ($ ($ name : ident $ setter : ident $ builder : ident $ type : ty ;) *) => { $ (# [doc = concat ! ("Set the `" , stringify ! ($ name) , "` component.")] # [inline] pub fn $ setter (& mut self , value : $ type) -> Option < () > { * self = self .$ builder (value) ?; Some (()) }) * } ; }
};
}
