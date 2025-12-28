macro_rules! deps {
    () => {
        QueryEdge!();
        QueryOrigin!();
        DatabaseKeyIndex!();
        QueryOriginKind!();
    };
}

macro_rules! impl_478 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for QueryOrigin { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { # [repr (u8)] # [derive (serde :: Deserialize)] # [serde (rename = "QueryOrigin")] pub enum QueryOriginOwned { Assigned (DatabaseKeyIndex) = QueryOriginKind :: Assigned as u8 , Derived (Box < [QueryEdge] >) = QueryOriginKind :: Derived as u8 , DerivedUntracked (Box < [QueryEdge] >) = QueryOriginKind :: DerivedUntracked as u8 , FixpointInitial = QueryOriginKind :: FixpointInitial as u8 , } Ok (match QueryOriginOwned :: deserialize (deserializer) ? { QueryOriginOwned :: Assigned (key) => QueryOrigin :: assigned (key) , QueryOriginOwned :: Derived (edges) => QueryOrigin :: derived (edges) , QueryOriginOwned :: DerivedUntracked (edges) => QueryOrigin :: derived_untracked (edges) , QueryOriginOwned :: FixpointInitial => QueryOrigin :: fixpoint_initial () , }) } }
    };
}

impl_478!();