macro_rules! AliasRelationDirection {
    () => {
        # [derive (Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Copy)] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext , Encodable_NoContext , Decodable_NoContext))] pub enum AliasRelationDirection { Equate , Subtype , }
    };
}

AliasRelationDirection!()