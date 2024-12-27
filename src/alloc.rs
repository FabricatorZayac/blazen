const HEAP_BASE: *const u8 = (0x19a0 + 8192 + 1024) as *const u8;
const SLOW_HEAP: *const u8 = HEAP_BASE.wrapping_add(8 * 1024 + 1);

use core::{alloc::{GlobalAlloc, Layout}, intrinsics::size_of, ops::{Deref, DerefMut}};

use buddy_alloc::{BuddyAllocParam, FastAllocParam, NonThreadsafeAlloc};

// These values can be tuned
const FAST_HEAP_SIZE: usize = 8 * 1024; // 8 KB
const HEAP_SIZE: usize = 32 * 1024; // 32 KB
const LEAF_SIZE: usize = 16;

#[global_allocator]
static ALLOC: NonThreadsafeAlloc = {
    let fast_param = FastAllocParam::new(HEAP_BASE, FAST_HEAP_SIZE);
    let buddy_param = BuddyAllocParam::new(SLOW_HEAP, HEAP_SIZE, LEAF_SIZE);

    NonThreadsafeAlloc::new(fast_param, buddy_param)
};

pub struct HeapPtr<T>(*mut T, Layout);

impl<T> HeapPtr<T> {
    pub fn new(value: T) -> Self {
        unsafe {
            let layout = Layout::from_size_align(size_of::<T>(), align_of::<T>()).unwrap();
            let ptr = ALLOC.alloc(layout) as *mut T;
            *ptr = value;
            Self(ptr, layout)
        }
    }

    pub fn leak(self) -> &'static mut T {
        unsafe {self.0.as_mut()}.unwrap()
    }
}

impl<T> Deref for HeapPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }.unwrap()
    }
}

impl<T> DerefMut for HeapPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }.unwrap()
    }
}

impl<T> Drop for HeapPtr<T> {
    fn drop(&mut self) {
        unsafe {
            ALLOC.dealloc(self.0 as *mut u8, self.1);
        }
    }
}
