macro_rules! deps {
    () => {
        SerializeLevel!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Serialize for SerializeLevel < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if self . 0 == & Level :: ERROR { serializer . serialize_str ("ERROR") } else if self . 0 == & Level :: WARN { serializer . serialize_str ("WARN") } else if self . 0 == & Level :: INFO { serializer . serialize_str ("INFO") } else if self . 0 == & Level :: DEBUG { serializer . serialize_str ("DEBUG") } else if self . 0 == & Level :: TRACE { serializer . serialize_str ("TRACE") } else { unreachable ! () } } }
    };
}

impl_14!();