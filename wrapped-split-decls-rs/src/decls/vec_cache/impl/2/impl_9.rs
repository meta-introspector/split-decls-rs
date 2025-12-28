use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < K : Idx , V , I > Default for VecCache < K , V , I > { fn default () -> Self { VecCache { buckets : Default :: default () , key : PhantomData , len : Default :: default () , present : Default :: default () , } } }