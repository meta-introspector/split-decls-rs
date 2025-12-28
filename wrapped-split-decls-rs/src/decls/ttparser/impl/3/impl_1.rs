use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > TtParser < 'a > { pub fn new (macro_name : Ident) -> Self { unimplemented ! () } pub fn parse_tt < 'matcher , T : super :: Tracker < 'matcher > > (& mut self , _arg : & mut Cow < 'a , Parser < 'a > > , _matcher : & 'matcher [MatcherLoc] , _track : & mut T ,) -> ParseResult < T :: Failure > { unimplemented ! () } }
}