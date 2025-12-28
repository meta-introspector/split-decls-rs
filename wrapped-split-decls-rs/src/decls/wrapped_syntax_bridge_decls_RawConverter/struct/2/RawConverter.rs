use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A raw token (straight from lexer) converter"] struct RawConverter < 'a , Ctx > { lexed : parser :: LexedStr < 'a > , pos : usize , anchor : SpanAnchor , ctx : Ctx , mode : DocCommentDesugarMode , }