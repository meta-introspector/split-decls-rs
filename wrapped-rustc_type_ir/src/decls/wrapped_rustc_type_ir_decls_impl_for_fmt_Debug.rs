use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Debug for Variance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            match *self {
                Variance::Covariant => "+",
                Variance::Contravariant => "-",
                Variance::Invariant => "o",
                Variance::Bivariant => "*",
            },
        )
    }
}
