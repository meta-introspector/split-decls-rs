macro_rules! deps {
    () => {
        Error!();
        Function!();
        Result!();
    };
}

macro_rules! get_random {
    () => {
        deps!();
        # [cfg (feature = "builtins")] pub fn get_random (args : & HashMap < String , Value >) -> Result < Value > { let start = match args . get ("start") { Some (val) => match from_value :: < isize > (val . clone ()) { Ok (v) => v , Err (_) => { return Err (Error :: msg (format ! ("Function `get_random` received start={} but `start` can only be a number" , val))) ; } } , None => 0 , } ; let end = match args . get ("end") { Some (val) => match from_value :: < isize > (val . clone ()) { Ok (v) => v , Err (_) => { return Err (Error :: msg (format ! ("Function `get_random` received end={} but `end` can only be a number" , val))) ; } } , None => return Err (Error :: msg ("Function `get_random` didn't receive an `end` argument")) , } ; let mut rng = rand :: thread_rng () ; let res = rng . gen_range (start .. end) ; Ok (Value :: Number (res . into ())) }
    };
}

get_random!()