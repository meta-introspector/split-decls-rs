macro_rules! deps {
    () => {
        Repr!();
        Decor!();
    };
}

macro_rules! Formatted {
    () => {
        deps!();
        # [doc = " A scalar TOML [`Value`][crate::Value]'s logical value and its representation in a `&str`"] # [doc = ""] # [doc = " This includes the surrounding whitespace and comments."] # [derive (Eq , PartialEq , Clone , Hash)] pub struct Formatted < T > { value : T , repr : Option < Repr > , decor : Decor , }
    };
}

Formatted!();