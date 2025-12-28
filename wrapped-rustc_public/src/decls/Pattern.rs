macro_rules! deps {
    () => {
        TyConst!();
    };
}

macro_rules! Pattern {
    () => {
        deps!();
        # [doc = " Represents a pattern in the type system"] # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum Pattern { Range { start : Option < TyConst > , end : Option < TyConst > , include_end : bool } , }
    };
}

Pattern!()