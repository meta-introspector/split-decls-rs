// Generated macro for normalize_bounds (function)
macro_rules! Depcratenormalize_bounds {
() => {
// Module: crate
// Provides: {"normalize_bounds"}
// Dependencies: {}
# [doc = " Normalizes a slice of bounds by replacing [`TraitBound::Slf`] with `slf`."] fn normalize_bounds (slf : Trait , bounds : & [TraitBound]) -> impl '_ + Iterator < Item = Trait > { bounds . iter () . map (move | bound | match bound { TraitBound :: Slf => slf , TraitBound :: Other (trt) => * trt , }) }
};
}
