// Generated macro for VisitStaticTuple (enum)
macro_rules! DepcrateVisitStaticTuple {
() => {
// Module: crate
// Provides: {"VisitStaticTuple"}
// Dependencies: {}
enum VisitStaticTuple < S : Serializer > { Start (S :: SerializeTuple) , End (Result < S :: Ok , S :: Error >) , Tmp , }
};
}
