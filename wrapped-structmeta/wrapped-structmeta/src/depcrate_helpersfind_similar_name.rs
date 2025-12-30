// Generated macro for find_similar_name (function)
macro_rules! Depcrate_helpersfind_similar_name {
() => {
// Module: crate::helpers
// Provides: {"find_similar_name"}
// Dependencies: {}
fn find_similar_name < 'a > (names : & [& [& 'a str]] , ident : & Ident) -> Option < & 'a str > { let c0 : Vec < _ > = ident . to_string () . chars () . collect () ; let mut c1 = Vec :: new () ; let mut r = None ; let mut r_d = usize :: MAX ; for & names in names { for & name in names { c1 . clear () ; c1 . extend (name . chars ()) ; if let Some (d) = distance (& c0 , & c1) { if d < r_d { r_d = d ; r = Some (name) ; } if d == r_d && Some (name) != r { return None ; } } } } r }
};
}
