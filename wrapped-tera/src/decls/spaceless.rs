macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! spaceless {
    () => {
        deps!();
        # [doc = " Removes spaces between html tags from string"] pub fn spaceless (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("spaceless" , "value" , String , value) ; Ok (to_value (SPACELESS_RE . replace_all (& s , "><")) . unwrap ()) }
    };
}

spaceless!()