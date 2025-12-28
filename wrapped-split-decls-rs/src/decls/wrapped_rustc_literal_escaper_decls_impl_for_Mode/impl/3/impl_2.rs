use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Mode { pub fn in_double_quotes (self) -> bool { match self { Mode :: Str | Mode :: RawStr | Mode :: ByteStr | Mode :: RawByteStr | Mode :: CStr | Mode :: RawCStr => true , Mode :: Char | Mode :: Byte => false , } } pub fn prefix_noraw (self) -> & 'static str { match self { Mode :: Char | Mode :: Str | Mode :: RawStr => "" , Mode :: Byte | Mode :: ByteStr | Mode :: RawByteStr => "b" , Mode :: CStr | Mode :: RawCStr => "c" , } } }
}