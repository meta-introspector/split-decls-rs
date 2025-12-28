macro_rules! deps {
    () => {
        Cx!();
        NestedGoals!();
    };
}

macro_rules! CacheData {
    () => {
        deps!();
        # [derive_where (Debug ; X : Cx)] pub (super) struct CacheData < 'a , X : Cx > { pub (super) result : X :: Result , pub (super) required_depth : usize , pub (super) encountered_overflow : bool , pub (super) nested_goals : & 'a NestedGoals < X > , }
    };
}

CacheData!();