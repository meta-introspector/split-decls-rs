// Generated macro for impl_61 (impl)
macro_rules! Depcrate_editionimpl_61 {
() => {
// Module: crate::edition
// Provides: {"impl_61"}
// Dependencies: {}
impl FromStr for Edition { type Err = () ; fn from_str (s : & str) -> Result < Self , () > { match s { "2015" => Ok (Edition :: Edition2015) , "2018" => Ok (Edition :: Edition2018) , "2021" => Ok (Edition :: Edition2021) , "2024" => Ok (Edition :: Edition2024) , "future" => Ok (Edition :: EditionFuture) , _ => Err (()) , } } }
};
}
