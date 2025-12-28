macro_rules! ValueOrPlace {
    () => {
        # [doc = " Used as the result of an operand or r-value."] # [derive (Debug)] pub enum ValueOrPlace < V > { Value (V) , Place (PlaceIndex) , }
    };
}

ValueOrPlace!()