macro_rules! deps {
    () => {
        Error!();
        Result!();
        CallStack!();
        Val!();
    };
}

macro_rules! process_path {
    () => {
        deps!();
        fn process_path < 'a > (path : & str , call_stack : & CallStack < 'a >) -> Result < Val < 'a > > { if ! path . contains ('[') { match call_stack . lookup (path) { Some (v) => Ok (v) , None => Err (Error :: msg (format ! ("Variable `{}` not found in context while rendering '{}'" , path , call_stack . active_template () . name))) , } } else { let full_path = evaluate_sub_variables (path , call_stack) ? ; match call_stack . lookup (& full_path) { Some (v) => Ok (v) , None => Err (Error :: msg (format ! ("Variable `{}` not found in context while rendering '{}': \
                 the evaluated version was `{}`. Maybe the index is out of bounds?" , path , call_stack . active_template () . name , full_path ,))) , } } }
    };
}

process_path!();