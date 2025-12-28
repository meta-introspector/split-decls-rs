use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Replace any \"impl trait\" types with \"Box<dyn trait>\" or equivalent."] fn deimplify (rt : & mut ReturnType) { if let ReturnType :: Type (_ , ty) = rt { if let Type :: ImplTrait (ref tit) = & * * ty { let needs_pin = tit . bounds . iter () . any (| tpb | { if let TypeParamBound :: Trait (tb) = tpb { if let Some (seg) = tb . path . segments . last () { seg . ident == "Future" || seg . ident == "Stream" } else { false } } else { false } }) ; let bounds = & tit . bounds ; if needs_pin { * ty = parse2 (quote ! (:: std :: pin :: Pin < Box < dyn # bounds >>)) . unwrap () ; } else { * ty = parse2 (quote ! (Box < dyn # bounds >)) . unwrap () ; } } } }
}