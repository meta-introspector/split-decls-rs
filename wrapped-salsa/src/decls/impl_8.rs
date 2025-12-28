macro_rules! deps {
    () => {
        Location!();
        FxIndexSet!();
        IngredientIndex!();
        IngredientImpl!();
        Revision!();
        Zalsa!();
        FxHashSet!();
        JarKind!();
        RawDatabase!();
        Id!();
        QueryEdge!();
        Accumulator!();
        Ingredient!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < A : Accumulator > Ingredient for IngredientImpl < A > { fn location (& self) -> & 'static crate :: ingredient :: Location { & const { crate :: ingredient :: Location { file : file ! () , line : line ! () , } } } fn ingredient_index (& self) -> IngredientIndex { self . index } unsafe fn maybe_changed_after (& self , _zalsa : & crate :: zalsa :: Zalsa , _db : crate :: database :: RawDatabase < '_ > , _input : Id , _revision : Revision , _cycle_heads : & mut VerifyCycleHeads ,) -> VerifyResult { panic ! ("nothing should ever depend on an accumulator directly") } fn collect_minimum_serialized_edges (& self , _zalsa : & Zalsa , _edge : QueryEdge , _serialized_edges : & mut FxIndexSet < QueryEdge > , _visited_edges : & mut FxHashSet < QueryEdge > ,) { panic ! ("nothing should ever depend on an accumulator directly") } fn debug_name (& self) -> & 'static str { A :: DEBUG_NAME } fn jar_kind (& self) -> JarKind { JarKind :: Struct } fn memo_table_types (& self) -> & Arc < MemoTableTypes > { unreachable ! ("accumulator does not allocate pages") } fn memo_table_types_mut (& mut self) -> & mut Arc < MemoTableTypes > { unreachable ! ("accumulator does not allocate pages") } }
    };
}

impl_8!()