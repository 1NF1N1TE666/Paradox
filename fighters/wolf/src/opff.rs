utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn special_s_disable(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_GROUND | *SITUATION_KIND_CLIFF) || fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL
    ]) {
        VarModule::off_flag(fighter.battle_object, vars::wolf::status::SPECIAL_S_DISABLE);
    }
}

unsafe fn shine_jump_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP,
        *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END
    ]) {
        fighter.check_jump_cancel(true);
    }
}

unsafe fn firefox_startup_ledgegrab(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        fighter.sub_transition_group_check_air_cliff();
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_AIR) 
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP,
        *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT
    ]) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    special_s_disable(fighter);
    shine_jump_cancel(fighter);
    firefox_startup_ledgegrab(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn wolf_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        wolf_frame(fighter)
    }
}

pub unsafe fn wolf_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, wolf_frame_wrapper);
}