macro_rules! ForLoopState {
    () => {
        # [doc = " Enumerates the states of a for loop"] # [derive (Clone , Copy , Debug , PartialEq)] pub enum ForLoopState { # [doc = " State during iteration"] Normal , # [doc = " State on encountering *break* statement"] Break , # [doc = " State on encountering *continue* statement"] Continue , }
    };
}

ForLoopState!()