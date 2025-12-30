// Generated macro for VisitStaticStruct (enum)
macro_rules! DepcrateVisitStaticStruct {
() => {
// Module: crate
// Provides: {"VisitStaticStruct"}
// Dependencies: {}
enum VisitStaticStruct < S : Serializer > { Start { name : & 'static str , fields : Fields < 'static > , serializer : S , } , End (Result < S :: Ok , S :: Error >) , Tmp , }
};
}
