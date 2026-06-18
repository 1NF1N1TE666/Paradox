utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn laser_landcancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        boma.check_land_cancel(None);
    }
}

unsafe fn shine_jump_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END
    ]) {
        fighter.check_jump_cancel(true);
    }
}

unsafe fn firefox_startup_ledgegrab(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        fighter.sub_transition_group_check_air_cliff();
    }
}

unsafe fn aim_throw_lasers(boma: &mut BattleObjectModuleAccessor) {
    let frame = boma.motion_frame();
    let lr = PostureModule::lr(boma);

    if boma.is_motion(Hash40::new("throw_hi"))
    && 7.0 <= frame
    && frame < 18.0 {
        let rot = Vector3f::new(0.0, boma.stick_x() * lr * -20.0, 0.0);
        boma.set_joint_rotate("clavicler", rot);
    } else if boma.is_motion(Hash40::new("throw_b"))
    && 10.0 <= frame
    && frame < 16.0 {
        let rot = Vector3f::new(0.0, boma.stick_y() * -20.0, 0.0);
        boma.set_joint_rotate("shoulderr", rot);
    } else {}
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_AIR) 
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT
    ]) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    laser_landcancel(boma, status_kind, situation_kind, cat[1], stick_y);
    shine_jump_cancel(fighter);
    firefox_startup_ledgegrab(fighter);
    aim_throw_lasers(boma);
    fastfall_specials(fighter);
}

pub extern "C" fn fox_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		fox_frame(fighter)
    }
}

pub unsafe fn fox_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, fox_frame_wrapper);
}