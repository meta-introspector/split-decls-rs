macro_rules! deps {
    () => {
        Ascii!();
    };
}

macro_rules! Encoding {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug)] enum Encoding < S > { Ascii (Ascii < S >) , Unicode (Unicode < S >) , }
    };
}

Encoding!()