macro_rules! deps {
    () => {
        Deserializer!();
        Value!();
        Error!();
        ValueDeserializer!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl < 'de , S : AsRef < str > > serde_core :: Deserializer < 'de > for Deserializer < S > { type Error = Error ; fn deserialize_any < V > (self , visitor : V) -> Result < < V as serde_core :: de :: Visitor < 'de > > :: Value , < Self as serde_core :: de :: Deserializer < 'de > > :: Error > where V : serde_core :: de :: Visitor < 'de > , { let raw = self . raw ; ValueDeserializer :: new (self . root) . deserialize_any (visitor) . map_err (| mut e : Self :: Error | { let raw = raw . as_ref () . map (| r | r . as_ref ()) ; e . set_input (raw) ; e }) } fn deserialize_option < V > (self , visitor : V) -> Result < < V as serde_core :: de :: Visitor < 'de > > :: Value , Error > where V : serde_core :: de :: Visitor < 'de > , { let raw = self . raw ; ValueDeserializer :: new (self . root) . deserialize_option (visitor) . map_err (| mut e : Self :: Error | { let raw = raw . as_ref () . map (| r | r . as_ref ()) ; e . set_input (raw) ; e }) } fn deserialize_newtype_struct < V > (self , name : & 'static str , visitor : V ,) -> Result < < V as serde_core :: de :: Visitor < 'de > > :: Value , Error > where V : serde_core :: de :: Visitor < 'de > , { let raw = self . raw ; ValueDeserializer :: new (self . root) . deserialize_newtype_struct (name , visitor) . map_err (| mut e : Self :: Error | { let raw = raw . as_ref () . map (| r | r . as_ref ()) ; e . set_input (raw) ; e }) } fn deserialize_struct < V > (self , name : & 'static str , fields : & 'static [& 'static str] , visitor : V ,) -> Result < < V as serde_core :: de :: Visitor < 'de > > :: Value , Error > where V : serde_core :: de :: Visitor < 'de > , { let raw = self . raw ; ValueDeserializer :: new (self . root) . deserialize_struct (name , fields , visitor) . map_err (| mut e : Self :: Error | { let raw = raw . as_ref () . map (| r | r . as_ref ()) ; e . set_input (raw) ; e }) } fn deserialize_enum < V > (self , name : & 'static str , variants : & 'static [& 'static str] , visitor : V ,) -> Result < < V as serde_core :: de :: Visitor < 'de > > :: Value , Error > where V : serde_core :: de :: Visitor < 'de > , { let raw = self . raw ; ValueDeserializer :: new (self . root) . deserialize_enum (name , variants , visitor) . map_err (| mut e : Self :: Error | { let raw = raw . as_ref () . map (| r | r . as_ref ()) ; e . set_input (raw) ; e }) } serde_core :: forward_to_deserialize_any ! { bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq bytes byte_buf map unit ignored_any unit_struct tuple_struct tuple identifier } }
    };
}

impl_332!()