extern crate arrayvec;

use Buffer;
use BufferRef;
use ToBufferRef;
use self::arrayvec::ArrayVec;
use std::slice;

/// The intermediate step from a `ArrayVec` to a `BufferRef`.
pub struct ArrayVecBuffer<'data, const CAP: usize> {
    // Will only touch the length of the `ArrayVec` through this reference,
    // except in `ArrayVecBuffer::buffer`.
    vec: &'data mut ArrayVec<u8, CAP>,
    initialized: usize,
}

impl<'d, const CAP: usize> ArrayVecBuffer<'d, CAP> {
    fn new(vec: &'d mut ArrayVec<u8, CAP>) -> Self {
        Self {
            vec: vec,
            initialized: 0,
        }
    }
    fn buffer<'s>(&'s mut self) -> BufferRef<'d, 's> {
        let len = self.vec.len();
        let remaining = self.vec.capacity() - len;
        unsafe {
            let start = self.vec.as_mut_ptr().offset(len as isize);
            // This is unsafe, we now have two unique (mutable) references
            // to the same `ArrayVec`. However, we will only access
            // `self.vec.len` through `self` and only the contents through
            // the `BufferRef`.
            BufferRef::new(slice::from_raw_parts_mut(start, remaining),
                           &mut self.initialized)
        }
    }
}

impl<'d, const CAP: usize> Drop for ArrayVecBuffer<'d, CAP> {
    fn drop(&mut self) {
        let len = self.vec.len();
        unsafe {
            self.vec.set_len(len + self.initialized);
        }
    }
}

impl<'d, const CAP: usize> Buffer<'d> for &'d mut ArrayVec<u8, CAP> {
    type Intermediate = ArrayVecBuffer<'d, CAP>;
    fn to_to_buffer_ref(self) -> Self::Intermediate {
        ArrayVecBuffer::new(self)
    }
}

impl<'d, const CAP: usize> ToBufferRef<'d> for ArrayVecBuffer<'d, CAP> {
    fn to_buffer_ref<'s>(&'s mut self) -> BufferRef<'d, 's> {
        self.buffer()
    }
}
