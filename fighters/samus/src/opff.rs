utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;
use crate::status::speedbooster_shinespark::*;

unsafe fn special_n_throw_lw(fighter: &mut L2CFighterCommon) {
    if !fighter.is_motion(Hash40::new("throw_lw"))
    && VarModule::get_float(fighter.object(), vars::samus::instance::SPECIAL_N_THROW_LW_CHARGE_STORAGE) != -1.0 {
        VarModule::set_float(fighter.object(), vars::samus::instance::SPECIAL_N_THROW_LW_CHARGE_STORAGE, -1.0);
    }
}

unsafe fn special_s_force_form(boma: *mut BattleObjectModuleAccessor, battle_object: *mut BattleObject, status_kind: i32, situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if VarModule::is_flag(battle_object, vars::samus::instance::ICE_MODE){
        if (&[
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G,
        ]).contains(&status_kind) {
            StatusModule::change_status_request_from_script(boma, if situation_kind == *SITUATION_KIND_AIR {*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A} else {*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G}, false);
        }
    } else {
        if (&[
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G,
        ]).contains(&status_kind) {
            StatusModule::change_status_request_from_script(boma, if situation_kind == *SITUATION_KIND_AIR {*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A} else {*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G},  false);
        }
    }
}

unsafe fn aim(fighter: &mut L2CFighterCommon) {
    let frame = MotionModule::frame(fighter.module_accessor);
    let flip = if fighter.is_status(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A) {-1.0} else {1.0};
    let mut clavicler_rot = Vector3f::zero();
    let mut claviclel_rot = Vector3f::zero();
    let mut neck_rot = Vector3f::zero();
    let mut bust_rot = Vector3f::zero();

    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H,
        *FIGHTER_STATUS_KIND_SPECIAL_S
    ]) || (fighter.is_status(
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F
    ) && frame < 3.0) || (fighter.is_status_one_of(&[
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A,
    ]) && frame < 18.0) || (fighter.is_status_one_of(&[
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A
    ]) && frame < 21.0) {
        let prev_angle = VarModule::get_float(fighter.object(), vars::samus::instance::AIM_ANGLE);
        let angle;
        if fighter.stick_x().abs() + fighter.stick_y().abs() == 0.0 {
            angle = 0.0;
        } else {
            let mut stick_deg = ControlModule::get_stick_angle(fighter.module_accessor).to_degrees();
            if fighter.lr() < 0.0 {
                if stick_deg < 0.0 {
                    stick_deg = -180.0 - stick_deg;
                } else {
                    stick_deg = 180.0 - stick_deg;
                }
            }
            angle = stick_deg.clamp(-90.0, 90.0);
        }
        angle.clamp(prev_angle - 5.0, prev_angle + 5.0);
        VarModule::set_float(fighter.object(), vars::samus::instance::AIM_ANGLE, angle);

        let neck_offset = angle.clamp(-30.0, 30.0);
        neck_rot.z -= neck_offset;
        fighter.set_joint_rotate("neck", neck_rot);

        let clavicler_offset = angle.clamp(-90.0, 90.0) * flip;
        if fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F
        ]) {
            clavicler_rot.x += clavicler_offset;
            fighter.set_joint_rotate("clavicler", clavicler_rot);
            let claviclel_offset = angle.clamp(-30.0, 30.0);
            claviclel_rot.x -= claviclel_offset;
            fighter.set_joint_rotate("claviclel", claviclel_rot);
        } else {
            clavicler_rot.z -= clavicler_offset;
            fighter.set_joint_rotate("clavicler", clavicler_rot);
        }
        if angle.abs() > 45.0 && fighter.is_situation(*SITUATION_KIND_AIR) {
            let bust_offset = angle - (45.0 * angle.signum());
            if fighter.is_status_one_of(&[
                *FIGHTER_STATUS_KIND_SPECIAL_N,
                *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H,
                *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F
            ]) {
                bust_rot.z -= bust_offset;
                fighter.set_joint_rotate("waist", bust_rot);
            } else {
                bust_rot.y -= bust_offset;
                fighter.set_joint_rotate("bust", bust_rot);
            }
        }
    } else {
        let prev_angle = VarModule::get_float(fighter.object(), vars::samus::instance::AIM_ANGLE);
        let angle = 0.0_f32.clamp(prev_angle - 5.0, prev_angle + 5.0);
        VarModule::set_float(fighter.object(), vars::samus::instance::AIM_ANGLE, angle);

        if angle != 0.0_f32 {
            let neck_offset = angle.clamp(-30.0, 30.0);
            neck_rot.z -= neck_offset;
            fighter.set_joint_rotate("neck", neck_rot);

            if angle.abs() > 45.0 && fighter.is_situation(*SITUATION_KIND_AIR) {
                let bust_offset = angle - (45.0 * angle.signum());
                if fighter.is_status_one_of(&[
                    *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F,
                    *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E,
                    *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C
                ]) {
                    bust_rot.z -= bust_offset;
                    fighter.set_joint_rotate("waist", bust_rot);
                } else {
                    bust_rot.y -= bust_offset;
                    fighter.set_joint_rotate("bust", bust_rot);
                }
            }

            let clavicler_offset = angle.clamp(-90.0, 90.0) * flip;
            if fighter.is_status_one_of(&[
                *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F,
                *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E,
                *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C
            ]) {
                clavicler_rot.x += clavicler_offset;
                fighter.set_joint_rotate("clavicler", clavicler_rot);

                let claviclel_offset = angle.clamp(-30.0, 30.0);
                claviclel_rot.x -= claviclel_offset;
                fighter.set_joint_rotate("claviclel", claviclel_rot);
            } else {
                clavicler_rot.z -= clavicler_offset;
                fighter.set_joint_rotate("clavicler", clavicler_rot);
            }
        }
    }
}

