macro_rules! deps {
    () => {
        ForLoop!();
    };
}

macro_rules! FrameType {
    () => {
        deps!();
        # [doc = " Enumerates the types of stack frames"] # [derive (Clone , Copy , Debug , PartialEq)] pub enum FrameType { # [doc = " Original frame"] Origin , # [doc = " New frame for macro call"] Macro , # [doc = " New frame for for loop"] ForLoop , # [doc = " Include template"] Include , }
    };
}

FrameType!()