//! Ist gerade ein Mikrofon oder eine Kamera in Benutzung – egal von welcher App?
//!
//! Fragt nur den Status `DeviceIsRunningSomewhere` ab. Es wird nichts
//! aufgenommen, deshalb braucht es auch keine Mikrofon-/Kamera-Freigabe.

use std::ffi::c_void;

#[repr(C)]
struct PropertyAddress {
    selector: u32,
    scope: u32,
    element: u32,
}

const fn fourcc(s: &[u8; 4]) -> u32 {
    u32::from_be_bytes(*s)
}

const SYSTEM_OBJECT: u32 = 1;
const PROP_DEVICES: u32 = fourcc(b"dev#");
const PROP_STREAMS: u32 = fourcc(b"stm#");
const PROP_RUNNING_SOMEWHERE: u32 = fourcc(b"gone");
const SCOPE_GLOBAL: u32 = fourcc(b"glob");
const SCOPE_INPUT: u32 = fourcc(b"inpt");
const ELEMENT_MAIN: u32 = 0;

#[link(name = "CoreAudio", kind = "framework")]
extern "C" {
    fn AudioObjectGetPropertyDataSize(
        id: u32,
        addr: *const PropertyAddress,
        qualifier_size: u32,
        qualifier: *const c_void,
        out_size: *mut u32,
    ) -> i32;
    fn AudioObjectGetPropertyData(
        id: u32,
        addr: *const PropertyAddress,
        qualifier_size: u32,
        qualifier: *const c_void,
        io_size: *mut u32,
        out: *mut c_void,
    ) -> i32;
}

#[link(name = "CoreMediaIO", kind = "framework")]
extern "C" {
    fn CMIOObjectGetPropertyDataSize(
        id: u32,
        addr: *const PropertyAddress,
        qualifier_size: u32,
        qualifier: *const c_void,
        out_size: *mut u32,
    ) -> i32;
    fn CMIOObjectGetPropertyData(
        id: u32,
        addr: *const PropertyAddress,
        qualifier_size: u32,
        qualifier: *const c_void,
        data_size: u32,
        data_used: *mut u32,
        out: *mut c_void,
    ) -> i32;
}

fn audio_devices() -> Vec<u32> {
    let addr = PropertyAddress { selector: PROP_DEVICES, scope: SCOPE_GLOBAL, element: ELEMENT_MAIN };
    let mut size = 0u32;
    unsafe {
        if AudioObjectGetPropertyDataSize(SYSTEM_OBJECT, &addr, 0, std::ptr::null(), &mut size) != 0 {
            return Vec::new();
        }
        let mut ids = vec![0u32; size as usize / 4];
        if AudioObjectGetPropertyData(SYSTEM_OBJECT, &addr, 0, std::ptr::null(), &mut size, ids.as_mut_ptr().cast()) != 0 {
            return Vec::new();
        }
        ids.truncate(size as usize / 4);
        ids
    }
}

fn has_input_streams(dev: u32) -> bool {
    let addr = PropertyAddress { selector: PROP_STREAMS, scope: SCOPE_INPUT, element: ELEMENT_MAIN };
    let mut size = 0u32;
    unsafe { AudioObjectGetPropertyDataSize(dev, &addr, 0, std::ptr::null(), &mut size) == 0 && size > 0 }
}

fn audio_running(dev: u32) -> bool {
    let addr = PropertyAddress { selector: PROP_RUNNING_SOMEWHERE, scope: SCOPE_GLOBAL, element: ELEMENT_MAIN };
    let mut val = 0u32;
    let mut size = 4u32;
    unsafe {
        AudioObjectGetPropertyData(dev, &addr, 0, std::ptr::null(), &mut size, (&mut val as *mut u32).cast()) == 0
            && val != 0
    }
}

pub fn mic_in_use() -> bool {
    audio_devices().into_iter().any(|d| has_input_streams(d) && audio_running(d))
}

pub fn camera_in_use() -> bool {
    let addr = PropertyAddress { selector: PROP_DEVICES, scope: SCOPE_GLOBAL, element: ELEMENT_MAIN };
    let mut size = 0u32;
    unsafe {
        if CMIOObjectGetPropertyDataSize(SYSTEM_OBJECT, &addr, 0, std::ptr::null(), &mut size) != 0 {
            return false;
        }
        let mut ids = vec![0u32; size as usize / 4];
        let mut used = 0u32;
        if CMIOObjectGetPropertyData(SYSTEM_OBJECT, &addr, 0, std::ptr::null(), size, &mut used, ids.as_mut_ptr().cast()) != 0 {
            return false;
        }
        ids.truncate(used as usize / 4);
        ids.into_iter().any(|dev| {
            let a = PropertyAddress { selector: PROP_RUNNING_SOMEWHERE, scope: SCOPE_GLOBAL, element: ELEMENT_MAIN };
            let mut val = 0u32;
            let mut used = 0u32;
            CMIOObjectGetPropertyData(dev, &a, 0, std::ptr::null(), 4, &mut used, (&mut val as *mut u32).cast()) == 0
                && val != 0
        })
    }
}
