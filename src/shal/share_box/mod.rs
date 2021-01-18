use super::share_level::ShareLevelT;
use super::ShareRoot;

pub struct ShareBox<T, L: ShareLevelT> {
    // TODO
    data: T,
    level: L,
}

impl<T, L: ShareLevelT> ShareBox<T, L> {
    pub fn new(data: T, level: L) -> ShareBox<T, L> {
        // TODO
        ShareBox { data, level }
    }

    pub fn get_level(&self) -> L {
        // TODO
        self.level
    }

    // get API

    pub fn get<R, F>(&self, root: &mut ShareRoot<L>, f: F) -> R
    where
        F: Fn(&T) -> R,
    {
        // TODO
        f(&self.data)
    }

    // TODO -> R
    pub fn get_mut<R, F>(&self, root: &mut ShareRoot<L>, f: F)
    where
        F: Fn(&mut T) -> R,
    {
        // TODO
        //f(&mut self.data)
    }
}
