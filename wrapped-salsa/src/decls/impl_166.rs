macro_rules! deps {
    () => {
        Zalsa!();
        Revision!();
        Database!();
        IngredientIndex!();
        RawDatabase!();
        Id!();
        FxIndexSet!();
        IngredientImpl!();
        Location!();
        FxHashSet!();
        QueryEdge!();
        JarKind!();
        Ingredient!();
        Configuration!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < C : Configuration > Ingredient for IngredientImpl < C > { fn location (& self) -> & 'static crate :: ingredient :: Location { & C :: LOCATION } fn ingredient_index (& self) -> IngredientIndex { self . ingredient_index } unsafe fn maybe_changed_after (& self , _zalsa : & crate :: zalsa :: Zalsa , _db : crate :: database :: RawDatabase < '_ > , _input : Id , _revision : Revision , _cycle_heads : & mut VerifyCycleHeads ,) -> VerifyResult { panic ! ("nothing should ever depend on an input struct directly") } fn collect_minimum_serialized_edges (& self , _zalsa : & Zalsa , _edge : QueryEdge , _serialized_edges : & mut FxIndexSet < QueryEdge > , _visited_edges : & mut FxHashSet < QueryEdge > ,) { panic ! ("nothing should ever depend on an input struct directly") } fn debug_name (& self) -> & 'static str { C :: DEBUG_NAME } fn jar_kind (& self) -> JarKind { JarKind :: Struct } fn memo_table_types (& self) -> & Arc < MemoTableTypes > { & self . memo_table_types } fn memo_table_types_mut (& mut self) -> & mut Arc < MemoTableTypes > { & mut self . memo_table_types } # [doc = " Returns memory usage information about any inputs."] # [cfg (feature = "salsa_unstable")] fn memory_usage (& self , db : & dyn crate :: Database) -> Option < Vec < crate :: database :: SlotInfo > > { let memory_usage = self . entries (db . zalsa ()) . map (| entry | unsafe { entry . value . memory_usage (& self . memo_table_types) }) . collect () ; Some (memory_usage) } fn is_persistable (& self) -> bool { C :: PERSIST } fn should_serialize (& self , zalsa : & Zalsa) -> bool { C :: PERSIST && self . entries (zalsa) . next () . is_some () } # [cfg (feature = "persistence")] unsafe fn serialize < 'db > (& 'db self , zalsa : & 'db Zalsa , f : & mut dyn FnMut (& dyn erased_serde :: Serialize) ,) { f (& persistence :: SerializeIngredient { zalsa , _ingredient : self , }) } # [cfg (feature = "persistence")] fn deserialize (& mut self , zalsa : & mut Zalsa , deserializer : & mut dyn erased_serde :: Deserializer ,) -> Result < () , erased_serde :: Error > { let deserialize = persistence :: DeserializeIngredient { zalsa , ingredient : self , } ; serde :: de :: DeserializeSeed :: deserialize (deserialize , deserializer) } }
    };
}

impl_166!();