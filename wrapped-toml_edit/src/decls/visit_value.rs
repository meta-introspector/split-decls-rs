macro_rules! deps {
    () => {
        InlineTable!();
        Value!();
        Array!();
        Visit!();
    };
}

macro_rules! visit_value {
    () => {
        deps!();
        pub fn visit_value < 'doc , V > (v : & mut V , node : & 'doc Value) where V : Visit < 'doc > + ? Sized , { match node { Value :: String (s) => v . visit_string (s) , Value :: Integer (i) => v . visit_integer (i) , Value :: Float (f) => v . visit_float (f) , Value :: Boolean (b) => v . visit_boolean (b) , Value :: Datetime (dt) => v . visit_datetime (dt) , Value :: Array (array) => v . visit_array (array) , Value :: InlineTable (table) => v . visit_inline_table (table) , } }
    };
}

visit_value!();