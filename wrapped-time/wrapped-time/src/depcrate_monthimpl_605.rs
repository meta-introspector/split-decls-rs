// Generated macro for impl_605 (impl)
macro_rules! Depcrate_monthimpl_605 {
() => {
// Module: crate::month
// Provides: {"impl_605"}
// Dependencies: {}
impl FromStr for Month { type Err = error :: InvalidVariant ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "January" => Ok (January) , "February" => Ok (February) , "March" => Ok (March) , "April" => Ok (April) , "May" => Ok (May) , "June" => Ok (June) , "July" => Ok (July) , "August" => Ok (August) , "September" => Ok (September) , "October" => Ok (October) , "November" => Ok (November) , "December" => Ok (December) , _ => Err (error :: InvalidVariant) , } } }
};
}
