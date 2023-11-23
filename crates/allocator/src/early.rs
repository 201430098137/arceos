use core::alloc::Layout;
use core::ptr::NonNull;

use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};


pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    byte_start_addr: usize,
    page_start_addr: usize,
    byte_end_addr: usize,
    page_end_addr: usize,
    allocated_byte_num: usize,
}


impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            byte_start_addr: 0,
            page_start_addr: 0,
            byte_end_addr: 0,
            page_end_addr: 0,
            allocated_byte_num: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.byte_start_addr = start;
        self.byte_end_addr = start;
        self.page_start_addr = start + size;
        self.page_end_addr = start + size;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        Ok(())
    }
}

impl<const PAGE_SIZE: usize>  ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        if self.byte_end_addr + layout.size() >= self.page_end_addr {
           return Err(AllocError::NoMemory);
        }
        let addr = self.byte_end_addr;
        self.byte_end_addr =  self.byte_end_addr + layout.size();
        self.allocated_byte_num += 1;
        Ok(unsafe { NonNull::new_unchecked(addr as *mut u8) })
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        self.allocated_byte_num -= 1;

        if self.allocated_byte_num == 0 {
            // all allocated byte is dealloc, free byte memory
            self.byte_end_addr = self.byte_start_addr;
        }
    }

    fn total_bytes(&self) -> usize {
        self.page_start_addr - self.byte_start_addr
    }

    fn used_bytes(&self) -> usize {
        self.byte_end_addr - self.byte_start_addr
    }

    fn available_bytes(&self) -> usize {
        self.page_end_addr - self.byte_end_addr
    }
}


impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        if self.page_end_addr - PAGE_SIZE * num_pages <= self.byte_end_addr {
            return Err(AllocError::NoMemory);
        }
        self.page_end_addr =  self.page_end_addr - PAGE_SIZE * num_pages;
        Ok(self.page_end_addr)
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        // TODO: not decrease `used_pages` if deallocation failed
    }

    fn total_pages(&self) -> usize {
        self.page_start_addr - self.byte_start_addr
    }

    fn used_pages(&self) -> usize {
        (self.page_start_addr - self.page_end_addr) / PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        (self.page_end_addr - self.byte_end_addr) / PAGE_SIZE
    }
}
