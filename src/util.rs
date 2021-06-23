use std::{mem::MaybeUninit, slice};

pub trait ContainerSizeGames<T> {
    unsafe fn shorten(self, len: usize, value: T) -> Self;
    unsafe fn shorten_with<F>(self, len: usize, f: F) -> Self
    where
        F: Fn() -> T;
    unsafe fn lengthen(self, len: usize, value: T) -> Self;
    unsafe fn lengthen_with<F>(self, len: usize, f: F) -> Self
    where
        F: Fn() -> T;
    fn change_len(self, len: usize, value: T) -> Self;

    unsafe fn get_copied(&self, index: usize) -> T;
}

impl<T> ContainerSizeGames<T> for Vec<T>
where
    T: Copy,
{
    unsafe fn shorten(mut self, len: usize, value: T) -> Self {
        debug_assert!(len <= self.len());

        // SAFETY: this is safe if `len` is assumed to be smaller than the current length of the vector because that means everything up to `len` is alraedy initialized and that `len` is smaller than the vectors capacity (those are the requirements of `set_len`).
        // I don't need to `drop` the ignored values (`self[len..self.len()]`) because `T` is `Copy` and thus is not `Drop`
        unsafe {
            self.set_len(len);
        }

        self.fill(value);

        self
    }

    unsafe fn shorten_with<F>(mut self, len: usize, f: F) -> Self
    where
        F: FnMut() -> T,
    {
        debug_assert!(len <= self.len());

        // SAFETY: this is safe if `len` is assumed to be smaller than the current length of the vector because that means everything up to `len` is alraedy initialized and that `len` is smaller than the vectors capacity (those are the requirements of `set_len`).
        // I don't need to `drop` the ignored values (`self[len..self.len()]`) because `T` is `Copy` and thus is not `Drop`
        unsafe {
            self.set_len(len);
        }

        self.fill_with(f);

        self
    }

    unsafe fn lengthen(mut self, len: usize, value: T) -> Self {
        debug_assert!(len >= self.len());

        // reserve capacity
        self.reserve(len - self.len());

        // fill up with value
        let ptr = self.as_mut_ptr();

        // SAFETY: ptr was just returned from `Vec::as_mut_ptr` so it should be fine, len is longer than `Vec::len` and thus uninitialized but that is Ok because the slice I'm creating is of `MaybeUninit`s
        let slice: &mut [MaybeUninit<T>] = unsafe { slice::from_raw_parts_mut(ptr as *mut _, len) };
        slice.fill(MaybeUninit::new(value));

        // SAFETY: the capacity needed was reserved and the raw slice was filled with `value` and thus is initialized
        unsafe {
            self.set_len(len);
        }

        self
    }

    unsafe fn lengthen_with<F>(mut self, len: usize, f: F) -> Self
    where
        F: Fn() -> T,
    {
        debug_assert!(len >= self.len());

        // reserve capacity
        self.reserve(len - self.len());

        // fill up with value
        let ptr = self.as_mut_ptr();

        // SAFETY: ptr was just returned from `Vec::as_mut_ptr` so it should be fine, len is longer than `Vec::len` and thus uninitialized but that is Ok because the slice I'm creating is of `MaybeUninit`s
        let slice: &mut [MaybeUninit<T>] = unsafe { slice::from_raw_parts_mut(ptr as *mut _, len) };
        slice.fill_with(|| MaybeUninit::new(f()));

        // SAFETY: the capacity needed was reserved and the raw slice was filled with `value` and thus is initialized
        unsafe {
            self.set_len(len);
        }

        self
    }

    fn change_len(self, len: usize, value: T) -> Self {
        if len > self.len() {
            unsafe { self.lengthen(len, value) }
        } else {
            unsafe { self.shorten(len, value) }
        }
    }

    unsafe fn get_copied(&self, index: usize) -> T {
        *self.get_unchecked(index)
    }
}
