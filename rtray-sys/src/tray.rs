/* automatically generated by rust-bindgen 0.69.4 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tray {
    pub icon: *mut ::std::os::raw::c_char,
    pub menu: *mut tray_menu,
}
#[test]
fn bindgen_test_layout_tray() {
    const UNINIT: ::std::mem::MaybeUninit<tray> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tray>(),
        16usize,
        concat!("Size of: ", stringify!(tray))
    );
    assert_eq!(
        ::std::mem::align_of::<tray>(),
        8usize,
        concat!("Alignment of ", stringify!(tray))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).icon) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tray),
            "::",
            stringify!(icon)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).menu) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tray),
            "::",
            stringify!(menu)
        )
    );
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
#[test]
fn bindgen_test_layout_tray_menu() {
    const UNINIT: ::std::mem::MaybeUninit<tray_menu> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tray_menu>(),
        40usize,
        concat!("Size of: ", stringify!(tray_menu))
    );
    assert_eq!(
        ::std::mem::align_of::<tray_menu>(),
        8usize,
        concat!("Alignment of ", stringify!(tray_menu))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).text) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tray_menu),
            "::",
            stringify!(text)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).disabled) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tray_menu),
            "::",
            stringify!(disabled)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).checked) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tray_menu),
            "::",
            stringify!(checked)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cb) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tray_menu),
            "::",
            stringify!(cb)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).context) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tray_menu),
            "::",
            stringify!(context)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).submenu) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tray_menu),
            "::",
            stringify!(submenu)
        )
    );
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