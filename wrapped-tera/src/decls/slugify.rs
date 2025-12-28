macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! slugify {
    () => {
        deps!();
        # [doc = " Transform a string into a slug"] # [cfg (feature = "builtins")] pub fn slugify (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("slugify" , "value" , String , value) ; Ok (to_value (slug :: slugify (s)) . unwrap ()) }
    };
}

slugify!();