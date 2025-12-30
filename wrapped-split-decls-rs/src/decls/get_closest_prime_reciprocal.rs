// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "get_closest_prime_reciprocal",
decl_type: "function",
source_file: "./src/macro_analyzer_parts/scoring.rs",
source_crate: ".",
deps: [],
uses: ["PRIMES", "MAX"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! get_closest_prime_reciprocal {
    () => {
        pub fn get_closest_prime_reciprocal (relative_frequency : f64) -> f64 { if relative_frequency == 0.0 { return 0.0 ; } if relative_frequency >= 1.0 { return 1.0 ; } let mut closest_score = 0.0 ; let mut min_diff = f64 :: MAX ; for & p in PRIMES { let score = 1.0 / (p as f64) ; let diff = (relative_frequency - score) . abs () ; if diff < min_diff { min_diff = diff ; closest_score = score ; } } closest_score }
    };
}

get_closest_prime_reciprocal!();