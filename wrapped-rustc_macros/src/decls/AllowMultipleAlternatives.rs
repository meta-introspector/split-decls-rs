macro_rules! AllowMultipleAlternatives {
    () => {
        # [derive (Copy , Clone , Debug)] pub (super) enum AllowMultipleAlternatives { No , Yes , }
    };
}

AllowMultipleAlternatives!()