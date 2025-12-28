macro_rules! deps {
    () => {
        Error!();
        If!();
        Result!();
        Filter!();
    };
}

macro_rules! filter {
    () => {
        deps!();
        # [doc = " Filter the array values, returning only the values where the `attribute` is equal to the `value`"] # [doc = " Values without the `attribute` or with a null `attribute` are discarded"] # [doc = " If the `value` is not passed, discard all elements where the attribute is null."] pub fn filter (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let mut arr = try_get_value ! ("filter" , "value" , Vec < Value >, value) ; if arr . is_empty () { return Ok (arr . into ()) ; } let key = match args . get ("attribute") { Some (val) => try_get_value ! ("filter" , "attribute" , String , val) , None => return Err (Error :: msg ("The `filter` filter has to have an `attribute` argument")) , } ; let value = args . get ("value") . unwrap_or (& Value :: Null) ; arr = arr . into_iter () . filter (| v | { let val = dotted_pointer (v , & key) . unwrap_or (& Value :: Null) ; if value . is_null () { ! val . is_null () } else { val == value } }) . collect :: < Vec < _ > > () ; Ok (to_value (arr) . unwrap ()) }
    };
}

filter!();