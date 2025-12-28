use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct TtTreeSink < 'a , Ctx > where SpanData < Ctx > : Copy , { buf : String , cursor : Cursor < 'a , SpanData < Ctx > > , text_pos : TextSize , inner : SyntaxTreeBuilder , token_map : SpanMap < Ctx > , }