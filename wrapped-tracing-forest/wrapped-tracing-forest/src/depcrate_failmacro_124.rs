// Generated macro for macro_124 (macro)
macro_rules! Depcrate_failmacro_124 {
() => {
// Module: crate::fail
// Provides: {"macro_124"}
// Dependencies: {}
cfg_uuid ! { pub const NO_CURRENT_SPAN : & str = "The subscriber isn't in any spans" ; pub const NO_FOREST_LAYER : & str = "The span has no `Span` in extensions, perhaps you forgot to add a `ForestLayer` to your subscriber?" ; # [cold] # [inline (never)] pub fn subscriber_not_found <'a , S > () -> &'a S { panic ! ("Subscriber could not be downcasted to `{}`" , std :: any :: type_name ::< S > ()) ; } }
};
}
