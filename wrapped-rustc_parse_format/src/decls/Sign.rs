macro_rules! Sign {
    () => {
        # [doc = " Enum for the sign flags."] # [derive (Copy , Clone , Debug , PartialEq)] pub enum Sign { # [doc = " The `+` flag."] Plus , # [doc = " The `-` flag."] Minus , }
    };
}

Sign!();