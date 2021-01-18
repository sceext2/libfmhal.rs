// atomic wrapper with API like core::sync::atomic
// will downgrade to lock (ShareBox) on platforms not support atomic memory operation (such as stm32f0)
// on platforms with native atomic support (such as stm32f1), performance should be better than lock

use super::share_level::ShareLevelT;
use super::ShareRoot;

pub trait ShareBoxAtomicT<T, L: ShareLevelT, A: ShareAtomicT<T>> {
    fn new(data: T, level: L) -> Self;
    fn get_level(&self) -> L;

    // bind ShareRoot
    fn bind(&self, root: &mut ShareRoot<L>) -> A;
}

pub trait ShareAtomicT<T> {
    // atomic operations

    fn load(&self) -> T;
    fn store(&self, val: T);
    fn swap(&self, val: T) -> T;
    fn compare_and_swap(&self, current: T, new: T) -> T;

    fn fetch_add(&self, val: T) -> T;
    fn fetch_sub(&self, val: T) -> T;
    fn fetch_and(&self, val: T) -> T;
    fn fetch_nand(&self, val: T) -> T;
    fn fetch_or(&self, val: T) -> T;
    fn fetch_xor(&self, val: T) -> T;
    fn fetch_max(&self, val: T) -> T;
    fn fetch_min(&self, val: T) -> T;

    fn fetch_update<F>(&self, f: F) -> Result<T, T>
    where
        F: FnMut(T) -> Option<T>;
}

pub struct ShareBoxAtomicUsize {
    // TODO
}
// impl ShareBoxAtomicT<usize, L, ShareAtomicUsize> for ShareBoxAtomicUsize

pub struct ShareAtomicUsize {
    // TODO
}
// impl ShareAtomicT<usize> for ShareAtomicUsize

pub struct ShareBoxAtomicIsize {
    // TODO
}

pub struct ShareAtomicIsize {
    // TODO
}
