macro_rules! deps {
    () => {
        Condition!();
        Reason!();
    };
}

macro_rules! Answer {
    () => {
        deps!();
        # [doc = " Either transmutation is allowed, we have an error, or we have an optional"] # [doc = " Condition that must hold."] # [derive (Debug , Hash , Eq , PartialEq , Clone)] pub enum Answer < R , T > { Yes , No (Reason < T >) , If (Condition < R , T >) , }
    };
}

Answer!();