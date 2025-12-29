// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_150",
decl_type: "function",
source_file: "./src/bott_periodicity.rs",
source_crate: ".",
deps: ["BottLevel"],
uses: ["Three", "Six", "Zero", "BottLevel", "Five", "Two", "One", "Four", "Seven"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        BottLevel!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl BottLevel { pub fn from_n (n : usize) -> Self { match n % 8 { 0 => BottLevel :: Zero , 1 => BottLevel :: One , 2 => BottLevel :: Two , 3 => BottLevel :: Three , 4 => BottLevel :: Four , 5 => BottLevel :: Five , 6 => BottLevel :: Six , 7 => BottLevel :: Seven , _ => unreachable ! () , } } pub fn dimension (& self) -> usize { match self { BottLevel :: Zero => 0 , BottLevel :: One => 1 , BottLevel :: Two => 0 , BottLevel :: Three => 1 , BottLevel :: Four => 0 , BottLevel :: Five => 1 , BottLevel :: Six => 0 , BottLevel :: Seven => 1 , } } }
    };
}

impl_150!();