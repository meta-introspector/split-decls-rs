macro_rules! deps {
    () => {
        RawString!();
    };
}

macro_rules! Decor {
    () => {
        deps!();
        # [doc = " A prefix and suffix,"] # [doc = ""] # [doc = " Including comments, whitespaces and newlines."] # [derive (Eq , PartialEq , Clone , Default , Hash)] pub struct Decor { prefix : Option < RawString > , suffix : Option < RawString > , }
    };
}

Decor!()