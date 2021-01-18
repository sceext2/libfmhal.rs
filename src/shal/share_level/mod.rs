pub use super::shali::share_level::*;

pub trait ShareLevelET: Ord {
    fn to_index(&self) -> u8;
}

pub trait ShareLevelT: Copy {
    fn get_level(&self) -> ShareLevelE;
}
