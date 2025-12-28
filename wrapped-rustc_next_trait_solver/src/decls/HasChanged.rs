macro_rules! HasChanged {
    () => {
        # [doc = " Whether evaluating this goal ended up changing the"] # [doc = " inference state."] # [derive (PartialEq , Eq , Debug , Hash , Clone , Copy)] pub enum HasChanged { Yes , No , }
    };
}

HasChanged!()