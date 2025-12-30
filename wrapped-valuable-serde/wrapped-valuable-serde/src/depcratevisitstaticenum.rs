// Generated macro for VisitStaticEnum (enum)
macro_rules! DepcrateVisitStaticEnum {
() => {
// Module: crate
// Provides: {"VisitStaticEnum"}
// Dependencies: {}
enum VisitStaticEnum < S : Serializer > { Start { name : & 'static str , def : & 'static [VariantDef < 'static >] , variant : & 'static VariantDef < 'static > , serializer : S , } , End (Result < S :: Ok , S :: Error >) , Tmp , }
};
}
