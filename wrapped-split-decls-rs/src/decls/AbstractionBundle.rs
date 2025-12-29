// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "AbstractionBundle",
decl_type: "function",
source_file: "./src/bott_periodicity.rs",
source_crate: ".",
deps: ["AbstractionContent", "BottLevel"],
uses: ["Debug", "AbstractionBundle", "The", "Vec", "AbstractionContent", "Characteristic", "An", "Absolute", "Bott", "Clone", "BottLevel", "Which"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        AbstractionContent!();
        BottLevel!();
    };
}

macro_rules! AbstractionBundle {
    () => {
        deps!();
        # [doc = " An abstraction at level n in the periodic tower"] # [derive (Debug , Clone)] pub struct AbstractionBundle { # [doc = " Which level (mod 8) in the Bott tower"] pub bott_level : BottLevel , # [doc = " Absolute level (how many times we've gone around)"] pub winding_number : usize , # [doc = " The actual content (shape repeats, but \"meaning\" differs)"] pub content : AbstractionContent , # [doc = " Characteristic classes (topological invariants)"] pub chern_classes : Vec < i32 > , }
    };
}

AbstractionBundle!();