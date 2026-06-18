use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_CliffCatchMove,
            status_CliffAttack_Main,
            status_CliffClimb_Main,
            status_CliffEscape_Main,
            status_CliffJump1,
            status_end_CliffJump1,
            status_CliffJump2_Main,
            sub_cliff_uniq_process_exit_Common,
            get_cliff_wait_hit_xlu_frame
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CliffCatchMove)]
unsafe fn status_CliffCatchMove(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CLIFF_XLU) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CliffAttack_Main)]
unsafe fn status_CliffAttack_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CliffClimb_Main)]
unsafe fn status_CliffClimb_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CliffEscape_Main)]
unsafe fn status_CliffEscape_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CliffJump1)]
unsafe fn status_CliffJump1(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = call_original!(fighter);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let xlu_end_frame = FighterMotionModuleImpl::get_hit_normal_frame(fighter.module_accessor, Hash40::new_raw(motion_kind), true);
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    if xlu_end_frame > end_frame {
        let mut motion_rate = end_frame / xlu_end_frame;
        if motion_rate < 1.0 {
            motion_rate += 0.001;
        }
        MotionModule::set_rate(fighter.module_accessor, motion_rate);
        MotionModule::set_whole_rate(fighter.module_accessor, 1.0);
    }
    InputModule::set_persist_lifetime(fighter.battle_object, 10);
    InputModule::enable_persist(fighter.battle_object);

    ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffJump1)]
unsafe fn status_end_CliffJump1(fighter: &mut L2CFighterCommon) -> L2CValue {
    InputModule::disable_persist(fighter.battle_object);
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CliffJump2_Main)]
unsafe fn status_CliffJump2_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LASSO);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
    }

    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }

    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_cliff_uniq_process_exit_Common)]
unsafe fn sub_cliff_uniq_process_exit_Common(fighter: &mut L2CFighterCommon, is_leave_cliff: L2CValue) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF) {
        if ![*FIGHTER_STATUS_KIND_CLIFF_ATTACK,
            *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
            *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP1
        ].contains(&fighter.global_table[STATUS_KIND_INTERRUPT].get_i32()) {
            let cliff_no_catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_no_catch_frame"));
            WorkModule::set_int(fighter.module_accessor, cliff_no_catch_frame, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME);
        }
        // Allows lingering ledge intan on ledgedrop
        if fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_FALL {
            HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
            VarModule::set_int(fighter.battle_object, vars::common::instance::CLIFF_XLU_FRAME, 0);
        }
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF);
    if is_leave_cliff.get_bool() {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF);
        GroundModule::leave_cliff(fighter.module_accessor);
        // Allows lingering ledge intan on ledgedrop
        if fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_FALL {
            HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
            VarModule::set_int(fighter.battle_object, vars::common::instance::CLIFF_XLU_FRAME, 0);
        }
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_get_cliff_wait_hit_xlu_frame)]
unsafe fn get_cliff_wait_hit_xlu_frame(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cliff_xlu_frame = call_original!(fighter).get_i32();
    VarModule::set_int(fighter.battle_object, vars::common::instance::CLIFF_XLU_FRAME, cliff_xlu_frame);
    call_original!(fighter)
}