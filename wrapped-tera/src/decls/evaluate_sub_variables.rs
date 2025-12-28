macro_rules! deps {
    () => {
        CallStack!();
        Result!();
        Error!();
        Tera!();
    };
}

macro_rules! evaluate_sub_variables {
    () => {
        deps!();
        # [doc = " This will convert a Tera variable to a json pointer if it is possible by replacing"] # [doc = " the index with their evaluated stringified value"] fn evaluate_sub_variables (key : & str , call_stack : & CallStack) -> Result < String > { let sub_vars_to_calc = pull_out_square_bracket (key) ; let mut new_key = key . to_string () ; for sub_var in & sub_vars_to_calc { match process_path (sub_var . as_ref () , call_stack) { Err (e) => { return Err (Error :: msg (format ! ("Variable {} can not be evaluated because: {}" , key , e))) ; } Ok (post_var) => { let post_var_as_str = match * post_var { Value :: String (ref s) => format ! (r#""{}""# , s) , Value :: Number (ref n) => n . to_string () , _ => { return Err (Error :: msg (format ! ("Only variables evaluating to String or Number can be used as \
                             index (`{}` of `{}`)" , sub_var , key ,))) ; } } ; let nk = new_key . clone () ; let divider = "[" . to_string () + sub_var + "]" ; let mut the_parts = nk . splitn (2 , divider . as_str ()) ; new_key = the_parts . next () . unwrap () . to_string () + "." + post_var_as_str . as_ref () + the_parts . next () . unwrap_or ("") ; } } } Ok (new_key . replace ('/' , "~1") . replace ("['" , ".\"") . replace ("[\"" , ".\"") . replace ('[' , ".") . replace ("']" , "\"") . replace ("\"]" , "\"") . replace (']' , "")) }
    };
}

evaluate_sub_variables!();