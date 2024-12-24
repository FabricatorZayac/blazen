use buddy_alloc::{BuddyAllocParam, FastAllocParam, NonThreadsafeAlloc};

// These values can be tuned
const FAST_HEAP_SIZE: usize = 1 * 1024; // 1 KB
const HEAP_SIZE: usize = 1 * 1024; // 1 KB
const LEAF_SIZE: usize = 16;

static mut FAST_HEAP: [u8; FAST_HEAP_SIZE] = [0u8; FAST_HEAP_SIZE];
static mut HEAP: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];

#[global_allocator]
static ALLOC: NonThreadsafeAlloc = unsafe {
    let fast_ptr = &raw const FAST_HEAP[0];
    let fast_param = FastAllocParam::new(fast_ptr, FAST_HEAP_SIZE);

    let buddy_ptr = &raw const HEAP[0];
    let buddy_param = BuddyAllocParam::new(buddy_ptr, HEAP_SIZE, LEAF_SIZE);
    NonThreadsafeAlloc::new(fast_param, buddy_param)
};
