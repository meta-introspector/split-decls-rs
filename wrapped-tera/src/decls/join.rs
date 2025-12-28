macro_rules! deps {
    () => {
        Error!();
        Result!();
        If!();
    };
}

macro_rules! join {
    () => {
        deps!();
        # [doc = " Joins all values in the array by the `sep` argument given"] # [doc = " If no separator is given, it will use `\"\"` (empty string) as separator"] # [doc = " If the array is empty, returns empty string"] pub fn join (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let arr = try_get_value ! ("join" , "value" , Vec < Value >, value) ; let sep = match args . get ("sep") { Some (val) => { let s = try_get_value ! ("truncate" , "sep" , String , val) ; s . replace ("\\n" , "\n") . replace ("\\t" , "\t") } None => String :: new () , } ; let rendered = arr . iter () . map (| v | render_to_string (| | "joining array" . to_string () , | w | v . render (w))) . collect :: < Result < Vec < _ > > > () ? ; to_value (rendered . join (& sep)) . map_err (Error :: json) }
    };
}

join!()