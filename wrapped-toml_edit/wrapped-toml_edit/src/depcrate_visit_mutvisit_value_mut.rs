// Generated macro for visit_value_mut (function)
macro_rules! Depcrate_visit_mutvisit_value_mut {
() => {
// Module: crate::visit_mut
// Provides: {"visit_value_mut"}
// Dependencies: {}
pub fn visit_value_mut < V > (v : & mut V , node : & mut Value) where V : VisitMut + ? Sized , { match node { Value :: String (s) => v . visit_string_mut (s) , Value :: Integer (i) => v . visit_integer_mut (i) , Value :: Float (f) => v . visit_float_mut (f) , Value :: Boolean (b) => v . visit_boolean_mut (b) , Value :: Datetime (dt) => v . visit_datetime_mut (dt) , Value :: Array (array) => v . visit_array_mut (array) , Value :: InlineTable (table) => v . visit_inline_table_mut (table) , } }
};
}
