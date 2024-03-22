/* automatically generated by rust-bindgen 0.69.4 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tray {
    pub icon: *mut ::std::os::raw::c_char,
    pub menu: *mut tray_menu,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tray_menu {
    pub text: *mut ::std::os::raw::c_char,
    pub disabled: ::std::os::raw::c_int,
    pub checked: ::std::os::raw::c_int,
    pub cb: ::std::option::Option<unsafe extern "C" fn(arg1: *mut tray_menu)>,
    pub context: *mut ::std::os::raw::c_void,
    pub submenu: *mut tray_menu,
}
extern "C" {
    #[link_name = "tray_update__extern"]
    pub fn tray_update(tray: *mut tray);
}
extern "C" {
    #[link_name = "tray_init__extern"]
    pub fn tray_init(tray: *mut tray) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "tray_loop__extern"]
    pub fn tray_loop(blocking: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "tray_exit__extern"]
    pub fn tray_exit();
}
