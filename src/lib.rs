#![no_std]
#![deny(unsafe_code)]

mod syscore;
mod ed;

#[cfg(feature = "shal")]
mod shal;

#[cfg(feature = "el")]
mod el;

#[cfg(feature = "target")]
mod target;

// TODO

#[cfg(test)]
mod test;
