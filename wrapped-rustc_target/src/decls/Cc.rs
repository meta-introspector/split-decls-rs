macro_rules! Cc {
    () => {
        # [doc = " Linker is called through a C/C++ compiler."] # [derive (Clone , Copy , Debug , Eq , Ord , PartialEq , PartialOrd)] pub enum Cc { Yes , No , }
    };
}

Cc!()