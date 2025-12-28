macro_rules! deps {
    () => {
        MemoIngredientIndices!();
        SalsaStructInDb!();
        Memo!();
        HashEqLike!();
        Accumulator!();
        NewMemoIngredientIndices!();
        Database!();
        JarKind!();
        AsId!();
        IngredientImpl!();
        Runtime!();
        Durability!();
        Jar!();
        Ingredient!();
        HasBuilder!();
        HasStorage!();
        ErasedJar!();
        Id!();
        JarImpl!();
        Location!();
        MemoIngredientMap!();
        Revision!();
        FromId!();
        Update!();
        HasJar!();
        DatabaseKeyIndex!();
        Configuration!();
        MemoIngredientSingletonIndex!();
        IngredientIndices!();
        Storage!();
        ZalsaLocal!();
        ZalsaDatabase!();
        Zalsa!();
        TrackedStructInDb!();
        DatabaseDownCaster!();
        Stamp!();
        FromIdWithDb!();
        IngredientIndex!();
        CycleRecoveryStrategy!();
        Value!();
        Lookup!();
    };
}

macro_rules! plumbing {
    () => {
        deps!();
        # [doc = " Internal names used by salsa macros."] # [doc = ""] # [doc = " # WARNING"] # [doc = ""] # [doc = " The contents of this module are NOT subject to semver."] # [doc (hidden)] pub mod plumbing { pub use std :: any :: TypeId ; pub use std :: option :: Option :: { self , None , Some } ; # [cfg (feature = "accumulator")] pub use salsa_macro_rules :: setup_accumulator_impl ; pub use salsa_macro_rules :: { gate_accumulated , macro_if , maybe_backdate , maybe_default , maybe_default_tt , return_mode_expression , return_mode_ty , setup_input_struct , setup_interned_struct , setup_tracked_assoc_fn_body , setup_tracked_fn , setup_tracked_method_body , setup_tracked_struct , unexpected_cycle_initial , unexpected_cycle_recovery , } ; # [cfg (feature = "accumulator")] pub use crate :: accumulator :: Accumulator ; pub use crate :: attach :: { attach , with_attached_database } ; pub use crate :: cycle :: CycleRecoveryStrategy ; pub use crate :: database :: { Database , current_revision } ; pub use crate :: durability :: Durability ; pub use crate :: id :: { AsId , FromId , FromIdWithDb , Id } ; pub use crate :: ingredient :: { Ingredient , Jar , Location } ; pub use crate :: ingredient_cache :: IngredientCache ; pub use crate :: key :: DatabaseKeyIndex ; pub use crate :: memo_ingredient_indices :: { IngredientIndices , MemoIngredientIndices , MemoIngredientMap , MemoIngredientSingletonIndex , NewMemoIngredientIndices , } ; pub use crate :: revision :: Revision ; pub use crate :: runtime :: { Runtime , Stamp , stamp } ; pub use crate :: salsa_struct :: SalsaStructInDb ; pub use crate :: storage :: { HasStorage , Storage } ; pub use crate :: table :: memo :: MemoTableWithTypes ; pub use crate :: tracked_struct :: TrackedStructInDb ; pub use crate :: update :: helper :: { Dispatch as UpdateDispatch , Fallback as UpdateFallback } ; pub use crate :: update :: { Update , always_update } ; pub use crate :: views :: DatabaseDownCaster ; pub use crate :: zalsa :: { ErasedJar , HasJar , IngredientIndex , JarKind , Zalsa , ZalsaDatabase , register_jar , transmute_data_ptr , views , } ; pub use crate :: zalsa_local :: ZalsaLocal ; # [cfg (feature = "persistence")] pub use serde ; # [cfg (not (feature = "persistence"))] pub mod serde { pub trait Serializer { type Ok ; type Error ; } pub trait Deserializer < 'de > { type Ok ; type Error ; } } # [cfg (feature = "accumulator")] pub mod accumulator { pub use crate :: accumulator :: { IngredientImpl , JarImpl } ; } pub mod input { pub use crate :: input :: input_field :: FieldIngredientImpl ; pub use crate :: input :: setter :: SetterImpl ; pub use crate :: input :: singleton :: { NotSingleton , Singleton } ; pub use crate :: input :: { Configuration , HasBuilder , IngredientImpl , JarImpl , Value } ; } pub mod interned { pub use crate :: interned :: { Configuration , HashEqLike , IngredientImpl , JarImpl , Lookup , Value , } ; } pub mod function { pub use crate :: function :: Configuration ; pub use crate :: function :: IngredientImpl ; pub use crate :: function :: Memo ; pub use crate :: table :: memo :: MemoEntryType ; } pub mod tracked_struct { pub use crate :: tracked_struct :: tracked_field :: FieldIngredientImpl ; pub use crate :: tracked_struct :: { Configuration , IngredientImpl , JarImpl , Value } ; } }
    };
}

plumbing!()