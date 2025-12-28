use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type LayoutCalculatorResult < FieldIdx , VariantIdx , F > = Result < LayoutData < FieldIdx , VariantIdx > , LayoutCalculatorError < F > > ;
}