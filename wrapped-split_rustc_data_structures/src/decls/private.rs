macro_rules! private {
    () => {
        mod private { # [derive (Clone , Copy , Debug)] pub struct PrivateZst ; }
    };
}

private!()