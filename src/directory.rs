use std::ops::Drop;
use std::marker::PhantomData;

use utils::NewFromPtr;

use Database;
use Filenames;

use ffi;

#[derive(Debug)]
pub struct Directory<'d>(
    *mut ffi::notmuch_directory_t,
    PhantomData<&'d Database>,
);

impl<'d> Directory<'d>{
    pub fn child_directories(self: &'d Self) -> Filenames<'d>{
        Filenames::new(unsafe {
            ffi::notmuch_directory_get_child_directories(self.0)
        })
    }
}

impl<'d> NewFromPtr<*mut ffi::notmuch_directory_t> for Directory<'d> {
    fn new(ptr: *mut ffi::notmuch_directory_t) -> Directory<'d> {
        Directory(ptr, PhantomData)
    }
}

impl<'d> Drop for Directory<'d> {
    fn drop(self: &mut Self) {
        unsafe {
            ffi::notmuch_directory_destroy(self.0)
        };
    }
}

unsafe impl<'d> Send for Directory<'d>{}
unsafe impl<'d> Sync for Directory<'d>{}
