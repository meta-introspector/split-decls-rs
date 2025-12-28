macro_rules! Round {
    () => {
        # [derive (Default)] struct Round { samples : Vec < (f64 , f64) > , plot : String , linear : bool , }
    };
}

Round!();