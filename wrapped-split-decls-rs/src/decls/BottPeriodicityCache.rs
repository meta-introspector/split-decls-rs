// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "BottPeriodicityCache",
decl_type: "function",
source_file: "./src/bott_periodicity.rs",
source_crate: ".",
deps: ["AbstractionBundle"],
uses: ["BottPeriodicityCache", "AbstractionBundle", "Clone", "Cache", "Vec", "Debug", "Bott", "HashMap", "String"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        AbstractionBundle!();
    };
}

macro_rules! BottPeriodicityCache {
    () => {
        deps!();
        # [doc = " Cache for Bott periodicity computations"] # [derive (Debug , Clone)] pub struct BottPeriodicityCache { cache : std :: collections :: HashMap < usize , AbstractionBundle > , pub current_generation : usize , pub levels : Vec < AbstractionBundle > , pub fiber_bundles : Vec < String > , }
    };
}

BottPeriodicityCache!();