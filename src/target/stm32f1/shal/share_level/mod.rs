// stm32f1* with Cortex-M3 CPU, up to 16 interrupt levels

use core::cmp::Ordering;

use crate::shal::share_level::{ShareLevelET, ShareLevelT};

// Level0 is lowest, with less priority

#[derive(Clone, Copy, Debug, Ord, Eq, PartialEq)]
#[allow(unused)]
pub enum ShareLevelE {
    Level0 = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
    Level4 = 4,
    Level5 = 5,
    Level6 = 6,
    Level7 = 7,
    Level8 = 8,
    Level9 = 9,
    Level10 = 10,
    Level11 = 11,
    Level12 = 12,
    Level13 = 13,
    Level14 = 14,
    Level15 = 15,
}

impl ShareLevelET for ShareLevelE {
    fn to_index(&self) -> u8 {
        return *self as u8;
    }
}

impl PartialOrd for ShareLevelE {
    fn partial_cmp(&self, r: &Self) -> Option<Ordering> {
        let a = self.to_index();
        let b = r.to_index();
        if a < b {
            Some(Ordering::Less)
        } else if a == b {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

pub trait Level0T: ShareLevelT {}
pub trait Level1T: Level0T {}
pub trait Level2T: Level1T {}
pub trait Level3T: Level2T {}
pub trait Level4T: Level3T {}
pub trait Level5T: Level4T {}
pub trait Level6T: Level5T {}
pub trait Level7T: Level6T {}
pub trait Level8T: Level7T {}
pub trait Level9T: Level8T {}
pub trait Level10T: Level9T {}
pub trait Level11T: Level10T {}
pub trait Level12T: Level11T {}
pub trait Level13T: Level12T {}
pub trait Level14T: Level13T {}
pub trait Level15T: Level14T {}

// level0
pub struct Level0 {
    level: ShareLevelE,
}

impl Level0T for Level0 {}

impl ShareLevelT for Level0 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level0 {
    pub fn new() -> Level0 {
        Level0 {
            level: ShareLevelE::Level0,
        }
    }
}

// level1
pub struct Level1 {
    level: ShareLevelE,
}

impl Level1T for Level1 {}
impl Level0T for Level1 {}

impl ShareLevelT for Level1 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level1 {
    pub fn new() -> Level1 {
        Level1 {
            level: ShareLevelE::Level1,
        }
    }
}

// level2
pub struct Level2 {
    level: ShareLevelE,
}

impl Level2T for Level2 {}
impl Level1T for Level2 {}
impl Level0T for Level2 {}

impl ShareLevelT for Level2 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level2 {
    pub fn new() -> Level2 {
        Level2 {
            level: ShareLevelE::Level2,
        }
    }
}

// level3
pub struct Level3 {
    level: ShareLevelE,
}

impl Level3T for Level3 {}
impl Level2T for Level3 {}
impl Level1T for Level3 {}
impl Level0T for Level3 {}

impl ShareLevelT for Level3 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level3 {
    pub fn new() -> Level3 {
        Level3 {
            level: ShareLevelE::Level3,
        }
    }
}

// level4
pub struct Level4 {
    level: ShareLevelE,
}

impl Level4T for Level4 {}
impl Level3T for Level4 {}
impl Level2T for Level4 {}
impl Level1T for Level4 {}
impl Level0T for Level4 {}

impl ShareLevelT for Level4 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level4 {
    pub fn new() -> Level4 {
        Level4 {
            level: ShareLevelE::Level4,
        }
    }
}

// level5
pub struct Level5 {
    level: ShareLevelE,
}

impl Level5T for Level5 {}
impl Level4T for Level5 {}
impl Level3T for Level5 {}
impl Level2T for Level5 {}
impl Level1T for Level5 {}
impl Level0T for Level5 {}

impl ShareLevelT for Level5 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level5 {
    pub fn new() -> Level5 {
        Level5 {
            level: ShareLevelE::Level5,
        }
    }
}

// level6
pub struct Level6 {
    level: ShareLevelE,
}

impl Level6T for Level6 {}
impl Level5T for Level6 {}
impl Level4T for Level6 {}
impl Level3T for Level6 {}
impl Level2T for Level6 {}
impl Level1T for Level6 {}
impl Level0T for Level6 {}

impl ShareLevelT for Level6 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level6 {
    pub fn new() -> Level6 {
        Level6 {
            level: ShareLevelE::Level6,
        }
    }
}

// level7
pub struct Level7 {
    level: ShareLevelE,
}

impl Level7T for Level7 {}
impl Level6T for Level7 {}
impl Level5T for Level7 {}
impl Level4T for Level7 {}
impl Level3T for Level7 {}
impl Level2T for Level7 {}
impl Level1T for Level7 {}
impl Level0T for Level7 {}

impl ShareLevelT for Level7 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level7 {
    pub fn new() -> Level7 {
        Level7 {
            level: ShareLevelE::Level7,
        }
    }
}

// level8
pub struct Level8 {
    level: ShareLevelE,
}

impl Level8T for Level8 {}
impl Level7T for Level8 {}
impl Level6T for Level8 {}
impl Level5T for Level8 {}
impl Level4T for Level8 {}
impl Level3T for Level8 {}
impl Level2T for Level8 {}
impl Level1T for Level8 {}
impl Level0T for Level8 {}

impl ShareLevelT for Level8 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level8 {
    pub fn new() -> Level8 {
        Level8 {
            level: ShareLevelE::Level8,
        }
    }
}

// level9
pub struct Level9 {
    level: ShareLevelE,
}

impl Level9T for Level9 {}
impl Level8T for Level9 {}
impl Level7T for Level9 {}
impl Level6T for Level9 {}
impl Level5T for Level9 {}
impl Level4T for Level9 {}
impl Level3T for Level9 {}
impl Level2T for Level9 {}
impl Level1T for Level9 {}
impl Level0T for Level9 {}

impl ShareLevelT for Level9 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level9 {
    pub fn new() -> Level9 {
        Level9 {
            level: ShareLevelE::Level9,
        }
    }
}

// level10
pub struct Level10 {
    level: ShareLevelE,
}

impl Level10T for Level10 {}
impl Level9T for Level10 {}
impl Level8T for Level10 {}
impl Level7T for Level10 {}
impl Level6T for Level10 {}
impl Level5T for Level10 {}
impl Level4T for Level10 {}
impl Level3T for Level10 {}
impl Level2T for Level10 {}
impl Level1T for Level10 {}
impl Level0T for Level10 {}

impl ShareLevelT for Level10 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level10 {
    pub fn new() -> Level10 {
        Level10 {
            level: ShareLevelE::Level10,
        }
    }
}

// level11
pub struct Level11 {
    level: ShareLevelE,
}

impl Level11T for Level11 {}
impl Level10T for Level11 {}
impl Level9T for Level11 {}
impl Level8T for Level11 {}
impl Level7T for Level11 {}
impl Level6T for Level11 {}
impl Level5T for Level11 {}
impl Level4T for Level11 {}
impl Level3T for Level11 {}
impl Level2T for Level11 {}
impl Level1T for Level11 {}
impl Level0T for Level11 {}

impl ShareLevelT for Level11 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level11 {
    pub fn new() -> Level11 {
        Level11 {
            level: ShareLevelE::Level11,
        }
    }
}

// level12
pub struct Level12 {
    level: ShareLevelE,
}

impl Level12T for Level12 {}
impl Level11T for Level12 {}
impl Level10T for Level12 {}
impl Level9T for Level12 {}
impl Level8T for Level12 {}
impl Level7T for Level12 {}
impl Level6T for Level12 {}
impl Level5T for Level12 {}
impl Level4T for Level12 {}
impl Level3T for Level12 {}
impl Level2T for Level12 {}
impl Level1T for Level12 {}
impl Level0T for Level12 {}

impl ShareLevelT for Level12 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level12 {
    pub fn new() -> Level12 {
        Level12 {
            level: ShareLevelE::Level12,
        }
    }
}

// level13
pub struct Level13 {
    level: ShareLevelE,
}

impl Level13T for Level13 {}
impl Level12T for Level13 {}
impl Level11T for Level13 {}
impl Level10T for Level13 {}
impl Level9T for Level13 {}
impl Level8T for Level13 {}
impl Level7T for Level13 {}
impl Level6T for Level13 {}
impl Level5T for Level13 {}
impl Level4T for Level13 {}
impl Level3T for Level13 {}
impl Level2T for Level13 {}
impl Level1T for Level13 {}
impl Level0T for Level13 {}

impl ShareLevelT for Level13 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level13 {
    pub fn new() -> Level13 {
        Level13 {
            level: ShareLevelE::Level13,
        }
    }
}

// level14
pub struct Level14 {
    level: ShareLevelE,
}

impl Level14T for Level14 {}
impl Level13T for Level14 {}
impl Level12T for Level14 {}
impl Level11T for Level14 {}
impl Level10T for Level14 {}
impl Level9T for Level14 {}
impl Level8T for Level14 {}
impl Level7T for Level14 {}
impl Level6T for Level14 {}
impl Level5T for Level14 {}
impl Level4T for Level14 {}
impl Level3T for Level14 {}
impl Level2T for Level14 {}
impl Level1T for Level14 {}
impl Level0T for Level14 {}

impl ShareLevelT for Level14 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level14 {
    pub fn new() -> Level14 {
        Level14 {
            level: ShareLevelE::Level14,
        }
    }
}

// level15
pub struct Level15 {
    level: ShareLevelE,
}

impl Level15T for Level15 {}
impl Level14T for Level15 {}
impl Level13T for Level15 {}
impl Level12T for Level15 {}
impl Level11T for Level15 {}
impl Level10T for Level15 {}
impl Level9T for Level15 {}
impl Level8T for Level15 {}
impl Level7T for Level15 {}
impl Level6T for Level15 {}
impl Level5T for Level15 {}
impl Level4T for Level15 {}
impl Level3T for Level15 {}
impl Level2T for Level15 {}
impl Level1T for Level15 {}
impl Level0T for Level15 {}

impl ShareLevelT for Level15 {
    fn get_level(&self) -> ShareLevelE {
        self.level
    }
}

impl Level15 {
    pub fn new() -> Level15 {
        Level15 {
            level: ShareLevelE::Level15,
        }
    }
}
