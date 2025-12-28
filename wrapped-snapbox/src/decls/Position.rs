macro_rules! deps {
    () => {
        Inline!();
    };
}

macro_rules! Position {
    () => {
        deps!();
        # [doc = " Position within Rust source code, see [`Inline`]"] # [doc (hidden)] # [derive (Clone , Debug , PartialEq , Eq)] pub struct Position { # [doc (hidden)] pub file : std :: path :: PathBuf , # [doc (hidden)] pub line : u32 , # [doc (hidden)] pub column : u32 , }
    };
}

Position!();