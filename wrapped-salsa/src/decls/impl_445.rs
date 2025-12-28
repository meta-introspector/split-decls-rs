macro_rules! deps {
    () => {
        Table!();
        ZalsaLocal!();
        Id!();
        StorageNonce!();
        Nonce!();
        Views!();
        MemoIngredientIndex!();
        Database!();
        Zalsa!();
        SalsaStructInDb!();
        EventKind!();
        ErasedJar!();
        Ingredient!();
        Event!();
        IngredientIndex!();
        Runtime!();
    };
}

macro_rules! impl_445 {
    () => {
        deps!();
        impl Zalsa { pub (crate) fn new < Db : Database > (event_callback : Option < Box < dyn Fn (crate :: Event) + Send + Sync + 'static > > , jars : Vec < ErasedJar > ,) -> Self { let mut zalsa = Self { views_of : Views :: new :: < Db > () , jar_map : HashMap :: default () , ingredient_to_id_struct_type_id_map : Default :: default () , ingredients_vec : Vec :: new () , ingredients_requiring_reset : boxcar :: Vec :: new () , runtime : Runtime :: default () , memo_ingredient_indices : Default :: default () , event_callback , # [cfg (not (feature = "inventory"))] nonce : NONCE . nonce () , } ; # [cfg (feature = "inventory")] let mut jars = inventory :: iter :: < ErasedJar > () . copied () . chain (jars) . collect :: < Vec < _ > > () ; # [cfg (not (feature = "inventory"))] let mut jars = jars ; jars . sort_by (| a , b | a . kind . cmp (& b . kind) . then (a . type_name () . cmp (b . type_name ()))) ; for jar in jars { zalsa . insert_jar (jar) ; } zalsa } # [cfg (not (feature = "inventory"))] pub (crate) fn nonce (& self) -> crate :: nonce :: Nonce < StorageNonce > { self . nonce } pub (crate) fn runtime (& self) -> & Runtime { & self . runtime } pub (crate) fn runtime_mut (& mut self) -> & mut Runtime { & mut self . runtime } # [doc = " Returns the [`Table`] used to store the value of salsa structs"] # [inline] pub fn table (& self) -> & Table { self . runtime . table () } # [doc = " Returns a mutable reference to the [`Table`] used to store the value of salsa structs"] # [inline] # [allow (dead_code)] pub (crate) fn table_mut (& mut self) -> & mut Table { self . runtime . table_mut () } # [doc = " Returns the [`MemoTable`][] for the salsa struct with the given id"] pub (crate) fn memo_table_for < T : SalsaStructInDb > (& self , id : Id) -> MemoTableWithTypes < '_ > { unsafe { T :: memo_table (self , id , self . current_revision ()) } } # [doc = " Returns the ingredient at the given index, or panics if it is out-of-bounds."] # [inline] pub fn lookup_ingredient (& self , index : IngredientIndex) -> & dyn Ingredient { self . ingredients_vec [index . as_u32 () as usize] . as_ref () } # [doc = " Returns the ingredient at the given index."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The index must be in-bounds."] # [inline] pub unsafe fn lookup_ingredient_unchecked (& self , index : IngredientIndex) -> & dyn Ingredient { unsafe { self . ingredients_vec . get_unchecked (index . as_u32 () as usize) . as_ref () } } pub (crate) fn ingredient_index_for_memo (& self , struct_ingredient_index : IngredientIndex , memo_ingredient_index : MemoIngredientIndex ,) -> IngredientIndex { self . memo_ingredient_indices [struct_ingredient_index . as_u32 () as usize] [memo_ingredient_index . as_usize ()] } # [allow (unused)] pub (crate) fn ingredients (& self) -> impl Iterator < Item = & dyn Ingredient > { self . ingredients_vec . iter () . map (| ingredient | ingredient . as_ref ()) } # [doc = " Starts unwinding the stack if the current revision is cancelled."] # [doc = ""] # [doc = " This method can be called by query implementations that perform"] # [doc = " potentially expensive computations, in order to speed up propagation of"] # [doc = " cancellation."] # [doc = ""] # [doc = " Cancellation will automatically be triggered by salsa on any query"] # [doc = " invocation."] # [inline] pub (crate) fn unwind_if_revision_cancelled (& self , zalsa_local : & ZalsaLocal) { self . event (& | | crate :: Event :: new (crate :: EventKind :: WillCheckCancellation)) ; if self . runtime () . load_cancellation_flag () { zalsa_local . unwind_cancelled (self . current_revision ()) ; } } pub (crate) fn next_memo_ingredient_index (& mut self , struct_ingredient_index : IngredientIndex , ingredient_index : IngredientIndex ,) -> MemoIngredientIndex { let memo_ingredients = & mut self . memo_ingredient_indices ; let idx = struct_ingredient_index . as_u32 () as usize ; let memo_ingredients = if let Some (memo_ingredients) = memo_ingredients . get_mut (idx) { memo_ingredients } else { memo_ingredients . resize_with (idx + 1 , Vec :: new) ; memo_ingredients . get_mut (idx) . unwrap () } ; let mi = MemoIngredientIndex :: from_usize (memo_ingredients . len ()) ; memo_ingredients . push (ingredient_index) ; mi } }
    };
}

impl_445!()