use core::{fmt::Debug, marker::PhantomData, ops::Deref, ptr::NonNull};

#[repr(C)]
pub struct Slice<'a, T> {
    ptr: NonNull<T>,
    len: usize,
    _p: PhantomData<&'a [T]>,
}
impl<'a, T> Clone for Slice<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, T> Copy for Slice<'a, T> {}

impl<'a, T> From<&'a [T]> for Slice<'a, T> {
    fn from(value: &'a [T]) -> Self {
        Self {
            ptr: core::ptr::NonNull::from(value).cast(),
            len: value.len(),
            _p: PhantomData,
        }
    }
}

impl<'a, T> From<Slice<'a, T>> for &'a [T] {
    fn from(value: Slice<'a, T>) -> Self {
        value.as_slice()
    }
}

impl<'a, T: Eq> PartialEq<Self> for Slice<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}
impl<'a, T: Eq> Eq for Slice<'a, T> {}

impl<'a, T: Debug> Debug for Slice<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<'a, T> Slice<'a, T> {
    pub fn as_slice(self) -> &'a [T] {
        unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<'a, T> Deref for Slice<'a, T> {
    type Target = [T];

    fn deref(self: &'_ Slice<'a, T>) -> &'a [T] {
        (*self).as_slice()
    }
}
