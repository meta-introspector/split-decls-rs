// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_156",
decl_type: "function",
source_file: "./src/bott_periodicity.rs",
source_crate: ".",
deps: ["BottMap", "AbstractionBundle", "AbstractionContent", "SuspensionTower"],
uses: ["MetaPattern", "BottMap", "Build", "TokenStream", "AbstractionBundle", "Suspend", "Level", "Generate", "AbstractionContent", "SuspensionTower", "Concrete", "Pattern"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        BottMap!();
        AbstractionBundle!();
        AbstractionContent!();
        SuspensionTower!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl SuspensionTower { pub fn new (base : AbstractionBundle) -> Self { Self { levels : vec ! [base] , } } # [doc = " Suspend: go up one level in abstraction"] pub fn suspend (& mut self) { let current = self . levels . last () . unwrap () . clone () ; let suspended = BottMap :: apply (current) ; self . levels . push (suspended) ; } # [doc = " Build full 8-level tower"] pub fn build_full_period (& mut self) { while self . levels . len () < 8 { self . suspend () ; } } # [doc = " Generate macro at specific level"] pub fn generate_macro_at_level (& self , level : usize) -> TokenStream { if level >= self . levels . len () { return quote ! { compile_error ! ("Level not built yet") ; } ; } let bundle = & self . levels [level] ; match & bundle . content { AbstractionContent :: Concrete (tokens) => tokens . clone () , AbstractionContent :: Pattern { template : _ , fiber_dim : _ } => { quote ! { macro_rules ! pattern_macro { ($ ($ args : tt) *) => { $ ($ args) * } ; } } } AbstractionContent :: MetaPattern { meta_structure : _ } => { quote ! { macro_rules ! meta_pattern_macro { ($ ($ args : tt) *) => { $ ($ args) * } ; } } } _ => quote ! { } , } } }
    };
}

impl_156!();