// Generated macro for VisitList (enum)
macro_rules! DepcrateVisitList {
() => {
// Module: crate
// Provides: {"VisitList"}
// Dependencies: {}
enum VisitList < 'a , S : Serializer > { Serializer (& 'a mut S :: SerializeSeq) , Error (S :: Error) , }
};
}
