// Generated macro for VisitDynamic (enum)
macro_rules! DepcrateVisitDynamic {
() => {
// Module: crate
// Provides: {"VisitDynamic"}
// Dependencies: {}
enum VisitDynamic < 'a , S : Serializer > { NamedFields (& 'a mut S :: SerializeMap) , UnnamedFields (& 'a mut S :: SerializeSeq) , Error (S :: Error) , }
};
}
