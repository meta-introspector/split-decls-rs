macro_rules! deps {
    () => {
        ForLoopKind!();
        Val!();
        Forloop!();
        ForLoopState!();
        ForLoop!();
        ForLoopValues!();
    };
}

macro_rules! impl_510 {
    () => {
        deps!();
        impl < 'a > ForLoop < 'a > { pub fn from_array (value_name : & str , values : Val < 'a >) -> Self { ForLoop { key_name : None , value_name : value_name . to_string () , current : 0 , values : ForLoopValues :: Array (values) , kind : ForLoopKind :: Value , state : ForLoopState :: Normal , } } pub fn from_string (value_name : & str , values : Val < 'a >) -> Self { ForLoop { key_name : None , value_name : value_name . to_string () , current : 0 , values : ForLoopValues :: String (values) , kind : ForLoopKind :: Value , state : ForLoopState :: Normal , } } pub fn from_object (key_name : & str , value_name : & str , object : & 'a Value) -> Self { let object_values = object . as_object () . unwrap () ; let mut values = Vec :: with_capacity (object_values . len ()) ; for (k , v) in object_values { values . push ((k . to_string () , Cow :: Borrowed (v))) ; } ForLoop { key_name : Some (key_name . to_string ()) , value_name : value_name . to_string () , current : 0 , values : ForLoopValues :: Object (values) , kind : ForLoopKind :: KeyValue , state : ForLoopState :: Normal , } } pub fn from_object_owned (key_name : & str , value_name : & str , object : Value) -> Self { let object_values = match object { Value :: Object (c) => c , _ => unreachable ! ("Tried to create a Forloop from an object owned but it wasn't an object") , } ; let mut values = Vec :: with_capacity (object_values . len ()) ; for (k , v) in object_values { values . push ((k . to_string () , Cow :: Owned (v))) ; } ForLoop { key_name : Some (key_name . to_string ()) , value_name : value_name . to_string () , current : 0 , values : ForLoopValues :: Object (values) , kind : ForLoopKind :: KeyValue , state : ForLoopState :: Normal , } } # [inline] pub fn increment (& mut self) { self . current += 1 ; self . state = ForLoopState :: Normal ; } pub fn is_key_value (& self) -> bool { self . kind == ForLoopKind :: KeyValue } # [inline] pub fn break_loop (& mut self) { self . state = ForLoopState :: Break ; } # [inline] pub fn continue_loop (& mut self) { self . state = ForLoopState :: Continue ; } # [inline] pub fn get_current_value (& self) -> Val < 'a > { self . values . current_value (self . current) } # [doc = " Only called in `ForLoopKind::KeyValue`"] # [inline] pub fn get_current_key (& self) -> String { self . values . current_key (self . current) } # [doc = " Checks whether the key string given is the variable used as key for"] # [doc = " the current forloop"] pub fn is_key (& self , name : & str) -> bool { if self . kind == ForLoopKind :: Value { return false ; } if let Some (ref key_name) = self . key_name { return key_name == name ; } false } pub fn len (& self) -> usize { match self . values { ForLoopValues :: Array (ref values) => values . as_array () . expect ("Value is array") . len () , ForLoopValues :: String (ref values) => { values . as_str () . expect ("Value is string") . chars () . count () } ForLoopValues :: Object (ref values) => values . len () , } } }
    };
}

impl_510!();