macro_rules! deps {
    () => {
        UnitOnly!();
        Value!();
        Error!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl < 'de , E > serde_core :: de :: VariantAccess < 'de > for UnitOnly < E > where E : serde_core :: de :: Error , { type Error = E ; fn unit_variant (self) -> Result < () , < Self as serde_core :: de :: VariantAccess < 'de > > :: Error > { Ok (()) } fn newtype_variant_seed < T > (self , _seed : T) -> Result < < T as serde_core :: de :: DeserializeSeed < 'de > > :: Value , < Self as serde_core :: de :: VariantAccess < 'de > > :: Error > where T : serde_core :: de :: DeserializeSeed < 'de > , { Err (serde_core :: de :: Error :: invalid_type (serde_core :: de :: Unexpected :: UnitVariant , & "newtype variant" ,)) } fn tuple_variant < V > (self , _len : usize , _visitor : V) -> Result < < V as serde_core :: de :: Visitor < 'de > > :: Value , < Self as serde_core :: de :: VariantAccess < 'de > > :: Error > where V : serde_core :: de :: Visitor < 'de > , { Err (serde_core :: de :: Error :: invalid_type (serde_core :: de :: Unexpected :: UnitVariant , & "tuple variant" ,)) } fn struct_variant < V > (self , _fields : & 'static [& 'static str] , _visitor : V ,) -> Result < < V as serde_core :: de :: Visitor < 'de > > :: Value , < Self as serde_core :: de :: VariantAccess < 'de > > :: Error > where V : serde_core :: de :: Visitor < 'de > , { Err (serde_core :: de :: Error :: invalid_type (serde_core :: de :: Unexpected :: UnitVariant , & "struct variant" ,)) } }
    };
}

impl_301!();