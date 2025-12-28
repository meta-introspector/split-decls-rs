use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl CheckoutNotificationType { is_bit_set ! (is_conflict , CheckoutNotificationType :: CONFLICT) ; is_bit_set ! (is_dirty , CheckoutNotificationType :: DIRTY) ; is_bit_set ! (is_updated , CheckoutNotificationType :: UPDATED) ; is_bit_set ! (is_untracked , CheckoutNotificationType :: UNTRACKED) ; is_bit_set ! (is_ignored , CheckoutNotificationType :: IGNORED) ; }
}