macro_rules! ImplPolarity {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum ImplPolarity { Positive , Negative , Reservation , }
    };
}

ImplPolarity!()