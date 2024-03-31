pub mod iterators;
pub mod syntax_node;
pub mod wit;

pub use crate::wit::YggdrasilHost;
pub use rctree::Node;
pub use yggdrasil_rt::{state, OutputResult, Regex, State, YggdrasilParser, YggdrasilRule};

wit_bindgen::generate!({
    world: "host",
    allow_unused: true,
});

export!(YggdrasilHost);
