macro_rules! deps {
    () => {
        QueryOriginKind!();
        QueryEdge!();
        QueryOriginRef!();
        DatabaseKeyIndex!();
        QueryOrigin!();
        IngredientIndex!();
    };
}

macro_rules! impl_476 {
    () => {
        deps!();
        impl QueryOrigin { # [doc = " Create a query origin of type `QueryOriginKind::FixpointInitial`."] pub fn fixpoint_initial () -> QueryOrigin { QueryOrigin { kind : QueryOriginKind :: FixpointInitial , metadata : 0 , data : QueryOriginData { empty : () } , } } pub fn is_derived_untracked (& self) -> bool { matches ! (self . kind , QueryOriginKind :: DerivedUntracked) } # [doc = " Create a query origin of type `QueryOriginKind::Derived`, with the given edges."] pub fn derived (input_outputs : Box < [QueryEdge] >) -> QueryOrigin { let length = u32 :: try_from (input_outputs . len ()) . expect ("exceeded more than `u32::MAX` query edges; this should never happen.") ; let input_outputs = unsafe { NonNull :: new_unchecked (Box :: into_raw (input_outputs) . cast :: < QueryEdge > ()) } ; QueryOrigin { kind : QueryOriginKind :: Derived , metadata : length , data : QueryOriginData { input_outputs } , } } # [doc = " Create a query origin of type `QueryOriginKind::DerivedUntracked`, with the given edges."] pub fn derived_untracked (input_outputs : Box < [QueryEdge] >) -> QueryOrigin { let mut origin = QueryOrigin :: derived (input_outputs) ; origin . kind = QueryOriginKind :: DerivedUntracked ; origin } # [doc = " Create a query origin of type `QueryOriginKind::Assigned`, with the given key."] pub fn assigned (key : DatabaseKeyIndex) -> QueryOrigin { QueryOrigin { kind : QueryOriginKind :: Assigned , metadata : key . ingredient_index () . as_u32 () , data : QueryOriginData { index : key . key_index () , } , } } # [doc = " Return a read-only reference to this query origin."] pub fn as_ref (& self) -> QueryOriginRef < '_ > { match self . kind { QueryOriginKind :: Assigned => { let index = unsafe { self . data . index } ; let ingredient_index = unsafe { IngredientIndex :: new_unchecked (self . metadata) } ; QueryOriginRef :: Assigned (DatabaseKeyIndex :: new (ingredient_index , index)) } QueryOriginKind :: Derived => { let input_outputs = unsafe { self . data . input_outputs } ; let length = self . metadata as usize ; let input_outputs = unsafe { std :: slice :: from_raw_parts (input_outputs . as_ptr () , length) } ; QueryOriginRef :: Derived (input_outputs) } QueryOriginKind :: DerivedUntracked => { let input_outputs = unsafe { self . data . input_outputs } ; let length = self . metadata as usize ; let input_outputs = unsafe { std :: slice :: from_raw_parts (input_outputs . as_ptr () , length) } ; QueryOriginRef :: DerivedUntracked (input_outputs) } QueryOriginKind :: FixpointInitial => QueryOriginRef :: FixpointInitial , } } }
    };
}

impl_476!();