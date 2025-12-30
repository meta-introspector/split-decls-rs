// Generated macro for Container (struct)
macro_rules! Depcrate_internals_attrContainer {
() => {
// Module: crate::internals::attr
// Provides: {"Container"}
// Dependencies: {}
# [doc = " Represents struct or enum attribute information."] pub struct Container { name : MultiName , transparent : bool , deny_unknown_fields : bool , default : Default , rename_all_rules : RenameAllRules , rename_all_fields_rules : RenameAllRules , ser_bound : Option < Vec < syn :: WherePredicate > > , de_bound : Option < Vec < syn :: WherePredicate > > , tag : TagType , type_from : Option < syn :: Type > , type_try_from : Option < syn :: Type > , type_into : Option < syn :: Type > , remote : Option < syn :: Path > , identifier : Identifier , serde_path : Option < syn :: Path > , is_packed : bool , # [doc = " Error message generated when type can't be deserialized"] expecting : Option < String > , non_exhaustive : bool , }
};
}
