#![feature(c_variadic, extern_types, ptr_wrapping_offset_from, thread_local)]

pub mod core;
pub mod descriptor;
pub mod hotplug;
pub mod io;
pub mod os;
pub mod strerror;
pub mod sync;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
