// public API for shal

mod share_box;
mod share_box_atomic;
pub mod share_level;
mod share_root;

// internal implementation based on 'target'
#[cfg(feature = "target_stm32f1")]
use crate::target::stm32f1::shal as shali;
