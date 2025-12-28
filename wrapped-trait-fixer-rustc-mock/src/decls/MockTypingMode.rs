macro_rules! MockTypingMode {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum MockTypingMode { NonBodyAnalysis , }
    };
}

MockTypingMode!()