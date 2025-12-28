macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! extract_string {
    () => {
        deps!();
        # [doc = " Helper function to extract string from an [`Option<Value>`] to remove boilerplate"] # [doc = " with tester error handling"] pub fn extract_string < 'a > (tester_name : & str , part : & str , value : Option < & 'a Value > ,) -> Result < & 'a str > { match value . and_then (Value :: as_str) { Some (s) => Ok (s) , None => Err (Error :: msg (format ! ("Tester `{}` was called {} that isn't a string" , tester_name , part))) , } }
    };
}

extract_string!();