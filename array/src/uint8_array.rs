use std::convert::TryInto;

use std::alloc::{ Layout, alloc, dealloc };

#[repr(C)]
pub struct Uint8Array{
    _data: *mut u8,
    _length: usize,
    _capacity: usize,
}

impl Uint8Array {
    pub fn new() -> Self {
        Self {
            _data: std::ptr::null::<u8>() as *mut u8,
            _length: 0,
            _capacity: 0,
        }
    }

	pub fn as_ptr(&self) -> *const u8 {
		self._data as *const u8
	}

	pub fn as_mut_ptr(&self) -> *mut u8 {
		self._data
	}

	pub fn len(&self) -> usize {
		if self._length <= (isize::MAX as usize) {
			return self._length
		}
		else {
			panic!("Length is greater than the pointer size");
		}
	}

	pub fn cap(&self) -> usize {
		if self._capacity <= (isize::MAX as usize) {
			return self._capacity;
		}
		else {
			panic!("Capacity is greater than the pointer size");
		}
	}

	pub fn set_len(&mut self, len: usize) {
		if self._length <= isize::MAX.try_into().unwrap(){
			self._length = len;
		}
		else {
			panic!("Length cannot be greater than isize::MAX");
		}
	}

	pub fn set_cap(&mut self, cap: usize) {
		if self._capacity <= isize::MAX.try_into().unwrap(){
			self._capacity = cap;
		}
		else {
			panic!("Length cannot be greater than isize::MAX");
		}
	}
    
    pub fn push(&mut self, val: u8) -> () {
        unsafe {
            if self.len() == self.cap() {
                if self.cap() == 0 {
                    self.grow(10);
                }
                else {
                    self.grow(self.cap() * 2);
                }
            }
            *self.as_mut_ptr().add(self.len()) = val;
            self.set_len(self.len() + 1);
        }
    }
    
    pub fn pop(&mut self) -> Option<u8> {
        if self.len() == 0 {
            None
        }
        else {
            unsafe {
                self.set_len(self.len() - 1);
                Some(std::ptr::read(self.as_ptr().add(self.len())))
            }
        }
    }
    
    pub fn insert(&mut self, index: usize, element: u8) {
        if index > self.len() {
            panic!("Invalid index");
        }
        else {
            unsafe {
                if self.len() == self.cap() {
                    self.grow(1);
                }
                std::ptr::copy(self.as_ptr().add(index), self.as_mut_ptr().add(index + 1), self.len() - index);
                *self.as_mut_ptr().add(index) = element;
                self.set_len(self.len() + 1);
            }
        }
    }
    
    pub fn remove(&mut self, index: usize) -> () {
        if index >= self.len() {
            panic!("Invalid index");
        }
        else {
            unsafe {
                std::ptr::copy(self.as_ptr().add(index + 1), self.as_mut_ptr().add(index), self.len() - index);
                self.set_len(self.len() - 1);
            }
        }
    }
    
    pub fn grow(&mut self, size: usize) -> () {
        unsafe {
            let layout = Layout::array::<u8>(self.cap() + size).unwrap();
            let ptr_u8 = alloc(layout);
            let arr_ptr = ptr_u8 as *mut u8;
            if self.cap() != 0 {
                std::ptr::copy(self.as_ptr(), arr_ptr, self.cap());
                let old_layout = Layout::array::<u8>(self.cap()).unwrap();
                dealloc(self.as_mut_ptr(), old_layout);
            }
            self._data = arr_ptr;
            self.set_cap(self.cap() + size);
        }
    }
    
    pub fn as_slice<'a>(&self) -> &'a [u8] {
        unsafe {
            std::slice::from_raw_parts(self.as_ptr(), self.len())
        }
    }
    
    pub fn as_array<const N: usize>(&self) -> [u8; N] {
        self.as_slice().try_into().unwrap()
    }
    
    pub fn get(&self, index: usize) -> Result<u8, &'static str> {
        if index < self.len() {
            unsafe {
                Ok(*self.as_ptr().add(index))
            }
        }
        else {
            Err("Index out of range!")
        }
    }
    
    pub fn set(&mut self, index: usize, element: u8) -> Result<(), &'static str> {
        if index >= self.len() {
            Err("Index out of range!")
        }
        else {
            unsafe {
                *(self.as_mut_ptr().add(index)) = element;
                Ok(())
            }
        }
    }
    
    pub fn write(&mut self, buffer: &[u8]) -> () {
        unsafe {
            self.grow(buffer.len());
            let ptr = buffer.as_ptr();
            std::ptr::copy(ptr, self.as_mut_ptr().add(self.len()), buffer.len());
            self.set_len(self.len() + buffer.len());
        }
    }
}

impl Drop for Uint8Array  {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::array::<u8>(self.cap()).unwrap();
            dealloc(self.as_mut_ptr(), layout);
        }
    }
}

impl std::fmt::Debug for Uint8Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "Uint8Array({}){:?}", self.len(), self.as_slice())
    }
}

#[no_mangle]
pub extern "C" fn Uint8Array_new() -> Uint8Array {
	Uint8Array::new()
}

#[no_mangle]
pub extern "C" fn Uint8Array_as_ptr(arr: *mut Uint8Array) -> *mut u8 {
	unsafe {
		if !arr.is_null() {
			(&*arr).as_mut_ptr()
		}
		else {
			std::ptr::null_mut()
		}
	}
}

#[no_mangle]
pub extern "C" fn Uint8Array_len(arr: *const Uint8Array) -> usize {
	unsafe {
		if !arr.is_null() {
			(&*arr).len()
		}
		else {
			0
		}
	}
}

#[no_mangle]
pub extern "C" fn Uint8Array_push(arr: *mut Uint8Array, element: u8) {
	unsafe {
		(&mut *arr).push(element);
	}
}

/// Removes the last item from the list and returns it.
/// If the list is empty, it returns 0.
/// However, a list can have 0 as its last value and
/// care should be taken while using this method.
/// 
/// Firstly check the length of the array with the len() method.
/// If it is not 0, call the pop method and do something with the
/// value, otherwise, don't call the pop method.
/// 
/// ```c
/// void main() {
/// 	Uint8Array arr = Uint8Array_new();
/// 	Uint8Array_push(&arr, 12);
/// 	Uint8Array_push(&arr, 14);
/// 	if (Uint8Array_length(&arr) != 0) {
/// 		printf("The last value is: ", Uint8Array_pop(&arr));
/// 	}
/// 	else {
/// 		printf("The array is empty");
/// 	}
/// }
/// ```
#[no_mangle]
pub extern "C" fn Uint8Array_pop(arr: *mut Uint8Array) -> u8 {
	unsafe {
		match (&mut *arr).pop() {
			Some(n) => n,
			None =>  0,
		}
	}
}