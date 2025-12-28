macro_rules! deps {
    () => {
        Val!();
        ForLoopValues!();
    };
}

macro_rules! impl_508 {
    () => {
        deps!();
        impl < 'a > ForLoopValues < 'a > { pub fn current_key (& self , i : usize) -> String { match * self { ForLoopValues :: Array (_) | ForLoopValues :: String (_) => { unreachable ! ("No key in array list or string") } ForLoopValues :: Object (ref values) => { values . get (i) . expect ("Failed getting current key") . 0 . clone () } } } pub fn current_value (& self , i : usize) -> Val < 'a > { match * self { ForLoopValues :: Array (ref values) => match * values { Cow :: Borrowed (v) => { Cow :: Borrowed (v . as_array () . expect ("Is array") . get (i) . expect ("Value")) } Cow :: Owned (_) => { Cow :: Owned (values . as_array () . expect ("Is array") . get (i) . expect ("Value") . clone ()) } } , ForLoopValues :: String (ref values) => { let mut graphemes = values . as_str () . expect ("Is string") . graphemes (true) ; Cow :: Owned (Value :: String (graphemes . nth (i) . expect ("Value") . to_string ())) } ForLoopValues :: Object (ref values) => values . get (i) . expect ("Value") . 1 . clone () , } } }
    };
}

impl_508!()