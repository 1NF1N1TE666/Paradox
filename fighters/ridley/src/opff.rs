utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe extern "C" fn init(fighter: &mut L2CFighterCommon) {
    VarModule::off_flag(fighter.object(), vars::ridley::instance::SPECIAL_LW_IS_SKEWER);
}

unsafe fn laser_land_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT,
    ]) {
        fighter.check_land_cancel(None);
    }
}

unsafe fn rotate_bone(boma: &mut BattleObjectModuleAccessor, max_angle: f32, min_angle: f32, strength: f32) {
    let mut angle = min_angle.abs();
    if strength > 0.0 {
        angle = max_angle
    }
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: -((angle * -1.0 * strength) - 2.5)};
    let fighter = utils::util::get_fighter_common_from_accessor(boma);
    fighter.set_joint_rotate("tail1", rotation);
}

unsafe fn tail_lean(boma: &mut BattleObjectModuleAccessor, lean_frame: f32, return_frame: f32, max_angle: f32, min_angle: f32) {
    let stick_y = ControlModule::get_stick_y(boma);
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let tail_y = VarModule::get_float(boma.object(), vars::ridley::status::SPECIAL_LW_STICK_Y);
    if frame >= 0.0 && frame < lean_frame {
        VarModule::set_float(boma.object(), vars::ridley::status::SPECIAL_LW_STICK_Y, stick_y);
        rotate_bone(boma, max_angle, min_angle, stick_y * ((frame as f32) / 30.0));
    } else if frame >= lean_frame && frame < return_frame {
        rotate_bone(boma, max_angle, min_angle, tail_y);
    } else {
        rotate_bone(boma, max_angle, min_angle, tail_y * (1.0 - ((frame - return_frame) / (end_frame - return_frame))));
    }
}

unsafe fn angled_skewer(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && fighter.is_situation(*SITUATION_KIND_GROUND) {
        tail_lean(fighter.boma(), 31.0, 41.0, 45.0, -45.0);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_CHARGE,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL_JUMP,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_FINISH,
        statuses::ridley::SPECIAL_LW_POGO
    ]) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon) {
    laser_land_cancel(fighter);
    angled_skewer(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn ridley_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        ridley_frame(fighter);
    }
}

pub unsafe fn ridley_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_start(init);
    agent.on_line(Main, ridley_frame_wrapper);
}