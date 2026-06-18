use super::*;
use globals::*;

// This file contains code related to knockdown states

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_DamageFlyChkUniq,
            status_pre_Down,
            status_DownStand_Main,
            status_DownStandFb_Main,
            status_DownStandAttack_Main
        );
    }
}

// This runs as you enter knockdown
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_Down)]
unsafe fn status_pre_Down(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false, // false = can be grabbed
        false,
        0,
        *FIGHTER_STATUS_ATTR_SLOPE_TOP_UNLIMIT as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_DamageFlyChkUniq)]
unsafe fn sub_DamageFlyChkUniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = call_original!(fighter);
    if ret.get_bool() {
        if fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP) {
            fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP);
            fighter.set_float(0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_SPEED_UP_MAX_MAG);
        }
    }
    ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DownStand_Main)]
unsafe fn status_DownStand_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(motion_kind), true);
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    if cancel_frame > end_frame {
        if StatusModule::is_changing(fighter.module_accessor) {
            let mut motion_rate = end_frame / cancel_frame;
            if motion_rate < 1.0 {
                motion_rate += 0.001;
            }
            MotionModule::set_rate(fighter.module_accessor, motion_rate);
            MotionModule::set_whole_rate(fighter.module_accessor, 1.0);
        }
        
        let xlu_end_frame = FighterMotionModuleImpl::get_hit_normal_frame(fighter.module_accessor, Hash40::new_raw(motion_kind), true);
        if fighter.global_table[CURRENT_FRAME].get_f32() == xlu_end_frame {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    }

    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DownStandFb_Main)]
unsafe fn status_DownStandFb_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(motion_kind), true);
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    if cancel_frame > end_frame {
        if StatusModule::is_changing(fighter.module_accessor) {
            let mut motion_rate = end_frame / cancel_frame;
            if motion_rate < 1.0 {
                motion_rate += 0.001;
            }
            MotionModule::set_rate(fighter.module_accessor, motion_rate);
            MotionModule::set_whole_rate(fighter.module_accessor, 1.0);
        }
        
        let xlu_end_frame = FighterMotionModuleImpl::get_hit_normal_frame(fighter.module_accessor, Hash40::new_raw(motion_kind), true);
        if fighter.global_table[CURRENT_FRAME].get_f32() == xlu_end_frame {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    }

    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DownStandAttack_Main)]
unsafe fn status_DownStandAttack_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(motion_kind), true);
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    if cancel_frame > end_frame {
        if StatusModule::is_changing(fighter.module_accessor) {
            let mut motion_rate = end_frame / cancel_frame;
            if motion_rate < 1.0 {
                motion_rate += 0.001;
            }
            MotionModule::set_rate(fighter.module_accessor, motion_rate);
            MotionModule::set_whole_rate(fighter.module_accessor, 1.0);
        }
        
        let xlu_end_frame = FighterMotionModuleImpl::get_hit_normal_frame(fighter.module_accessor, Hash40::new_raw(motion_kind), true);
        if fighter.global_table[CURRENT_FRAME].get_f32() == xlu_end_frame {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    }

    call_original!(fighter)
}