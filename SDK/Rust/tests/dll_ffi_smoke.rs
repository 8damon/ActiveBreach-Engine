#![cfg(windows)]

use std::ffi::{CString, c_void};
use std::path::PathBuf;
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA};

type FnAbViolationCount = unsafe extern "C" fn() -> u32;
type FnAbClearViolationHandler = unsafe extern "C" fn();
type FnAbSetViolationHandler = unsafe extern "C" fn(Option<extern "C" fn(u32)>);
type FnActivebreachLaunch = unsafe extern "C" fn() -> u32;

fn find_activebreach_dll() -> Option<PathBuf> {
    if let Ok(explicit) = std::env::var("AB_DLL_PATH") {
        let p = PathBuf::from(explicit);
        if p.is_file() {
            return Some(p);
        }
    }

    let exe = std::env::current_exe().ok()?;
    for dir in exe.ancestors() {
        let candidates = [
            dir.join("activebreach.dll"),
            dir.join("debug").join("activebreach.dll"),
            dir.join("release").join("activebreach.dll"),
        ];
        for candidate in candidates {
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    None
}

unsafe fn load_symbol<T>(module: *mut c_void, name: &str) -> Option<T> {
    let sym = CString::new(name).ok()?;
    let ptr = GetProcAddress(module as _, sym.as_ptr());
    if ptr.is_null() {
        return None;
    }
    Some(std::mem::transmute_copy(&ptr))
}

#[test]
fn dll_exports_and_basic_calls_work() {
    let dll_path = match find_activebreach_dll() {
        Some(p) => p,
        None => {
            eprintln!("skipping: activebreach.dll not found (set AB_DLL_PATH to run this test)");
            return;
        }
    };

    let dll_c = CString::new(dll_path.to_string_lossy().as_bytes())
        .expect("dll path contains interior NUL");

    unsafe {
        let module = LoadLibraryA(dll_c.as_ptr());
        assert!(!module.is_null(), "failed to load {}", dll_path.display());

        let ab_violation_count: FnAbViolationCount =
            load_symbol(module as _, "ab_violation_count").expect("missing export: ab_violation_count");
        let ab_clear_violation_handler: FnAbClearViolationHandler = load_symbol(
            module as _,
            "ab_clear_violation_handler",
        )
        .expect("missing export: ab_clear_violation_handler");
        let ab_set_violation_handler: FnAbSetViolationHandler = load_symbol(
            module as _,
            "ab_set_violation_handler",
        )
        .expect("missing export: ab_set_violation_handler");
        let activebreach_launch: FnActivebreachLaunch =
            load_symbol(module as _, "activebreach_launch").expect("missing export: activebreach_launch");
        let ab_call_name = CString::new("ab_call").unwrap();
        let _ab_call_ptr = GetProcAddress(module as _, ab_call_name.as_ptr());
        assert!(!_ab_call_ptr.is_null(), "missing export: ab_call");

        let _ = activebreach_launch();
        let _ = ab_violation_count();
        ab_clear_violation_handler();
        ab_set_violation_handler(None);

        // Do not unload after launch: ActiveBreach may have a live dispatcher thread
        // executing code from this module, and unloading can cause AV on process teardown.
        let _ = module;
    }
}
