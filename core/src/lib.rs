use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod fetch;

#[no_mangle]
pub fn wallpaper_pls(path_ref: *const c_char, width: u32, height: u32) {
    let path = unsafe { CStr::from_ptr(path_ref) }.to_str().unwrap();
    fetch::render_wallpaper(&path, width, height);
}

#[no_mangle]
pub extern fn just_do_it() {
    let planet = fetch::assemble(4);
    planet.unwrap().save("out.png");
}

#[no_mangle]
pub extern fn save_planet(path_ref: *const c_char, level: u32) -> *const i8 {
    //let planet = assemble(level);
    let path_cstr = unsafe { CStr::from_ptr(path_ref) };

    let result = path_cstr.to_str().and_then( |path| {
        println!("Path is valid, {}", path);
        Ok(fetch::assemble(level).and_then( |rendered_image| {
            Ok(rendered_image.save(path))
        }))
    });

    let x = match result {
        Ok(s) => String::from("All Good"),
        Err(e) => format!("{}", e)
    };

    let code = CString::new(x).unwrap();

    let ptr = code.as_ptr();
    std::mem::forget(code); //Leak the string

    ptr
}