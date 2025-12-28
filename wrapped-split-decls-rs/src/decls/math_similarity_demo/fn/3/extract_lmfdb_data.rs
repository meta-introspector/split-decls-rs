use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn extract_lmfdb_data () -> Vec < EllipticCurveData > { vec ! [EllipticCurveData { field : "3.3.621.1" . to_string () , conductor_norm : 621 , curves_count : 1454 , isogeny_classes : 534 , complexity_ratio : 1454.0 / 534.0 , } , EllipticCurveData { field : "3.3.625.1" . to_string () , conductor_norm : 625 , curves_count : 1118 , isogeny_classes : 538 , complexity_ratio : 1118.0 / 538.0 , } ,] }