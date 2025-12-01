use std::ptr;

pub mod database;

pub trait ServiceModule {
	fn name(&self) -> &'static str;
	fn init(&self);
	fn destroy(&self);
}

pub struct ServiceLoader {
	modules: Vec<*mut dyn ServiceModule>
}

#[allow(unsafe_op_in_unsafe_fn)]
impl ServiceLoader {
	pub fn new() -> Self {
		return Self {
			modules: Vec::new()
		};
	}

	pub unsafe fn insmod(&mut self, module: *mut dyn ServiceModule) {
		self.modules.push(module);
	}

	pub unsafe fn load(&mut self) {
		for ptr in &self.modules {
			(**ptr).init();
			println!("loaded module: {}", (**ptr).name());
		}
	}

	pub unsafe fn unload(&mut self) {
		for ptr in &self.modules {
			(**ptr).destroy();
			drop(Box::from_raw(*ptr));

			println!("unloaded module: {}", (**ptr).name());
		}

		self.modules.clear();
	}
}

static mut LOADER: *mut ServiceLoader = ptr::null_mut();

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn init_loader() {
    LOADER = Box::into_raw(Box::new(ServiceLoader::new()));
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn get_loader() -> &'static mut ServiceLoader {
	return &mut *LOADER;
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn destroy_loader() {
    (*LOADER).unload();
    drop(Box::from_raw(LOADER));

    LOADER = ptr::null_mut();
}

#[macro_export]
macro_rules! module_init {
	($init_fn:ident) => {
		#[allow(unsafe_op_in_unsafe_fn)]
		pub unsafe fn __module_init() -> *mut dyn $crate::modules::ServiceModule {
			return Box::into_raw(Box::new($init_fn()));
		}
	};
}

#[macro_export]
macro_rules! declare_module {
	($name:ident) => {
		static mut INSTANCE: *mut $name = std::ptr::null_mut();

		#[allow(unsafe_op_in_unsafe_fn)]
		pub unsafe fn instance() -> &'static $name {
			return &*INSTANCE;
		}

		#[allow(unsafe_op_in_unsafe_fn)]
		unsafe fn __init() -> $name {
			let module = $name::new();
			INSTANCE = &module as *const $name as *mut $name;

			return module;
		}
	};
}
