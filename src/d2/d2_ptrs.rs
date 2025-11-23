#![allow(dead_code)]

use crate::d2::d2_structs::*;
use std::os::raw::c_char;
use windows::Win32::Foundation::HMODULE;
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::{ GetModuleHandleA, GetProcAddress, LoadLibraryA};

// DLL numbers
const DLLNO_D2CLIENT: u32 = 0;
const DLLNO_D2COMMON: u32 = 1;
const DLLNO_D2GFX: u32 = 2;
const DLLNO_D2LANG: u32 = 3;
const DLLNO_D2WIN: u32 = 4;
const DLLNO_D2NET: u32 = 5;
const DLLNO_D2GAME: u32 = 6;
const DLLNO_D2LAUNCH: u32 = 7;
const DLLNO_FOG: u32 = 8;
const DLLNO_BNCLIENT: u32 = 9;
const DLLNO_STORM: u32 = 10;
const DLLNO_D2CMP: u32 = 11;

const DLLS: [&str; 12] = [
    "D2Client.DLL\0",
    "D2Common.DLL\0",
    "D2Gfx.DLL\0",
    "D2Lang.DLL\0",
    "D2Win.DLL\0",
    "D2Net.DLL\0",
    "D2Game.DLL\0",
    "D2Launch.DLL\0",
    "Fog.DLL\0",
    "BNClient.DLL\0",
    "Storm.DLL\0",
    "D2Cmp.DLL\0",
];

pub struct D2Pointers {
    // FOG functions
    pub fog_10021: Option<unsafe extern "fastcall" fn(*const c_char)>,
    pub fog_10019:
        Option<unsafe extern "fastcall" fn(*const c_char, DWORD, *const c_char, DWORD) -> DWORD>,
    pub fog_10101: Option<unsafe extern "fastcall" fn(DWORD, DWORD) -> DWORD>,
    pub fog_10089: Option<unsafe extern "fastcall" fn(DWORD) -> DWORD>,
    pub fog_10218: Option<unsafe extern "fastcall" fn() -> DWORD>,

    // D2CLIENT functions
    pub d2client_pd2_init_game_misc_i: Option<unsafe extern "stdcall" fn(DWORD, DWORD, DWORD)>,

    // D2COMMON functions
    pub d2common_load_act: Option<
        unsafe extern "stdcall" fn(
            DWORD,
            DWORD,
            DWORD,
            DWORD,
            DWORD,
            DWORD,
            DWORD,
            DWORD,
            DWORD,
        ) -> *mut Act,
    >,
    pub d2common_pd2_add_room_data: Option<unsafe extern "stdcall" fn(*mut Act, i32, i32, i32, *mut Room1)>,
    pub d2common_pd2_remove_room_data: Option<unsafe extern "stdcall" fn(*mut Act, i32, i32, i32, *mut Room1)>,
    pub d2common_pd2_get_level: Option<unsafe extern "fastcall" fn(*mut ActMisc, DWORD) -> *mut Level>,
    pub d2common_pd2_init_data_tables: Option<unsafe extern "stdcall" fn(DWORD, DWORD, DWORD) -> DWORD>,
    pub d2common_pd2_init_level: Option<unsafe extern "stdcall" fn(*mut Level)>,
    pub d2common_pd2_unload_act: Option<unsafe extern "stdcall" fn(*mut Act)>,
    pub d2common_pd2_get_level_text: Option<unsafe extern "stdcall" fn(DWORD) -> *mut LevelTxt>,
    pub d2common_pd2_get_object_txt: Option<unsafe extern "stdcall" fn(DWORD) -> *mut ObjectTxt>,

    // D2WIN functions
    pub d2win_10086: Option<unsafe extern "fastcall" fn() -> DWORD>,
    pub d2win_10005:
        Option<unsafe extern "fastcall" fn(DWORD, DWORD, DWORD, *mut D2ClientStruct) -> DWORD>,

