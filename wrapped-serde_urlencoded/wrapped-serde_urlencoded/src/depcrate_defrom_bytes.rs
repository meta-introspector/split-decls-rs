// Generated macro for from_bytes (function)
macro_rules! Depcrate_defrom_bytes {
() => {
// Module: crate::de
// Provides: {"from_bytes"}
// Dependencies: {}
# [doc = " Deserializes a `application/x-www-form-urlencoded` value from a `&[u8]`."] # [doc = ""] # [doc = " ```"] # [doc = " let meal = vec!["] # [doc = "     (\"bread\".to_owned(), \"baguette\".to_owned()),"] # [doc = "     (\"cheese\".to_owned(), \"comté\".to_owned()),"] # [doc = "     (\"meat\".to_owned(), \"ham\".to_owned()),"] # [doc = "     (\"fat\".to_owned(), \"butter\".to_owned()),"] # [doc = " ];"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     serde_urlencoded::from_bytes::<Vec<(String, String)>>("] # [doc = "         b\"bread=baguette&cheese=comt%C3%A9&meat=ham&fat=butter\"),"] # [doc = "     Ok(meal));"] # [doc = " ```"] pub fn from_bytes < 'de , T > (input : & 'de [u8]) -> Result < T , Error > where T : de :: Deserialize < 'de > , { T :: deserialize (Deserializer :: new (parse (input))) }
};
}
