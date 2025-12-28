macro_rules! deps {
    () => {
        BufferData!();
        Hasher!();
        Buffer!();
        Accumulators!();
    };
}

macro_rules! serialize_impl {
    () => {
        deps!();
        # [cfg (feature = "serialize")] # [cfg_attr (docsrs , doc (cfg (feature = "serialize")))] mod serialize_impl { use serde :: { Deserialize , Serialize } ; use super :: * ; impl < 'de > Deserialize < 'de > for Hasher { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let shim = Deserialize :: deserialize (deserializer) ? ; let Shim { total_len , seed , core , buffer , buffer_usage , } = shim ; let Core { v1 , v2 , v3 , v4 } = core ; let mut buffer_data = BufferData :: new () ; buffer_data . bytes_mut () . copy_from_slice (& buffer) ; Ok (Hasher { seed , accumulators : Accumulators ([v1 , v2 , v3 , v4]) , buffer : Buffer { offset : buffer_usage , data : buffer_data , } , length : total_len , }) } } impl Serialize for Hasher { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { let Hasher { seed , ref accumulators , ref buffer , length , } = * self ; let [v1 , v2 , v3 , v4] = accumulators . 0 ; let Buffer { offset , ref data } = * buffer ; let buffer = * data . bytes () ; let shim = Shim { total_len : length , seed , core : Core { v1 , v2 , v3 , v4 } , buffer , buffer_usage : offset , } ; shim . serialize (serializer) } } # [derive (Serialize , Deserialize)] struct Shim { total_len : u64 , seed : u64 , core : Core , buffer : [u8 ; 32] , buffer_usage : usize , } # [derive (Serialize , Deserialize)] struct Core { v1 : u64 , v2 : u64 , v3 : u64 , v4 : u64 , } # [cfg (test)] mod test { use std :: hash :: Hasher as _ ; use super :: * ; type Result < T = () , E = serde_json :: Error > = core :: result :: Result < T , E > ; # [test] fn test_serialization_cycle () -> Result { let mut hasher = Hasher :: with_seed (0) ; hasher . write (b"Hello, world!\0") ; let _ = hasher . finish () ; let serialized = serde_json :: to_string (& hasher) ? ; let unserialized : Hasher = serde_json :: from_str (& serialized) ? ; assert_eq ! (hasher , unserialized) ; Ok (()) } # [test] fn test_serialization_stability () -> Result { let mut hasher = Hasher :: with_seed (0) ; hasher . write (b"Hello, world!\0") ; let _ = hasher . finish () ; let expected_serialized = r#"{
                "total_len": 14,
                "seed": 0,
                "core": {
                  "v1": 6983438078262162902,
                  "v2": 14029467366897019727,
                  "v3": 0,
                  "v4": 7046029288634856825
                },
                "buffer": [
                  72,  101, 108, 108, 111, 44, 32, 119,
                  111, 114, 108, 100, 33,  0,  0,  0,
                  0,   0,   0,   0,   0,   0,  0,  0,
                  0,   0,   0,   0,   0,   0,  0,  0
                ],
                "buffer_usage": 14
            }"# ; let unserialized : Hasher = serde_json :: from_str (expected_serialized) ? ; assert_eq ! (hasher , unserialized) ; let expected_value : serde_json :: Value = serde_json :: from_str (expected_serialized) ? ; let actual_value = serde_json :: to_value (& hasher) ? ; assert_eq ! (expected_value , actual_value) ; Ok (()) } } }
    };
}

serialize_impl!();