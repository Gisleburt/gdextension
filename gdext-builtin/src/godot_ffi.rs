use std::ffi::c_void;

/// Adds methods to convert from and to Godot FFI pointers.
pub trait GodotFfi {
    /// Construct from Godot opaque pointer.
    unsafe fn from_sys(opaque_ptr: *mut c_void) -> Self;

    /// Construct uninitialized opaque data, then initialize it with `init` function.
    unsafe fn from_sys_init(init: impl FnOnce(*mut c_void)) -> Self;

    /// Return Godot opaque pointer, for an immutable operation.
    ///
    /// Note that this is a `*mut` pointer despite taking `&self` by shared-ref.
    /// This is because most of Godot's native API is not const-correct. This can still
    /// enhance user code (calling `sys_mut` ensures no aliasing at the time of the call).
    fn sys(&self) -> *mut c_void;

    /// Return Godot opaque pointer, for a mutable operation.
    ///
    /// Should usually not be overridden; behaves like `sys()` but ensures no aliasing
    /// at the time of the call (not necessarily during any subsequent modifications though).
    fn sys_mut(&mut self) -> *mut c_void {
        self.sys()
    }
}

/// Implements the `GodotFfi` methods for a type with `Opaque` data that stores a pointer type
/// (e.g. string, object).
///
/// Expects a `from_opaque()` constructor and a `opaque` field.
#[macro_export]
macro_rules! impl_ffi_as_pointer {
    () => {
        unsafe fn from_sys(opaque_ptr: *mut std::ffi::c_void) -> Self {
            let opaque = std::mem::transmute(opaque_ptr);
            Self::from_opaque(opaque)
        }

        unsafe fn from_sys_init(init: impl FnOnce(*mut std::ffi::c_void)) -> Self {
            let mut raw = std::mem::MaybeUninit::uninit();
            init(std::ptr::read(raw.as_mut_ptr() as *mut *mut std::ffi::c_void));

            Self::from_opaque(raw.assume_init())
        }

        fn sys(&self) -> *mut std::ffi::c_void {
            unsafe { std::mem::transmute(self.opaque) }
        }
    };
}

/// Implements the `GodotFfi` methods for a type with `Opaque` data that stores a value type
/// (e.g. variant, vector2).
///
/// Expects a `from_opaque()` constructor and a `opaque` field.
#[macro_export]
macro_rules! impl_ffi_as_value {
    () => {
        unsafe fn from_sys(opaque_ptr: *mut std::ffi::c_void) -> Self {
            let opaque = std::ptr::read(opaque_ptr as *mut _);
            Self::from_opaque(opaque)
        }

        unsafe fn from_sys_init(init: impl FnOnce(*mut std::ffi::c_void)) -> Self {
            let mut raw = std::mem::MaybeUninit::uninit();
            init(raw.as_mut_ptr() as *mut std::ffi::c_void);

            Self::from_opaque(raw.assume_init())
        }

        fn sys(&self) -> *mut std::ffi::c_void {
            &self.opaque as *const _ as *mut std::ffi::c_void
        }
    };
}
