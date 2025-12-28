use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait PipelineFunctor < Input : Send + 'static , Output : Send + 'static , Config > { fn map < 'writer > (& 'writer self , writer : & 'writer mut (impl tokio :: io :: AsyncWriteExt + Unpin + Send) , input : Input , _config : & 'writer Option < Config > ,) -> Pin < Box < dyn Future < Output = Result < Output > > + Send + 'writer > > ; }