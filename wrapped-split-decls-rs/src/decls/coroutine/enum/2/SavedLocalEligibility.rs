use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Overlap eligibility and variant assignment for each CoroutineSavedLocal."] # [derive (Clone , Debug , PartialEq)] enum SavedLocalEligibility < VariantIdx , FieldIdx > { Unassigned , Assigned (VariantIdx) , Ineligible (Option < FieldIdx >) , }