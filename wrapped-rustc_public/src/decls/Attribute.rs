macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! Attribute {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub struct Attribute { value : String , span : Span , }
    };
}

Attribute!();