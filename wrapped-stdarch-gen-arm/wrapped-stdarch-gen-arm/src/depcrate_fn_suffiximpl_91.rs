// Generated macro for impl_91 (impl)
macro_rules! Depcrate_fn_suffiximpl_91 {
() => {
// Module: crate::fn_suffix
// Provides: {"impl_91"}
// Dependencies: {}
impl FromStr for SuffixKind { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "no" => Ok (SuffixKind :: Normal) , "noq" => Ok (SuffixKind :: NoQ) , "N" => Ok (SuffixKind :: NSuffix) , "noq_N" => Ok (SuffixKind :: NoQNSuffix) , "dup_nox" => Ok (SuffixKind :: DupNox) , "dup" => Ok (SuffixKind :: Dup) , "lane" => Ok (SuffixKind :: Lane) , "base" => Ok (SuffixKind :: Base) , "tuple" => Ok (SuffixKind :: Tuple) , "rot270" => Ok (SuffixKind :: Rot270) , "rot270_lane" => Ok (SuffixKind :: Rot270Lane) , "rot270_laneq" => Ok (SuffixKind :: Rot270LaneQ) , "rot90" => Ok (SuffixKind :: Rot90) , "rot90_lane" => Ok (SuffixKind :: Rot90Lane) , "rot90_laneq" => Ok (SuffixKind :: Rot90LaneQ) , "rot180" => Ok (SuffixKind :: Rot180) , "rot180_lane" => Ok (SuffixKind :: Rot180LaneQ) , "rot180_laneq" => Ok (SuffixKind :: Rot180LaneQ) , "u" => Ok (SuffixKind :: Unsigned) , "nox" => Ok (SuffixKind :: NoX) , "base_byte_size" => Ok (SuffixKind :: BaseByteSize) , "lane_nox" => Ok (SuffixKind :: LaneNoX) , "laneq_nox" => Ok (SuffixKind :: LaneQNoX) , _ => Err (format ! ("unknown suffix type: {s}")) , } } }
};
}
