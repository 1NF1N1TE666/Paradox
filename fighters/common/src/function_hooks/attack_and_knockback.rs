use super::*;
use globals::*;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;
use utils::ext::*;

static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x67a7b0;

#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, damage: f32, attack_kind: i32, damage_meteor: bool) -> u64 {
    let attacker_boma = &mut *sv_battle_object::module_accessor(attacker_id);
    let defender_boma = &mut *sv_battle_object::module_accessor(defender_id);
    
    original!()(fighter_manager, attacker_id, defender_id, damage, attack_kind, damage_meteor)
}

/*
    attacker_id:    object_id of attacker
    defender_id:    object_id of defender
    damage:         damage dealt by the hit
    attack_kind:    FIGHTER_LOG_ATTACK_KIND_... constant, for the "kind" of move hit
    damage_meteor:  if the hit spiked. related to FIGHTER_LOG_DATA_FLAG_ON_DAMAGE_METEOR
*/

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static OFFSET_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1, //.text:0000007100675A20                 SUB             SP, SP, #0xC0
    0xe8, 0x2b, 0x00, 0xfd, //.text:0000007100675A24                 STR             D8, [SP,#0xB0+var_60]
    0xfc, 0x6f, 0x06, 0xa9, //.text:0000007100675A28                 STP             X28, X27, [SP,#0xB0+var_50]
    0xfa, 0x67, 0x07, 0xa9, //.text:0000007100675A2C                 STP             X26, X25, [SP,#0xB0+var_40]
    0xf8, 0x5f, 0x08, 0xa9, //.text:0000007100675A30                 STP             X24, X23, [SP,#0xB0+var_30]
    0xf6, 0x57, 0x09, 0xa9, //.text:0000007100675A34                 STP             X22, X21, [SP,#0xB0+var_20]
    0xf4, 0x4f, 0x0a, 0xa9, //.text:0000007100675A38                 STP             X20, X19, [SP,#0xB0+var_10]
    0xfd, 0x7b, 0x0b, 0xa9, //.text:0000007100675A3C                 STP             X29, X30, [SP,#0xB0+var_s0]
    0xfd, 0xc3, 0x02, 0x91, //.text:0000007100675A40                 ADD             X29, SP, #0xB0
    0xfb, 0x03, 0x00, 0xaa  //.text:0000007100675A44                 MOV             X27, X0
];

#[skyline::hook(offset = 0x46b648, inline)]
unsafe fn get_hitstop_mul(ctx: &mut skyline::hooks::InlineCtx) {
    if ctx.registers[1].w() == 0x2 {
        let hitstop_mul: f32 = 1.0;
        ctx.registers_f[0].set_s(hitstop_mul)
    }
}

#[skyline::hook(offset = 0x3dd688, inline)]
unsafe extern "C" fn attack_module_set_power_hook_pattern(ctx: &mut skyline::hooks::InlineCtx) {
    ctx.registers_f[2].set_s(1.0);
}

#[skyline::hook(offset = 0x6c5980, inline)]
unsafe fn set_damage_lr(ctx: &mut skyline::hooks::InlineCtx) {
    let opponent_battle_object_id = *(ctx.registers[20].x() as *const u32).add(0x44 / 4);
    let opponent_battle_object = utils::util::get_battle_object_from_id(opponent_battle_object_id);
    let opponent_boma = (&mut *(*opponent_battle_object).module_accessor);

    if !opponent_boma.is_fighter() {
        return;
    }
    
    let opponent_pos_x = PostureModule::pos_x(opponent_boma);
    let boma = ctx.registers[19].x() as *mut smash::app::BattleObjectModuleAccessor;
    let pos_x = PostureModule::pos_x(boma);
    let lr = PostureModule::lr(boma);
    let attack_lr_check = VarModule::get_int((*opponent_boma).object(), vars::common::instance::ATTACK_LR_CHECK);
    let default_lr = if opponent_pos_x >= pos_x { 1.0 } else { -1.0 };
    let damage_lr: f32 = {
        if attack_lr_check != *ATTACK_LR_CHECK_F && attack_lr_check != *ATTACK_LR_CHECK_B {
            default_lr
        } else {
            let ecb_front = if opponent_pos_x >= pos_x {
                let ecb_right = *GroundModule::get_rhombus(boma, true).add(3);
                ecb_right.x
            } else {
                let ecb_left = *GroundModule::get_rhombus(boma, true).add(2);
                ecb_left.x
            };

            let attacker_lr = PostureModule::lr(opponent_boma);

            let is_behind_attacker = if opponent_pos_x >= pos_x {
                (attack_lr_check == *ATTACK_LR_CHECK_F && attacker_lr > 0.0)
                    || (attack_lr_check == *ATTACK_LR_CHECK_B && attacker_lr < 0.0)
            } else {
                (attack_lr_check == *ATTACK_LR_CHECK_F && attacker_lr < 0.0) || (attack_lr_check == *ATTACK_LR_CHECK_B && attacker_lr > 0.0)
            };

            let ecb_crosses_attacker = if opponent_pos_x >= pos_x {
                ecb_front >= opponent_pos_x
            } else {
                ecb_front <= opponent_pos_x
            };

            if is_behind_attacker && ecb_crosses_attacker {
                -default_lr
            } else {
                default_lr
            }
        }
    };

    ctx.registers_f[0].set_s(damage_lr)
}

#[skyline::hook(offset = 0x3ff1b8, inline)]
unsafe fn get_attack_lr_check(ctx: &mut skyline::hooks::InlineCtx) {
    let attack_module = ctx.registers[1].x();
    let boma = *(attack_module as *mut *mut BattleObjectModuleAccessor).add(1);

    if !(*boma).is_fighter() {
        return;
    }

    let attack_lr_check = ctx.registers[8].w() as i32;
    VarModule::set_int((*boma).object(), vars::common::instance::ATTACK_LR_CHECK, attack_lr_check);
}

pub fn install() {
    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }
    skyline::install_hooks!(
        notify_log_event_collision_hit_replace,
        get_hitstop_mul,
        attack_module_set_power_hook_pattern,
        set_damage_lr,
        get_attack_lr_check
    );
}