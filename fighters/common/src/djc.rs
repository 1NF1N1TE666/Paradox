use super::*;

use globals::*;

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_init_impl,
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_impl
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

/// Inherits the double jump animation movement when doing an aerial (init)
#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_inherit_jump_aerial_motion_uniq_process_init)]
pub unsafe extern "C" fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_init_impl(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_DEMON {
        call_original!(fighter)
    } else {
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);

        fighter.sub_attack_air_kind();
        if motion_kind == smash::hash40("jump_aerial_f") || motion_kind == smash::hash40("jump_aerial_b") {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
                MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
                MotionModule::set_weight(fighter.module_accessor, 1.0, true);
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) || frame < 2.0 {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
                } else {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
                }
            } else {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            }
        } else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        }
        let _ = fighter.sub_attack_air_uniq_process_init();
        0.into()
    }
}

/// Inherits the double jump animation movement when doing an aerial (exec)
#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec)]
pub unsafe extern "C" fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_impl(fighter: &mut L2CFighterCommon) -> L2CValue {
    if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND 
    && fighter.global_table[FIGHTER_KIND] != FIGHTER_KIND_DEMON
    && MotionModule::frame_2nd(fighter.module_accessor) >= 2.0
    && fighter.global_table[CURRENT_FRAME].get_i32() <= 6
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    }
    call_original!(fighter)
}