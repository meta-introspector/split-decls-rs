macro_rules! Identity {
    () => {
        # [doc = " A layer that does nothing."] # [derive (Clone , Debug , Default)] pub struct Identity { _p : () , }
    };
}

Identity!()