extern crate libc;
use libc::{size_t, malloc, free};
use std::mem;

extern crate uefi;
use uefi::{Handle, Handles};

#[test]
fn handle_iterator() {
        let nhandles = 10;
        let sz = mem::size_of::<Handle>() as size_t;
        let hptr = unsafe { malloc(sz * nhandles) as *mut Handle };

        assert_eq!(hptr.is_null(), false);

        let handles = Handles::new(hptr, nhandles);
        let mut ctr = 0;

        let iter = handles.into_iter();

        assert_eq!(nhandles, iter.len());

        for _ in iter {
            ctr += 1;
        }

        assert_eq!(nhandles, ctr);

        unsafe {
            free(hptr as *mut libc::c_void);
        }
}

