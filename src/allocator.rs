
extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};
use crate::paging::{Pager, Page, PAGE_SIZE};
use spin::RwLock;
use alloc::sync::Arc;
use core::cell::{RefCell, Cell};
use core::ops::DerefMut;
use core::ptr::{null, null_mut};
use crate::mem_sizes::MemorySize;
use core::borrow::Borrow;

struct BasicAllocator {
    pager: Cell<Pager>,
}

impl BasicAllocator {

    pub fn new() -> Self {
        Self {
            pager: Cell::new(Pager::new())
        }
    }

    pub fn get_pager(&self) -> &mut Pager {
        unsafe {&mut *self.pager.as_ptr()}
    }

    pub fn add_page(&self) {
        self.get_pager().alloc_page();

        // self.current_page.replace(Some(page));
    }

    pub fn avg_usage(&self) -> f64{
        let pager = self.get_pager();

        let mut sum = pager.get_static_page().map_or(0.0,|p| p.usage());

        for page in pager.get_dynamic_pages() {
            sum += page.usage();
        }

        sum / (pager.get_dynamic_pages().len() + 1) as f64
    }

    pub fn find_page_with_space(&self, layout: Layout) -> *mut Page {
        let mut pager = self.get_pager();
        //let mut pager = unsafe { &mut  *(self.pager.as_ptr()) };

        if !pager.bootstrapped() {
            pager.bootstrap(
                usize::max(layout.size(), PAGE_SIZE)
            );
        }

        for page in pager.get_dynamic_pages_mut() {
            if page.can_fit(layout) {
                return page as *mut _;
            }
        }

        if pager.use_static_page(layout) {
            let page = {
                let p = pager.get_static_page_mut();
                match p {
                    None => {
                        pager.get_static_page_mut().unwrap()
                    },
                    Some(p) => {
                        p as *mut Page
                    },
                }
            };

            page
        } else{
            (if layout.size() < PAGE_SIZE {
                pager.alloc_page()
            } else {
                pager.alloc_large(layout)
            }) as *mut Page
        }
    }

    // pub unsafe grow_main_heap(&mut self, )
}

#[global_allocator]
static mut DUMMY_ALLOCATOR: BasicAllocator= BasicAllocator {
    pager: Cell::new(Pager::new())
};


unsafe impl GlobalAlloc for BasicAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let page = &mut *self.find_page_with_space(layout);
        /*{
            if self.get_allocated_to() + size >= self.current_page
                .borrow()
                .as_ref()
                .unwrap()
                .read()
                .len() {
                self.add_page();
                // TODO: Add support for larger size objects
            }
            }
        */

        if !self.get_pager().is_auto_expanding() && self.avg_usage() > 0.6 {
            self.get_pager().set_is_auto_expanding(true);
            self.add_page();
            self.get_pager().set_is_auto_expanding(false);
        }

        let position = {
            let mut start = page.used();
            while start % layout.align() != 0 {
                start += 1;
            }
            start
        };

        let output = (&mut page[position]) as *mut u8;
        *page.used_mut() = position + size;
        output
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Does not dealloc lol
    }


}

#[cfg(test)]
mod test {
    use alloc::boxed::Box;
    use alloc::vec;

    #[test]
    fn create_box() {

        let b = Box::new(42);
        assert_eq!(*b, 42);
    }

    #[test]
    fn large_vector_grow() {


        let mut v = vec![];
        for _ in 0..500_000 {
            v.push([0usize; 512])
        }


    }

}