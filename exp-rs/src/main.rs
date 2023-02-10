extern "C" {
    pub fn scs_version() -> *const ::std::os::raw::c_char;
}

fn main() {
    unsafe {
        let version = std::ffi::CStr::from_ptr(scs_version());
        print!("{}", version.to_str().expect("bad string"));
    }
}
