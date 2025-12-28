macro_rules! deps {
    () => {
        VisitMut!();
        Array!();
        Value!();
        InlineTable!();
    };
}

macro_rules! visit_value_mut {
    () => {
        deps!();
        pub fn visit_value_mut < V > (v : & mut V , node : & mut Value) where V : VisitMut + ? Sized , { match node { Value :: String (s) => v . visit_string_mut (s) , Value :: Integer (i) => v . visit_integer_mut (i) , Value :: Float (f) => v . visit_float_mut (f) , Value :: Boolean (b) => v . visit_boolean_mut (b) , Value :: Datetime (dt) => v . visit_datetime_mut (dt) , Value :: Array (array) => v . visit_array_mut (array) , Value :: InlineTable (table) => v . visit_inline_table_mut (table) , } }
    };
}

visit_value_mut!();