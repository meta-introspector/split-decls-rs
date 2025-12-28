macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! sort {
    () => {
        deps!();
        # [doc = " Sorts the array in ascending order."] # [doc = " Use the 'attribute' argument to define a field to sort by."] pub fn sort (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let arr = try_get_value ! ("sort" , "value" , Vec < Value >, value) ; if arr . is_empty () { return Ok (arr . into ()) ; } let attribute = match args . get ("attribute") { Some (val) => try_get_value ! ("sort" , "attribute" , String , val) , None => String :: new () , } ; let first = dotted_pointer (& arr [0] , & attribute) . ok_or_else (| | { Error :: msg (format ! ("attribute '{}' does not reference a field" , attribute)) }) ? ; let mut strategy = get_sort_strategy_for_type (first) ? ; for v in & arr { let key = dotted_pointer (v , & attribute) . ok_or_else (| | { Error :: msg (format ! ("attribute '{}' does not reference a field" , attribute)) }) ? ; strategy . try_add_pair (v , key) ? ; } let sorted = strategy . sort () ; Ok (sorted . into ()) }
    };
}

sort!()