    // D2LANG functions
    pub d2lang_10008: Option<unsafe extern "fastcall" fn(DWORD, *const c_char, DWORD) -> DWORD>,

    // Variables
    pub p_storm_pd2_mpq_hash_table: *mut DWORD,
}

impl D2Pointers {
    pub fn new() -> Self {
        Self {
            fog_10021: None,
            fog_10019: None,
            fog_10101: None,
            fog_10089: None,
            fog_10218: None,
            d2client_pd2_init_game_misc_i: None,
            d2common_load_act: None,
            d2common_pd2_add_room_data: None,
            d2common_pd2_remove_room_data: None,
            d2common_pd2_get_level: None,
            d2common_pd2_init_data_tables: None,
            d2common_pd2_init_level: None,
            d2common_pd2_unload_act: None,
            d2common_pd2_get_level_text: None,
            d2common_pd2_get_object_txt: None,
            d2win_10086: None,
            d2win_10005: None,
            d2lang_10008: None,
            p_storm_pd2_mpq_hash_table: std::ptr::null_mut(),
        }
    }
}

unsafe fn get_dll_offset(dll_name: &str, offset: i32) -> Option<usize> {
    let pcstr = PCSTR::from_raw(dll_name.as_ptr());

    
    // Try to get already loaded module first
    let mut h_mod = match GetModuleHandleA(pcstr) {
        Ok(handle) => handle,
        Err(_) => HMODULE(0),
    };

    // If not loaded, try to load it
    if h_mod.0 == 0 {
        h_mod = match LoadLibraryA(pcstr) {
            Ok(handle) => handle,
            Err(e) => {
                log::error!(
                    "Failed to LoadLibraryA: {} (error code: {:?})",
                    dll_name.trim_end_matches('\0'),
                    e
                );
                return None;
            }
        };
    }

    if h_mod.0 == 0 {
        log::error!("Failed to load DLL: {}", dll_name.trim_end_matches('\0'));
        return None;
    }

    log::trace!(
        "Loaded DLL: {} at base address: 0x{:X}",
        dll_name.trim_end_matches('\0'),
        h_mod.0
    );

    if offset < 0 {
        // Negative offset means ordinal export
        let proc_addr = GetProcAddress(h_mod, PCSTR::from_raw((-offset) as *const u8));
        match proc_addr {
            Some(addr) => {
                log::trace!(
                    "  Found ordinal {} at 0x{:X}",
                    -offset,
                    addr as usize
                );
                Some(addr as usize)
            }
            None => {
                log::warn!(
                    "  Failed to find ordinal {} in {}",
                    -offset,
                    dll_name.trim_end_matches('\0')
                );
                None
            }
        }
    } else {
        // Positive offset is added to base address
        let addr = h_mod.0 as usize + offset as usize;
        log::trace!("  Offset 0x{:X} -> 0x{:X}", offset, addr);
        Some(addr)
    }
}

fn dlloffset(dll_no: u32, offset: i32) -> u32 {
    dll_no | ((offset as u32) << 8)
}

