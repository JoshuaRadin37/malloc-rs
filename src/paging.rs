

use crate::mem_sizes::MemorySize;
use alloc::vec::Vec;
use core::ptr::{null, null_mut, read_unaligned};
use spin::{Mutex, RwLock};



use lazy_static::lazy_static;
use core::ffi::c_void;

use libc::{
    size_t,
    c_int,
    off_t,

};
/*
extern {
    pub fn mmap(
        addr: *mut c_void,
        len: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t
    ) -> *mut c_void;
}

 */

use memmap;
use core::convert::TryInto;
use memmap::{MmapMut, MmapOptions};
use core::ops::{Deref, DerefMut, Index, IndexMut};
use alloc::sync::Arc;
use core::alloc::Layout;
use alloc::boxed::Box;
use core::borrow::BorrowMut;
use alloc::collections::LinkedList;


struct Heap {
    start: usize,
    end: usize
}



pub const PAGE_SIZE: usize = 4096;

static GLOBAL_ALLOC: Mutex<()>= Mutex::new(());

pub struct Page{
    map: MmapMut,
    used: usize
}

impl Page {
    pub fn new(map: MmapMut) -> Self {
        Self {
            map,
            used: 0
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn used(&self) -> usize {
        self.used
    }

    pub fn used_mut(&mut self) -> &mut usize {
        &mut self.used
    }

    pub fn can_fit(&self, layout: Layout) -> bool {
        layout.size() <= (self.len() - self.used)
    }

    pub fn usage(&self) -> f64 {
        self.used() as f64 / self.len() as f64
    }
}

pub struct Pager {
    static_page: Option<Page>,
    use_dynamic: bool,
    auto_expanding: bool,
    dynamic_pages: LinkedList<Page>
}



impl Pager {

    pub const fn new() -> Self {
        Self {
            static_page: None,
            use_dynamic: false,
            auto_expanding: false,
            dynamic_pages: LinkedList::new()
        }
    }

    pub fn bootstrap(&mut self, size: usize) {
        let mut result = memmap::MmapMut::map_anon(size).expect("Can't map anymore memory");
        let page = Page::new(result);
        self.static_page = Some(page);
        self.dynamic_pages = LinkedList::new();
        self.use_dynamic = true;
        self.alloc_page();

    }

    pub fn is_auto_expanding(&self) -> bool {
        self.auto_expanding
    }

    pub fn set_is_auto_expanding(&mut self, val: bool) {
        self.auto_expanding = val
    }

    fn get_map(layout: Layout) -> MmapMut {
        let layout = if layout.size() < PAGE_SIZE {
            Layout::from_size_align(PAGE_SIZE, PAGE_SIZE).expect("Matches value")
        } else {
            let size = layout.size();
            let divided = size / PAGE_SIZE;
            let fixed_size = PAGE_SIZE * ((divided + 1) / 2 * 3);
            Layout::from_size_align(fixed_size, PAGE_SIZE).expect("Matches expectations")
        };

        MmapOptions::new().len(layout.size()).map_anon().expect("Error creating map")
    }

    pub fn alloc_page(&mut self) -> &mut Page{
       // memmap::MmapOptions::new().stack()

        self.alloc_large(Layout::from_size_align(PAGE_SIZE, PAGE_SIZE)
            .expect("Matches expectations"))
            /*
        let mut result = memmap::MmapMut::map_anon(PAGE_SIZE).expect("Can't map anymore memory");
        let page = Page::new(result);


        if !self.bootstrapped() {
            self.bootstrap(PAGE_SIZE)
        }

        if self.use_dynamic() {
            self.dynamic_pages.push_back(page);
            self.dynamic_pages.back_mut().unwrap()
        } else {
            self.static_page = Some(page);
            if let Some(page) = &mut self.static_page {
                page
            } else {
                unreachable!()
            }
        }

             */

    }


    pub fn alloc_large(&mut self, mem_size: Layout) -> &mut Page {
        GLOBAL_ALLOC.lock();

        let mut result = Self::get_map(mem_size);
        let page = Page::new(result);

        if !self.bootstrapped() {
            self.bootstrap(PAGE_SIZE.into())
        }

        if self.use_dynamic() {
            self.dynamic_pages.push_back(page);
            self.dynamic_pages.back_mut().unwrap()
        } else {
            self.static_page = Some(page);
            if let Some(page) = &mut self.static_page {
                page
            } else {
                unreachable!()
            }
        }
    }

    fn use_dynamic(&self) -> bool {
        self.use_dynamic // && self.dynamic_pages.len() > 0
    }

    pub fn get_pages(&self) -> Vec<&Page> {
        unimplemented!()
    }

    pub fn get_static_page(&self) -> Option<&Page> {
        self.static_page.as_ref()
    }

    pub fn get_static_page_mut(&mut self) -> Option<&mut Page> {
        self.static_page.as_mut()
    }

    pub fn get_dynamic_pages(&self) -> &LinkedList<Page> {
        &self.dynamic_pages
    }

    pub fn get_dynamic_pages_mut(&mut self) -> &mut LinkedList<Page> {
        &mut self.dynamic_pages
    }

    pub fn bootstrapped(&self) -> bool {
        self.static_page.is_some()
    }

    pub fn use_static_page(&self, size: Layout) -> bool {
        self.static_page.as_ref().expect("Has to exist").can_fit(size) ||
        self.dynamic_pages.is_empty()
    }

}

impl Deref for Page {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        & *self.map
    }
}

impl DerefMut for Page {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.map
    }
}

impl Index<usize> for Page {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        & (**self)[index]
    }
}

impl IndexMut<usize> for Page {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut (**self)[index]
    }
}

#[cfg(test)]
mod test {
    use crate::paging::{Pager, Page};
    use core::mem::size_of;
    use crate::println;

    #[test]
    fn alloc_page() {

        /*
        let mut pager: Pager = Pager::new();
        let page = pager.alloc_page().read();
        println!("Size of page: {}", size_of::<Page>());

        page[0] = b'\0';

         */

    }
}
