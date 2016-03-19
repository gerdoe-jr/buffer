use Buffer;
use BufferRef;
use ToBufferRef;
use wildly_unsafe;

/// The intermediate step from a `BufferRef` to another `BufferRef`.
pub struct BufferRefBuffer<'ref_, 'data: 'ref_, 'size: 'ref_> {
    // Will only touch the length of the `BufferRef` through this reference,
    // except in `BufferRefBuffer::buffer`.
    buffer: &'ref_ mut BufferRef<'data, 'size>,
    initialized: usize,
}

impl<'r, 'd, 's> BufferRefBuffer<'r, 'd, 's> {
    fn new(buffer: &'r mut BufferRef<'d, 's>) -> BufferRefBuffer<'r, 'd, 's> {
        BufferRefBuffer {
            buffer: buffer,
            initialized: 0,
        }
    }
    fn buffer<'a>(&'a mut self) -> BufferRef<'d, 'a> {
        let len = *self.buffer.initialized_;
        let remaining = self.buffer.buffer.len() - len;
        unsafe {
            BufferRef::new(wildly_unsafe(&mut self.buffer.buffer[remaining..]),
                           &mut self.initialized)
        }
    }
}

impl<'r, 'd, 's> Drop for BufferRefBuffer<'r, 'd, 's> {
    fn drop(&mut self) {
        *self.buffer.initialized_ += self.initialized;
    }
}

impl<'r, 'd, 's> Buffer<'d> for &'r mut BufferRef<'d, 's> {
    type Intermediate = BufferRefBuffer<'r, 'd, 's>;
    fn to_to_buffer_ref(self) -> Self::Intermediate {
        BufferRefBuffer::new(self)
    }
}

impl<'r, 'd, 's> ToBufferRef<'d> for BufferRefBuffer<'r, 'd, 's> {
    fn to_buffer_ref<'a>(&'a mut self) -> BufferRef<'d, 'a> {
        self.buffer()
    }
}
