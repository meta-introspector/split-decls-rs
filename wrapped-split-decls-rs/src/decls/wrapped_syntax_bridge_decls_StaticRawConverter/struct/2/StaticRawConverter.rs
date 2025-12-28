use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A raw token (straight from lexer) converter that gives every token the same span."] struct StaticRawConverter < 'a , S > { lexed : parser :: LexedStr < 'a > , pos : usize , span : S , mode : DocCommentDesugarMode , }