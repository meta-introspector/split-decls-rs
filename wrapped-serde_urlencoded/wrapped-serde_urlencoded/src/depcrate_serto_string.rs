// Generated macro for to_string (function)
macro_rules! Depcrate_serto_string {
() => {
// Module: crate::ser
// Provides: {"to_string"}
// Dependencies: {}
# [doc = " Serializes a value into a `application/x-www-form-urlencoded` `String` buffer."] # [doc = ""] # [doc = " ```"] # [doc = " let meal = &["] # [doc = "     (\"bread\", \"baguette\"),"] # [doc = "     (\"cheese\", \"comté\"),"] # [doc = "     (\"meat\", \"ham\"),"] # [doc = "     (\"fat\", \"butter\"),"] # [doc = " ];"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     serde_urlencoded::to_string(meal),"] # [doc = "     Ok(\"bread=baguette&cheese=comt%C3%A9&meat=ham&fat=butter\".to_owned()));"] # [doc = " ```"] pub fn to_string < T : ser :: Serialize > (input : T) -> Result < String , Error > { let mut urlencoder = UrlEncodedSerializer :: new ("" . to_owned ()) ; input . serialize (Serializer :: new (& mut urlencoder)) ? ; Ok (urlencoder . finish ()) }
};
}
