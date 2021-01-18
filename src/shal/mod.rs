// public API for shal

mod share_box;
pub mod share_box_atomic;
pub mod share_level;
mod share_root;

pub use share_box::ShareBox;
pub use share_root::ShareRoot;

// internal implementation based on 'target'
#[cfg(feature = "target_stm32f1")]
use crate::target::stm32f1::shal as shali;
