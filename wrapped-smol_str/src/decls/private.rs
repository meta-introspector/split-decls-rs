macro_rules! private {
    () => {
        mod private { # [doc = " No downstream impls allowed."] pub trait Sealed { } impl Sealed for str { } }
    };
}

private!()