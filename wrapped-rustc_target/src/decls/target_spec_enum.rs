macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! target_spec_enum {
    () => {
        deps!();
        macro_rules ! target_spec_enum { ($ (# [$ attr : meta]) * pub enum $ name : ident { $ ($ (# [$ variant_attr : meta]) * $ variant : ident = $ string : literal ,) * } parse_error_type = $ parse_error_type : literal ;) => { $ (# [$ attr]) * # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug , PartialOrd , Ord)] # [derive (schemars :: JsonSchema)] pub enum $ name { $ ($ (# [$ variant_attr]) * # [serde (rename = $ string)] $ variant ,) * } impl FromStr for $ name { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { $ ($ string => Self ::$ variant ,) * _ => { let all = [$ (concat ! ("'" , $ string , "'")) ,*] . join (", ") ; return Err (format ! ("invalid {}: '{s}'. allowed values: {all}" , $ parse_error_type)) ; } }) } } impl $ name { pub fn desc (& self) -> &'static str { match self { $ (Self ::$ variant => $ string ,) * } } } impl crate :: json :: ToJson for $ name { fn to_json (& self) -> crate :: json :: Json { self . desc () . to_json () } } crate :: json :: serde_deserialize_from_str ! ($ name) ; impl std :: fmt :: Display for $ name { fn fmt (& self , f : & mut std :: fmt :: Formatter <'_ >) -> std :: fmt :: Result { f . write_str (self . desc ()) } } } ; }
    };
}

target_spec_enum!();