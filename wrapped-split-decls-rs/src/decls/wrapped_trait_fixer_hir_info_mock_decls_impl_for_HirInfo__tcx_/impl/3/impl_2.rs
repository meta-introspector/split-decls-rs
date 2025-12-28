use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'tcx > HirInfo < 'tcx > for MockItem < 'tcx > { type OwnerId = OwnerId ; type ItemKind = ItemKind < 'tcx > ; type Span = Span ; fn get_owner_id (& self) -> Self :: OwnerId { OwnerId :: DUMMY } fn get_item_kind < 'a > (& 'a self) -> & 'a Self :: ItemKind { & ItemKind :: Struct (PhantomData) } fn get_item_span (& self) -> Self :: Span { DUMMY_SP } }
}