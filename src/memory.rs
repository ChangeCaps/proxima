use std::{
    alloc::{self, Layout},
    borrow::Cow,
    ptr::{self, NonNull},
};

use crate::Word;

pub struct Memory {
    size: usize,
    base: NonNull<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            size: 0,
            base: NonNull::dangling(),
        }
    }

    pub fn with_size(size: usize) -> Self {
        let mut memory = Self::new();
        memory.grow(size);

        memory
    }

    pub fn grow(&mut self, size: usize) {
        let new_size = self.size + size;

        let new_layout = Layout::from_size_align(new_size, 1).unwrap();

        let ptr = if self.size == 0 {
            unsafe { alloc::alloc_zeroed(new_layout) }
        } else {
            let old_layout = Layout::from_size_align(self.size, 1).unwrap();

            let ptr = unsafe { alloc::alloc_zeroed(new_layout) };

            unsafe { ptr::copy_nonoverlapping(self.base.as_ptr(), ptr, self.size) };

            unsafe { alloc::dealloc(self.base.as_ptr(), old_layout) };

            ptr
        };

        let new_base = match NonNull::new(ptr) {
            Some(base) => base,
            None => alloc::handle_alloc_error(new_layout),
        };

        self.size = new_size;
        self.base = new_base;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn read(&self, ptr: u32, size: u8) -> Option<Word> {
        if ptr as usize + size as usize >= self.size {
            return None;
        }

        let bytes = match size {
            1 => {
                let read = unsafe { *self.base.as_ptr().add(ptr as usize) };
                [0, 0, 0, read]
            }
            2 => {
                let read = unsafe { *self.base.as_ptr().add(ptr as usize).cast::<[u8; 2]>() };
                [0, 0, read[0], read[1]]
            }
            4 => unsafe { *self.base.as_ptr().add(ptr as usize).cast::<[u8; 4]>() },
            _ => return None,
        };

        Some(Word::from_bytes(bytes))
    }

    pub fn read_ptr(&self, ptr: u32) -> Option<*mut u8> {
        if ptr as usize >= self.size {
            return None;
        }

        Some(unsafe { self.base.as_ptr().add(ptr as usize) })
    }

    pub fn read_bytes(&self, ptr: u32, len: u32) -> Option<&[u8]> {
        if ptr as usize + len as usize >= self.size {
            return None;
        }

        Some(unsafe {
            &*ptr::slice_from_raw_parts(self.base.as_ptr().add(ptr as usize), len as usize)
        })
    }

    pub fn read_string(&self, ptr: u32, len: u32) -> Option<Cow<'_, str>> {
        if ptr as usize + len as usize >= self.size() {
            return None;
        }

        let ptr = self.read_ptr(ptr)?;

        let utf8 = unsafe { &*ptr::slice_from_raw_parts(ptr, len as usize) };
        let string = String::from_utf8_lossy(utf8);

        Some(string)
    }

    pub fn write(&mut self, word: Word, ptr: u32, size: u8) {
        if ptr as usize + size as usize >= self.size {
            return;
        }

        let bytes = word.to_bytes();

        match size {
            1 => {
                unsafe { *self.base.as_ptr().add(ptr as usize) = bytes[3] };
            }
            2 => {
                unsafe { *self.base.as_ptr().add(ptr as usize).cast() = [bytes[2], bytes[3]] };
            }
            4 => {
                unsafe { *self.base.as_ptr().add(ptr as usize).cast() = bytes };
            }
            _ => {}
        }
    }

    pub fn write_bytes(&mut self, ptr: u32, bytes: &[u8]) {
        if ptr as usize + bytes.len() >= self.size {
            return;
        }

        unsafe {
            ptr::copy(
                bytes.as_ptr(),
                self.base.as_ptr().add(ptr as usize),
                bytes.len(),
            )
        };
    }

    pub fn write_string(&mut self, ptr: u32, string: &str) {
        self.write_bytes(ptr, string.as_bytes());
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        if self.size > 0 {
            let layout = Layout::from_size_align(self.size, 1).unwrap();

            unsafe { alloc::dealloc(self.base.as_ptr(), layout) };
        }
    }
}
