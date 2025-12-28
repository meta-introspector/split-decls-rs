macro_rules! IdentIsRaw {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum IdentIsRaw { No , Yes , }
    };
}

IdentIsRaw!()