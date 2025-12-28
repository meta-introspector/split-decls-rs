macro_rules! deps {
    () => {
        Val!();
    };
}

macro_rules! ForLoopValues {
    () => {
        deps!();
        # [doc = " Enumerates on the types of values to be iterated, scalars and pairs"] # [derive (Debug)] pub enum ForLoopValues < 'a > { # [doc = " Values for an array style iteration"] Array (Val < 'a >) , # [doc = " Values for a per-character iteration on a string"] String (Val < 'a >) , # [doc = " Values for an object style iteration"] Object (Vec < (String , Val < 'a >) >) , }
    };
}

ForLoopValues!();