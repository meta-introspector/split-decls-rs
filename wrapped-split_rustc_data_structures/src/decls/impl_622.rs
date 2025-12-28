macro_rules! deps {
    () => {
        Edge!();
        TransitiveRelationBuilder!();
        Index!();
        Frozen!();
        TransitiveRelation!();
    };
}

macro_rules! impl_622 {
    () => {
        deps!();
        impl < T : Eq + Hash + Copy > TransitiveRelationBuilder < T > { pub fn is_empty (& self) -> bool { self . edges . is_empty () } pub fn elements (& self) -> impl Iterator < Item = & T > { self . elements . iter () } fn index (& self , a : T) -> Option < Index > { self . elements . get_index_of (& a) . map (Index) } fn add_index (& mut self , a : T) -> Index { let (index , _added) = self . elements . insert_full (a) ; Index (index) } # [doc = " Applies the (partial) function to each edge and returns a new"] # [doc = " relation builder. If `f` returns `None` for any end-point,"] # [doc = " returns `None`."] pub fn maybe_map < F , U > (& self , mut f : F) -> Option < TransitiveRelationBuilder < U > > where F : FnMut (T) -> Option < U > , U : Clone + Debug + Eq + Hash + Copy , { let mut result = TransitiveRelationBuilder :: default () ; for edge in & self . edges { result . add (f (self . elements [edge . source . 0]) ? , f (self . elements [edge . target . 0]) ?) ; } Some (result) } # [doc = " Indicate that `a < b` (where `<` is this relation)"] pub fn add (& mut self , a : T , b : T) { let a = self . add_index (a) ; let b = self . add_index (b) ; let edge = Edge { source : a , target : b } ; self . edges . insert (edge) ; } # [doc = " Compute the transitive closure derived from the edges, and converted to"] # [doc = " the final result. After this, all elements will be immutable to maintain"] # [doc = " the correctness of the result."] pub fn freeze (self) -> TransitiveRelation < T > { let mut matrix = BitMatrix :: new (self . elements . len () , self . elements . len ()) ; let mut changed = true ; while changed { changed = false ; for edge in & self . edges { changed |= matrix . insert (edge . source . 0 , edge . target . 0) ; changed |= matrix . union_rows (edge . target . 0 , edge . source . 0) ; } } TransitiveRelation { builder : Frozen :: freeze (self) , closure : Frozen :: freeze (matrix) } } }
    };
}

impl_622!()