#![cfg_attr(not(test), no_std)]
#![deny(unsafe_code)]

pub mod ed;
pub mod syscore;

#[cfg(feature = "shal")]
pub mod shal;

#[cfg(feature = "el")]
pub mod el;

// not pub
#[cfg(feature = "target")]
mod target;

#[cfg(test)]
mod test;
