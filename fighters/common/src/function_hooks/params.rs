use super::*;
use globals::*;

pub static mut FLOAT_OFFSET: usize = 0x4E53C0;

pub static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];

pub static mut INT_OFFSET : usize = 0x4E5380;

pub static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

#[skyline::hook(offset=FLOAT_OFFSET)]
pub unsafe fn get_param_float_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
	let mut boma = *((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
	let boma_reference: &mut BattleObjectModuleAccessor = &mut *boma;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if boma_reference.is_fighter() {
        if boma_reference.kind() == *FIGHTER_KIND_IKE {
            if param_type == hash40("param_special_lw") {
                if param_hash == hash40("special_lw_attack_power_limit") {
                    if VarModule::is_flag(boma_reference.object(), vars::ike::instance::STORED_AETHER) {
                        return 25.0;
                    }
                }
            }
        }
    } else if boma_reference.is_weapon() {} else {}

    original!()(module_accessor, param_type, param_hash)
}

#[skyline::hook(offset=INT_OFFSET)]
pub unsafe fn get_param_int_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
	let mut boma = *((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
	let boma_reference = &mut *boma;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if boma_reference.is_fighter() {} else if boma_reference.is_weapon() {} else {}

    original!()(module_accessor, param_type, param_hash)
}

pub fn install() {
	unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
    }
	skyline::install_hooks!(get_param_float_replace);
	skyline::install_hooks!(get_param_int_replace);
}