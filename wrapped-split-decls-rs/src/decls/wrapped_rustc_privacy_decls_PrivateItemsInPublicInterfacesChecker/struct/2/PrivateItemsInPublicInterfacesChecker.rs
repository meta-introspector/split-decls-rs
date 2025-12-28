use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct PrivateItemsInPublicInterfacesChecker < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , effective_visibilities : & 'a EffectiveVisibilities , }