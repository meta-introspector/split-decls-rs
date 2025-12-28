macro_rules! Movability {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Serialize)] pub enum Movability { Static , Movable , }
    };
}

Movability!()