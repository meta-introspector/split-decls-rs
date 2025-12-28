macro_rules! deps {
    () => {
        Result!();
        Error!();
        Function!();
    };
}

macro_rules! range {
    () => {
        deps!();
        pub fn range (args : & HashMap < String , Value >) -> Result < Value > { let start = match args . get ("start") { Some (val) => match from_value :: < usize > (val . clone ()) { Ok (v) => v , Err (_) => { return Err (Error :: msg (format ! ("Function `range` received start={} but `start` can only be a number" , val))) ; } } , None => 0 , } ; let step_by = match args . get ("step_by") { Some (val) => match from_value :: < usize > (val . clone ()) { Ok (v) => v , Err (_) => { return Err (Error :: msg (format ! ("Function `range` received step_by={} but `step` can only be a number" , val))) ; } } , None => 1 , } ; let end = match args . get ("end") { Some (val) => match from_value :: < usize > (val . clone ()) { Ok (v) => v , Err (_) => { return Err (Error :: msg (format ! ("Function `range` received end={} but `end` can only be a number" , val))) ; } } , None => { return Err (Error :: msg ("Function `range` was called without a `end` argument")) ; } } ; if start > end { return Err (Error :: msg ("Function `range` was called with a `start` argument greater than the `end` one" ,)) ; } let mut i = start ; let mut res = vec ! [] ; while i < end { res . push (i) ; i += step_by ; } Ok (to_value (res) . unwrap ()) }
    };
}

range!()