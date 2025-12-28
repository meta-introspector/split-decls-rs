macro_rules! Height {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] pub struct Height (pub u16) ;
    };
}

Height!()