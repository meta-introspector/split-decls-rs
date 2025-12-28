macro_rules! Position {
    () => {
        # [doc = " Enum describing where an argument for a format can be located."] # [derive (Clone , Debug , PartialEq)] pub enum Position < 'input > { # [doc = " The argument is implied to be located at an index"] ArgumentImplicitlyIs (usize) , # [doc = " The argument is located at a specific index given in the format,"] ArgumentIs (usize) , # [doc = " The argument has a name."] ArgumentNamed (& 'input str) , }
    };
}

Position!();