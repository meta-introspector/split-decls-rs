macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! linebreaksbr {
    () => {
        deps!();
        # [doc = " Convert line breaks (`\\n` or `\\r\\n`) to HTML linebreaks (`<br>`)."] # [doc = ""] # [doc = " Example: The input \"Hello\\nWorld\" turns into \"Hello<br>World\"."] pub fn linebreaksbr (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("linebreaksbr" , "value" , String , value) ; Ok (to_value (s . replace ("\r\n" , "<br>") . replace ('\n' , "<br>")) . unwrap ()) }
    };
}

linebreaksbr!();