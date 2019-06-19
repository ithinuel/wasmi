extern crate atomic;

use alloc::sync::{Arc, Mutex};

pub use alloc::sync::{
    Arc as MyRc, Weak as MyWeak, MutexGuard as MyRef,
};
pub use self::atomic::{
    Atomic, Ordering::Relaxed as Ordering,
};

/// Thread-safe wrapper which can be used in place of a `RefCell`.
#[derive(Debug)]
pub struct MyRefCell<T>(Arc<Mutex<T>>);

impl<T> MyRefCell<T> {
    /// Create new wrapper object.
    pub fn new(obj: T) -> MyRefCell<T> {
        MyRefCell(Arc::new(Mutex::new(obj)))
    }

    /// Borrow a `MyRef` to the inner value.
    pub fn borrow(&self) -> ::MyRef<T> {
        self.0.lock().expect("bar")
    }

    /// Borrow a mutable `MyRef` to the inner value.
    pub fn borrow_mut(&self) -> ::MyRef<T> {
        self.0.lock().expect("bar")
    }
}

/// Thread-safe wrapper which can be used in place of a `Cell`.
#[derive(Debug)]
pub struct MyCell<T>(Atomic<T>) where T: Copy;

impl<T> MyCell<T> where T: Copy {
    /// Create new wrapper object.
    pub fn new(obj: T) -> MyCell<T> {
        MyCell(Atomic::new(obj))
    }

    /// Returns the inner value.
    pub fn get(&self) -> T {
        self.0.load(::Ordering)
    }

    /// Sets the inner value.
    pub fn set(&self, val: T) {
        self.0.store(val, ::Ordering);
    }
}