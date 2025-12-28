macro_rules! deps {
    () => {
        Zalsa!();
        IngredientIndex!();
        Id!();
        Value!();
        IngredientImpl!();
        Durability!();
        Runtime!();
        ZalsaLocal!();
        Configuration!();
        FromIdWithDb!();
        StructEntry!();
        Table!();
        DatabaseKeyIndex!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < C : Configuration > IngredientImpl < C > { pub fn new (index : IngredientIndex) -> Self { Self { ingredient_index : index , singleton : Default :: default () , memo_table_types : Arc :: new (MemoTableTypes :: default ()) , _phantom : std :: marker :: PhantomData , } } fn data (zalsa : & Zalsa , id : Id) -> & Value < C > { zalsa . table () . get (id) } fn data_raw (table : & Table , id : Id) -> * mut Value < C > { table . get_raw (id) } pub fn database_key_index (& self , id : Id) -> DatabaseKeyIndex { DatabaseKeyIndex :: new (self . ingredient_index , id) } pub fn new_input (& self , zalsa : & Zalsa , zalsa_local : & ZalsaLocal , fields : C :: Fields , revisions : C :: Revisions , durabilities : C :: Durabilities ,) -> C :: Struct { let id = self . singleton . with_scope (| | { zalsa_local . allocate (zalsa , self . ingredient_index , | _ | Value :: < C > { fields , revisions , durabilities , memos : unsafe { MemoTable :: new (self . memo_table_types ()) } , }) . 0 }) ; FromIdWithDb :: from_id (id , zalsa) } # [doc = " Change the value of the field `field_index` to a new value."] # [doc = ""] # [doc = " # Parameters"] # [doc = ""] # [doc = " * `runtime`, the salsa runtiem"] # [doc = " * `id`, id of the input struct"] # [doc = " * `field_index`, index of the field that will be changed"] # [doc = " * `durability`, durability of the new value. If omitted, uses the durability of the previous value."] # [doc = " * `setter`, function that modifies the fields tuple; should only modify the element for `field_index`"] pub fn set_field < R > (& mut self , runtime : & mut Runtime , id : C :: Struct , field_index : usize , durability : Option < Durability > , setter : impl FnOnce (& mut C :: Fields) -> R ,) -> R { let id : Id = id . as_id () ; let data_raw = Self :: data_raw (runtime . table () , id) ; let data = unsafe { & mut * data_raw } ; data . revisions [field_index] = runtime . current_revision () ; let field_durability = & mut data . durabilities [field_index] ; if * field_durability != Durability :: MIN { runtime . report_tracked_write (* field_durability) ; } * field_durability = durability . unwrap_or (* field_durability) ; setter (& mut data . fields) } # [doc = " Get the singleton input previously created (if any)."] # [doc (hidden)] pub fn get_singleton_input (& self , zalsa : & Zalsa) -> Option < C :: Struct > where C : Configuration < Singleton = Singleton > , { self . singleton . index () . map (| id | FromIdWithDb :: from_id (id , zalsa)) } # [doc = " Access field of an input."] # [doc = " Note that this function returns the entire tuple of value fields."] # [doc = " The caller is responsible for selecting the appropriate element."] pub fn field < 'db > (& 'db self , zalsa : & 'db Zalsa , zalsa_local : & 'db ZalsaLocal , id : C :: Struct , field_index : usize ,) -> & 'db C :: Fields { let field_ingredient_index = self . ingredient_index . successor (field_index) ; let id = id . as_id () ; let value = Self :: data (zalsa , id) ; let durability = value . durabilities [field_index] ; let revision = value . revisions [field_index] ; zalsa_local . report_tracked_read_simple (DatabaseKeyIndex :: new (field_ingredient_index , id) , durability , revision ,) ; & value . fields } # [doc = " Returns all data corresponding to the input struct."] pub fn entries < 'db > (& 'db self , zalsa : & 'db Zalsa) -> impl Iterator < Item = StructEntry < 'db , C > > { zalsa . table () . slots_of :: < Value < C > > () . map (| (id , value) | StructEntry { value , key : self . database_key_index (id) , }) } # [doc = " Peek at the field values without recording any read dependency."] # [doc = " Used for debug printouts."] pub fn leak_fields < 'db > (& 'db self , zalsa : & 'db Zalsa , id : C :: Struct) -> & 'db C :: Fields { let id = id . as_id () ; let value = Self :: data (zalsa , id) ; & value . fields } }
    };
}

impl_163!()