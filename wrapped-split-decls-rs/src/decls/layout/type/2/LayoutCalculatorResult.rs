use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type LayoutCalculatorResult < FieldIdx , VariantIdx , F > = Result < LayoutData < FieldIdx , VariantIdx > , LayoutCalculatorError < F > > ;