use std::ptr;
use std::slice;

use libc::malloc;

use naga::front;
use naga::valid;
use naga::back;

#[unsafe(no_mangle)]
pub extern "C" fn NagaSpirvToWgsl(SpirvData: *const u32, SpirvSize: usize) -> *mut i8 {
	unsafe {
		if SpirvData.is_null() || SpirvSize == 0 {
			return ptr::null_mut();
		}

		let spirv_words = {
			slice::from_raw_parts(SpirvData, SpirvSize)
		};

		let spirv_bytes = {
			slice::from_raw_parts(
				spirv_words.as_ptr() as *const u8,
				spirv_words.len() * std::mem::size_of::<u32>(),
			)
		};

		let module = match front::spv::parse_u8_slice(spirv_bytes, &front::spv::Options::default()) {
			Ok(m) => m,
			Err(_) => {
				return ptr::null_mut();
			}
		};
		
		let mut validator = valid::Validator::new(
			valid::ValidationFlags::all(),
			valid::Capabilities::all(),
		);

		let module_info = match validator.validate(&module) {
			Ok(info) => info,
			Err(_) => {
				return ptr::null_mut();
			}
		};

		let wgsl = match back::wgsl::write_string(&module, &module_info, back::wgsl::WriterFlags::empty()) {
			Ok(s) => s,
			Err(_) => {
				return ptr::null_mut();
			}
		};

		let mut bytes = wgsl.into_bytes();
		bytes.push(0);
		let total_size = bytes.len();
		
		let out = malloc(total_size) as *mut i8;
		ptr::copy_nonoverlapping(bytes.as_ptr(), out as *mut u8, total_size);
		return out;
	}
}