unsafe fn ice(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor) {
    if MotionModule::frame(fighter.module_accessor) % 15.0 < 1.0 
    || MotionModule::frame(fighter.module_accessor) <= 1.0 {
        suit_effect(boma, fighter.battle_object);
    }
}

unsafe fn dread(fighter: &mut L2CFighterCommon) {
    let lr = PostureModule::lr(fighter.module_accessor);
    let speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL).abs();
    if VarModule::is_flag(fighter.object(), vars::samus::instance::SPECIAL_HI_HOP_DISABLE) {
        if !fighter.is_situation(*SITUATION_KIND_AIR) {
            VarModule::off_flag(fighter.object(), vars::samus::instance::SPECIAL_HI_HOP_DISABLE);
        }
    }
    if VarModule::is_flag(fighter.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
        speedbooster_effect(fighter);
        JostleModule::set_status(fighter.module_accessor, false);
        if fighter.is_status(*FIGHTER_STATUS_KIND_DASH)
        && WorkModule::get_param_float(fighter.module_accessor, hash40("dash_speed"), 0) > speed {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 3.0 - speed, y: 0.0, z: 0.0});
        }
        if fighter.is_status(*FIGHTER_STATUS_KIND_RUN)
        && WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0) > speed {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 3.0 - speed, y: 0.0, z: 0.0});
        }
        if fighter.is_button_trigger(Buttons::AppealAll) 
        && fighter.is_status(*FIGHTER_STATUS_KIND_RUN) 
        && !StatusModule::is_changing(fighter.module_accessor) {
            speedbooster_off(fighter);
            StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, true);
        }
        if fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_STATUS_KIND_RUN,
            *FIGHTER_STATUS_KIND_JUMP_SQUAT,
            *FIGHTER_STATUS_KIND_JUMP, 
            *FIGHTER_STATUS_KIND_JUMP_AERIAL,
            *FIGHTER_STATUS_KIND_WALL_JUMP,
            *FIGHTER_STATUS_KIND_FALL, 
            *FIGHTER_STATUS_KIND_FALL_AERIAL,
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_ATTACK_DASH,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW,
            *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP
        ]) {
            if fighter.stick_x() * fighter.lr() >= 0.0 {
                VarModule::set_int(fighter.object(), vars::samus::instance::SPEEDBOOSTER_STICK_TIMER, 20);
            } else {
                if VarModule::get_int(fighter.object(), vars::samus::instance::SPEEDBOOSTER_STICK_TIMER) > 0 {
                    VarModule::dec_int(fighter.object(), vars::samus::instance::SPEEDBOOSTER_STICK_TIMER);
                } else {
                    speedbooster_off(fighter);
                }
            }
        } else {
            speedbooster_off(fighter);
        }
    } else {
        if fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_STATUS_KIND_RUN
        ]) && !VarModule::is_flag(fighter.object(), vars::samus::instance::SHINESPARK_ON) {
            speedbooster_on(fighter);
        }
    }
    if VarModule::is_flag(fighter.object(), vars::samus::instance::SHINESPARK_ON) {
        if fighter.is_status(*FIGHTER_STATUS_KIND_DEAD) {
            shinespark_off(fighter);
        } else {
            shinespark_effect(fighter);
            if fighter.is_cat_flag(Cat1::JumpButton) && fighter.stick_x().abs() < 0.4 {
                if (fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) >= fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) 
                && (CancelModule::is_enable_cancel(fighter.module_accessor) 
                || fighter.is_status_one_of(&[
                    *FIGHTER_STATUS_KIND_JUMP,
                    *FIGHTER_STATUS_KIND_JUMP_AERIAL,
                    *FIGHTER_STATUS_KIND_FALL,
                    *FIGHTER_STATUS_KIND_FALL_AERIAL
                ]))) || fighter.is_status(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW) {
                    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, true);
                    shinespark_off(fighter);
                }
            }
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW
    ]) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let frame = MotionModule::frame(boma);
    special_n_throw_lw(fighter);
    special_s_force_form(boma, fighter.battle_object, status_kind, situation_kind, frame);
    aim(fighter);
    ice(fighter, boma);
    dread(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn samus_frame_wrapper(fighter: &mut L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        samus_frame(fighter)
    }
}

pub unsafe fn samus_frame(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, samus_frame_wrapper);
}