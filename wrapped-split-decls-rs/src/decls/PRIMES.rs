// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "PRIMES",
decl_type: "function",
source_file: "./src/macro_analyzer_parts/scoring.rs",
source_crate: ".",
deps: [],
uses: ["PRIMES"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! PRIMES {
    () => {
        pub const PRIMES : & [usize] = & [2 , 3 , 5 , 7 , 11 , 13 , 17 , 19 , 23 , 29 , 31 , 37 , 41 , 43 , 47 , 53 , 59 , 61 , 67 , 71] ;
    };
}

PRIMES!();