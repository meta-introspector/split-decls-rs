// Generated macro for from_str (function)
macro_rules! Depcrate_defrom_str {
() => {
// Module: crate::de
// Provides: {"from_str"}
// Dependencies: {}
# [doc = " Deserializes a `application/x-www-form-urlencoded` value from a `&str`."] # [doc = ""] # [doc = " ```"] # [doc = " let meal = vec!["] # [doc = "     (\"bread\".to_owned(), \"baguette\".to_owned()),"] # [doc = "     (\"cheese\".to_owned(), \"comté\".to_owned()),"] # [doc = "     (\"meat\".to_owned(), \"ham\".to_owned()),"] # [doc = "     (\"fat\".to_owned(), \"butter\".to_owned()),"] # [doc = " ];"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     serde_urlencoded::from_str::<Vec<(String, String)>>("] # [doc = "         \"bread=baguette&cheese=comt%C3%A9&meat=ham&fat=butter\"),"] # [doc = "     Ok(meal));"] # [doc = " ```"] pub fn from_str < 'de , T > (input : & 'de str) -> Result < T , Error > where T : de :: Deserialize < 'de > , { from_bytes (input . as_bytes ()) }
};
}
