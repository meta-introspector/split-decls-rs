// Generated macro for PathSegmentsMut (struct)
macro_rules! Depcrate_path_segmentsPathSegmentsMut {
() => {
// Module: crate::path_segments
// Provides: {"PathSegmentsMut"}
// Dependencies: {}
# [doc = " Exposes methods to manipulate the path of an URL that is not cannot-be-base."] # [doc = ""] # [doc = " The path always starts with a `/` slash, and is made of slash-separated segments."] # [doc = " There is always at least one segment (which may be the empty string)."] # [doc = ""] # [doc = " Examples:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use url::Url;"] # [doc = ""] # [doc = " # #[cfg(feature = \"std\")]"] # [doc = " # use std::error::Error;"] # [doc = " # #[cfg(not(feature = \"std\"))]"] # [doc = " # use core::error::Error;"] # [doc = ""] # [doc = " # fn run() -> Result<(), Box<dyn Error>> {"] # [doc = " let mut url = Url::parse(\"mailto:me@example.com\")?;"] # [doc = " assert!(url.path_segments_mut().is_err());"] # [doc = ""] # [doc = " let mut url = Url::parse(\"http://example.net/foo/index.html\")?;"] # [doc = " url.path_segments_mut().map_err(|_| \"cannot be base\")?"] # [doc = "     .pop().push(\"img\").push(\"2/100%.png\");"] # [doc = " assert_eq!(url.as_str(), \"http://example.net/foo/img/2%2F100%25.png\");"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " # run().unwrap();"] # [doc = " ```"] # [derive (Debug)] pub struct PathSegmentsMut < 'a > { url : & 'a mut Url , after_first_slash : usize , after_path : String , old_after_path_position : u32 , }
};
}
