macro_rules! deps {
    () => {
        LineRange!();
    };
}

macro_rules! Snippet {
    () => {
        deps!();
        # [doc = " Represents code that will get replaced."] # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct Snippet { pub file_name : String , pub line_range : LineRange , pub range : Range < usize > , }
    };
}

Snippet!();