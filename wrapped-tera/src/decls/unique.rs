macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! unique {
    () => {
        deps!();
        # [doc = " Remove duplicates from an array."] # [doc = " Use the 'attribute' argument to define a field to filter on."] # [doc = " For strings, use the 'case_sensitive' argument (defaults to false) to control the comparison."] pub fn unique (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let arr = try_get_value ! ("unique" , "value" , Vec < Value >, value) ; if arr . is_empty () { return Ok (arr . into ()) ; } let case_sensitive = match args . get ("case_sensitive") { Some (val) => try_get_value ! ("unique" , "case_sensitive" , bool , val) , None => false , } ; let attribute = match args . get ("attribute") { Some (val) => try_get_value ! ("unique" , "attribute" , String , val) , None => String :: new () , } ; let first = dotted_pointer (& arr [0] , & attribute) . ok_or_else (| | { Error :: msg (format ! ("attribute '{}' does not reference a field" , attribute)) }) ? ; let disc = std :: mem :: discriminant (first) ; let mut strategy = get_unique_strategy_for_type (first , case_sensitive) ? ; let arr = arr . into_iter () . filter_map (| v | match dotted_pointer (& v , & attribute) { Some (key) => { if disc == std :: mem :: discriminant (key) { match strategy . insert (key) { Ok (false) => None , Ok (true) => Some (Ok (v)) , Err (e) => Some (Err (e)) , } } else { Some (Err (Error :: msg ("unique filter can't compare multiple types"))) } } None => None , }) . collect :: < Result < Vec < _ > > > () ; Ok (to_value (arr ?) . unwrap ()) }
    };
}

unique!()