macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! title {
    () => {
        deps!();
        # [doc = " Capitalizes each word in the string"] pub fn title (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("title" , "value" , String , value) ; Ok (to_value (WORDS_RE . replace_all (& s , | caps : & Captures | { let first = caps ["first"] . to_uppercase () ; let rest = caps ["rest"] . to_lowercase () ; format ! ("{}{}" , first , rest) })) . unwrap ()) }
    };
}

title!()