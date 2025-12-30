// Generated macro for visit_value (function)
macro_rules! Depcrate_visitvisit_value {
() => {
// Module: crate::visit
// Provides: {"visit_value"}
// Dependencies: {}
pub fn visit_value < 'doc , V > (v : & mut V , node : & 'doc Value) where V : Visit < 'doc > + ? Sized , { match node { Value :: String (s) => v . visit_string (s) , Value :: Integer (i) => v . visit_integer (i) , Value :: Float (f) => v . visit_float (f) , Value :: Boolean (b) => v . visit_boolean (b) , Value :: Datetime (dt) => v . visit_datetime (dt) , Value :: Array (array) => v . visit_array (array) , Value :: InlineTable (table) => v . visit_inline_table (table) , } }
};
}
