use super::share_level::ShareLevelT;

pub struct ShareRoot<L: ShareLevelT> {
    // TODO
    level: L,
}

impl<L: ShareLevelT> ShareRoot<L> {
    /// create a ShareRoot is unsafe, must set the right level, or BUG
    #[allow(unsafe_code)]
    pub unsafe fn new(level: L) -> ShareRoot<L> {
        // TODO
        ShareRoot { level }
    }

    pub fn get_level(&self) -> L {
        // TODO
        self.level
    }

    // lock API

    pub fn lock<T, F>(&mut self, f: F) -> T
    where
        F: Fn() -> T,
    {
        // TODO
        f()
    }

    pub fn lock_mut<T, F>(&mut self, mut f: F) -> T
    where
        F: FnMut() -> T,
    {
        // TODO
        f()
    }
}
