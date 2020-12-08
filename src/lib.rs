#![no_std]
#![deny(unsafe_code)]

mod chip_agent;
mod system_core;
mod ocd;
mod driver;

#[cfg(feature = "target_stm32f1")]
mod target_stm32f1;

#[cfg(feature = "target_stm32f0")]
mod target_stm32f0;

#[cfg(test)]
mod test;
