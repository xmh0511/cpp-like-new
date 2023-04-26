use std::{alloc::{Layout,self}, ptr::null_mut};
pub unsafe fn new_obj<T>(init_expr: T)-> * mut T{
	let layout = Layout::new::<T>();
	let raw_ptr = alloc::alloc(layout) as * mut T;
	std::ptr::write(raw_ptr, init_expr);
	raw_ptr
}

pub unsafe fn delete_obj<T>(ptr:* const T){
	let layout = Layout::new::<T>();
	alloc::dealloc(ptr as * mut u8, layout);
}

pub struct ArrayPtr<T>{
	raw_ptr:* mut T,
	size:usize,
	layout:Option<Layout>
}

impl<T> ArrayPtr<T>{
	pub fn as_slice(&self) -> Option<&[T]>{
		match self.layout{
			Some(_)=>{
				Some(unsafe {
					std::slice::from_raw_parts(self.raw_ptr, self.size)
				})
			}
			_=>{
				None
			}
		}
	}
	pub fn as_mut_slice(&self)-> Option<& mut [T]>{
		match self.layout{
			Some(_)=>{
				Some(unsafe {
					std::slice::from_raw_parts_mut(self.raw_ptr, self.size)
				})
			}
			_=>{
				None
			}
		}
	}
	pub fn len(&self)->usize{
		self.size
	}
}

/// new T[2]
pub unsafe fn new_arr<T:Clone + Default>(init_list:&[T], size:usize) -> ArrayPtr<T>{
	if init_list.len() > size{
		panic!("the number of initializer is greater than allocated size");
	}
	match Layout::array::<T>(size){
		Ok(layout)=>{
			let raw_ptr = alloc::alloc(layout.clone()) as * mut T;
			let mut start = raw_ptr;
			for i in init_list{
				std::ptr::write(start, i.clone());
				start = start.add(1);
			}
			let remaining_num = size - init_list.len();
			for _ in 0.. remaining_num{
				std::ptr::write(start, T::default());
				start = start.add(1);
			}
			ArrayPtr{raw_ptr:raw_ptr,size,layout: Some(layout)}
		}
		Err(_)=>{
			ArrayPtr{raw_ptr:null_mut(),size:0,layout:None}
		}
	}
}
pub unsafe fn delete_arr<T>(arr_ptr:ArrayPtr<T>){
	match arr_ptr.layout{
		Some(layout)=>{
			alloc::dealloc(arr_ptr.raw_ptr as * mut u8, layout);
		}
		_=>{}
	}
}

#[macro_export]
macro_rules! form_rust_arr_declarator_from_c_arr_declarator {
	($type:ty, $lt0:literal, $($lt:literal), +) => {
		[form_rust_arr_declarator_from_c_arr_declarator!($type, $($lt),+); $lt0]
	};
	($type:ty, $lt:literal)=>{
		[$type; $lt]
	}
}

#[macro_export]
macro_rules! new{
	($type:ty) => {
		{
			type T = $type;
			$crate::new_obj::<$type>(T::default())
		}
	};
	($type:ty { $init:expr })=>{
		$crate::new_obj::<$type>($init)
	};
	($type:ty [ $size:expr ])=>{
		$crate::new_arr::<$type>(&[],$size)
	};
	($type:ty [ $size:expr ]{ $($init:expr),* })=>{
		$crate::new_arr::<$type>(&[$($init),+],$size)
	};
	($type:ty [$size:expr] $([$lt:literal])+)=>{
		// i32 [e] [2] [3]
		//type T = [[i32;3];2];
		{
			type T = form_rust_arr_declarator_from_c_arr_declarator!($type, $($lt),+);
			$crate::new_arr::<T>(&[], $size)
		}
	};
	($type:ty [$size:expr] $([$lt:literal])+ { $($init:expr),* })=>{
		{
			type T = form_rust_arr_declarator_from_c_arr_declarator!($type, $($lt),+);
			$crate::new_arr::<T>(&[$($init),+], $size)
		}
	};
}

#[macro_export]
macro_rules! delete {
	($ptr:expr) => {
		$crate::delete_obj($ptr)
	};
	([] $ptr:expr)=>{
		$crate::delete_arr($ptr)
	};
}

#[cfg(test)]
mod test{
	#[test]
	fn test_obj(){
		unsafe{
			let default_ptr = new!(i32);
			assert_eq!(*default_ptr,0);
			let ptr = new!(i32 {10});
			assert_eq!(*ptr, 10);
			delete!(default_ptr);
			delete!(ptr);
		};
	}
	#[test]
	fn test_arr(){
		unsafe{
			let size = 2;
			let ptr = new!(i32[size]);
			assert_eq!(ptr.as_slice(),Some([0,0].as_slice()));
			let ptr_init = new!(i32[size]{1});
			assert_eq!(ptr_init.as_slice(),Some([1,0].as_slice()));
			delete!([] ptr);
			delete!([] ptr_init);
			let multiple_arr_ptr = new!(i32[size][3]);
			assert_eq!(multiple_arr_ptr.as_slice(),Some([[0,0,0],[0,0,0]].as_slice()));
			let multiple_arr_ptr_init = new!(i32[size][3]{[1,2,3]});
			assert_eq!(multiple_arr_ptr_init.as_slice(),Some([[1,2,3],[0,0,0]].as_slice()));
			delete!([] multiple_arr_ptr);
			delete!([] multiple_arr_ptr_init);
		};
	}
}