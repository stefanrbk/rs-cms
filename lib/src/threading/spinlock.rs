use std::{
    cell::UnsafeCell, ops::{Deref, DerefMut}, sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Release},
    }
};

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

#[allow(unsafe_code)]
unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> SpinGuard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        SpinGuard { lock: self }
    }
}

pub struct SpinGuard<'a, T> {
    lock: &'a SpinLock<T>
}

impl<T> Deref for SpinGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe_block!("Return the value within the UnsafeCell" => &*self.lock.value.get())
    }
}
impl<T> DerefMut for SpinGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe_block!("Return the value within the UnsafeCell" => &mut *self.lock.value.get())
    }
}

impl<T> Drop for SpinGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}