pub unsafe fn define_offsets(ptrs: &mut D2Pointers) -> Result<(), String> {
    log::info!("Loading D2 DLLs and resolving function pointers...");

    let mut critical_functions_missing = Vec::new();

    // FOG functions (critical)
    log::debug!("Loading Fog.dll functions...");
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_FOG as usize], -10021) {
        ptrs.fog_10021 = Some(std::mem::transmute(addr));
    } else {
        critical_functions_missing.push("FOG.10021");
    }
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_FOG as usize], -10019) {
        ptrs.fog_10019 = Some(std::mem::transmute(addr));
    }
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_FOG as usize], -10101) {
        ptrs.fog_10101 = Some(std::mem::transmute(addr));
    } else {
        critical_functions_missing.push("FOG.10101");
    }
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_FOG as usize], -10089) {
        ptrs.fog_10089 = Some(std::mem::transmute(addr));
    } else {
        critical_functions_missing.push("FOG.10089");
    }
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_FOG as usize], -10218) {
        ptrs.fog_10218 = Some(std::mem::transmute(addr));
    } else {
        critical_functions_missing.push("FOG.10218");
    }

    // D2CLIENT functions
    log::debug!("Loading D2Client.dll functions...");
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_D2CLIENT as usize], 0x4454B) {
        ptrs.d2client_pd2_init_game_misc_i = Some(std::mem::transmute(addr));
    }

    // D2COMMON functions (critical)
    log::debug!("Loading D2Common.dll functions...");
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_D2COMMON as usize], 0x3CB30) {
        ptrs.d2common_load_act = Some(std::mem::transmute(addr));
    } else {
        critical_functions_missing.push("D2COMMON.LoadAct");
    }
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_D2COMMON as usize], -10401) {
        ptrs.d2common_pd2_add_room_data = Some(std::mem::transmute(addr));
    }
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_D2COMMON as usize], -11099) {
        ptrs.d2common_pd2_remove_room_data = Some(std::mem::transmute(addr));
    }
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_D2COMMON as usize], -10207) {
        ptrs.d2common_pd2_get_level = Some(std::mem::transmute(addr));
    } else {
        critical_functions_missing.push("D2COMMON.GetLevel");
    }
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_D2COMMON as usize], -10943) {
        ptrs.d2common_pd2_init_data_tables = Some(std::mem::transmute(addr));
    } else {
        critical_functions_missing.push("D2COMMON.InitDataTables");
    }
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_D2COMMON as usize], -10322) {
        ptrs.d2common_pd2_init_level = Some(std::mem::transmute(addr));
    } else {
        critical_functions_missing.push("D2COMMON.InitLevel");
    }
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_D2COMMON as usize], -10868) {
        ptrs.d2common_pd2_unload_act = Some(std::mem::transmute(addr));
    }
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_D2COMMON as usize], -10014) {
        ptrs.d2common_pd2_get_level_text = Some(std::mem::transmute(addr));
    } else {
        critical_functions_missing.push("D2COMMON.GetLevelText");
    }
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_D2COMMON as usize], -10688) {
        ptrs.d2common_pd2_get_object_txt = Some(std::mem::transmute(addr));
    }

    // D2WIN functions (critical)
    log::debug!("Loading D2Win.dll functions...");
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_D2WIN as usize], -10086) {
        ptrs.d2win_10086 = Some(std::mem::transmute(addr));
    } else {
        critical_functions_missing.push("D2WIN.10086");
    }
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_D2WIN as usize], -10005) {
        ptrs.d2win_10005 = Some(std::mem::transmute(addr));
    } else {
        critical_functions_missing.push("D2WIN.10005");
    }

    // D2LANG functions (critical)
    log::debug!("Loading D2Lang.dll functions...");
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_D2LANG as usize], -10008) {
        ptrs.d2lang_10008 = Some(std::mem::transmute(addr));
    } else {
        critical_functions_missing.push("D2LANG.10008");
    }

    // STORM variables
    log::debug!("Loading Storm.dll variables...");
    if let Some(addr) = get_dll_offset(DLLS[DLLNO_STORM as usize], 0x53120) {
        ptrs.p_storm_pd2_mpq_hash_table = addr as *mut DWORD;
    } else {
        critical_functions_missing.push("STORM.MPQHashTable");
    }

    if !critical_functions_missing.is_empty() {
        log::error!(
            "Failed to load critical functions: {}",
            critical_functions_missing.join(", ")
        );
        return Err(format!(
            "Failed to load critical D2 functions. Missing: {}. \
             Make sure you're pointing to a valid Project Diablo 2 installation.",
            critical_functions_missing.join(", ")
        ));
    }

    log::info!("All D2 function pointers resolved successfully");
    Ok(())
}
