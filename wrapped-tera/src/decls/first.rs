macro_rules! deps {
    () => {
        If!();
        Result!();
    };
}

macro_rules! first {
    () => {
        deps!();
        # [doc = " Returns the first value of an array"] # [doc = " If the array is empty, returns empty string"] pub fn first (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let mut arr = try_get_value ! ("first" , "value" , Vec < Value >, value) ; if arr . is_empty () { Ok (to_value ("") . unwrap ()) } else { Ok (arr . swap_remove (0)) } }
    };
}

first!();