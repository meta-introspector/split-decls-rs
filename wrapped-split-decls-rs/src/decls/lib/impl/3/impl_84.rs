use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < FieldIdx : Idx , VariantIdx : Idx > LayoutData < FieldIdx , VariantIdx > { # [doc = " Returns `true` if this is an aggregate type (including a ScalarPair!)"] pub fn is_aggregate (& self) -> bool { match self . backend_repr { BackendRepr :: Scalar (_) | BackendRepr :: SimdVector { .. } => false , BackendRepr :: ScalarPair (..) | BackendRepr :: Memory { .. } => true , } } # [doc = " Returns `true` if this is an uninhabited type"] pub fn is_uninhabited (& self) -> bool { self . uninhabited } }