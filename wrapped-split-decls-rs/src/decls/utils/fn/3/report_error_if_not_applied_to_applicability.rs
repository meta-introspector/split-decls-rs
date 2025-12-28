use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Reports an error if the field's type is not `Applicability`."] pub (crate) fn report_error_if_not_applied_to_applicability (attr : & Attribute , info : & FieldInfo < '_ > ,) -> Result < () , DiagnosticDeriveError > { report_error_if_not_applied_to_ty (attr , info , & ["rustc_errors" , "Applicability"] , "`Applicability`" ,) }
}