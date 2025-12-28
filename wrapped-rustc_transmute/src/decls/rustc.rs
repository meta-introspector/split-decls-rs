macro_rules! deps {
    () => {
        Assume!();
        MaybeTransmutableQuery!();
        Answer!();
        Region!();
    };
}

macro_rules! rustc {
    () => {
        deps!();
        # [cfg (feature = "rustc")] mod rustc { use rustc_hir :: lang_items :: LangItem ; use rustc_middle :: ty :: { Const , Region , Ty , TyCtxt } ; use super :: * ; # [doc = " The source and destination types of a transmutation."] # [derive (Debug , Clone , Copy)] pub struct Types < 'tcx > { # [doc = " The source type."] pub src : Ty < 'tcx > , # [doc = " The destination type."] pub dst : Ty < 'tcx > , } pub struct TransmuteTypeEnv < 'tcx > { tcx : TyCtxt < 'tcx > , } impl < 'tcx > TransmuteTypeEnv < 'tcx > { pub fn new (tcx : TyCtxt < 'tcx >) -> Self { Self { tcx } } pub fn is_transmutable (& mut self , types : Types < 'tcx > , assume : crate :: Assume ,) -> crate :: Answer < Region < 'tcx > , Ty < 'tcx > > { crate :: maybe_transmutable :: MaybeTransmutableQuery :: new (types . src , types . dst , assume , self . tcx ,) . answer () } } impl Assume { # [doc = " Constructs an `Assume` from a given const-`Assume`."] pub fn from_const < 'tcx > (tcx : TyCtxt < 'tcx > , ct : Const < 'tcx >) -> Option < Self > { use rustc_middle :: ty :: ScalarInt ; use rustc_span :: sym ; let Some (cv) = ct . try_to_value () else { return None ; } ; let adt_def = cv . ty . ty_adt_def () ? ; if ! tcx . is_lang_item (adt_def . did () , LangItem :: TransmuteOpts) { tcx . dcx () . delayed_bug (format ! ("The given `const` was not marked with the `{}` lang item." , LangItem :: TransmuteOpts . name ())) ; return Some (Self { alignment : true , lifetimes : true , safety : true , validity : true , }) ; } let variant = adt_def . non_enum_variant () ; let fields = cv . valtree . unwrap_branch () ; let get_field = | name | { let (field_idx , _) = variant . fields . iter () . enumerate () . find (| (_ , field_def) | name == field_def . name) . unwrap_or_else (| | panic ! ("There were no fields named `{name}`.")) ; fields [field_idx] . unwrap_leaf () == ScalarInt :: TRUE } ; Some (Self { alignment : get_field (sym :: alignment) , lifetimes : get_field (sym :: lifetimes) , safety : get_field (sym :: safety) , validity : get_field (sym :: validity) , }) } } }
    };
}

rustc!();