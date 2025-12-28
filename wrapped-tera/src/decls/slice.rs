macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! slice {
    () => {
        deps!();
        # [doc = " Slice the array"] # [doc = " Use the `start` argument to define where to start (inclusive, default to `0`)"] # [doc = " and `end` argument to define where to stop (exclusive, default to the length of the array)"] # [doc = " `start` and `end` are 0-indexed"] pub fn slice (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let arr = try_get_value ! ("slice" , "value" , Vec < Value >, value) ; if arr . is_empty () { return Ok (arr . into ()) ; } let start = match args . get ("start") { Some (val) => get_index (try_get_value ! ("slice" , "start" , f64 , val) , & arr) , None => 0 , } ; let mut end = match args . get ("end") { Some (val) => get_index (try_get_value ! ("slice" , "end" , f64 , val) , & arr) , None => arr . len () , } ; if end > arr . len () { end = arr . len () ; } if start >= end { return Ok (Vec :: < Value > :: new () . into ()) ; } Ok (arr [start .. end] . into ()) }
    };
}

slice!()