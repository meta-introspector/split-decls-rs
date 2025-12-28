use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn custom_subdiagnostic_derive_impl (s : Structure) -> TokenStream { let name = & s . ast () . ident ; let expanded = quote ! { impl rustc_errors :: Subdiagnostic for # name { fn add_to_diag < G : rustc_errors :: EmissionGuarantee > (self , diag : & mut rustc_errors :: Diag <'_ , G >) { let msg = format ! ("Dummy subdiagnostic for {}" , stringify ! (# name)) ; diag . sub (rustc_errors :: Level :: Note , msg , self . span . into ()) ; } } } ; expanded . into () }