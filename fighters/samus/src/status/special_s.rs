use super::*;

unsafe extern "C" fn special_s_color(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::ICE_MODE)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_MATERIAL_MOTION) {
        MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_SAMUS_MOTION_PART_SET_KIND_VISOR, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_MATERIAL_MOTION);
    } else if VarModule::is_flag(fighter.battle_object, vars::samus::instance::ICE_MODE)
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_MATERIAL_MOTION) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_MATERIAL_MOTION);
        suit_effect(fighter.module_accessor, fighter.battle_object);
    } else {}

    0.into()
}

unsafe extern "C" fn special_s1g_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_s1a_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_s2g_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_s2a_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_s1g_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_color(fighter)
}

unsafe extern "C" fn special_s1a_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_color(fighter)
}

unsafe extern "C" fn special_s2g_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_color(fighter)
}

unsafe extern "C" fn special_s2a_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_color(fighter)
}



pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, special_s1g_exec);
    agent.status(Exec, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, special_s1a_exec);
    agent.status(Exec, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, special_s2g_exec);
    agent.status(Exec, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, special_s2a_exec);
    agent.status(End, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, special_s1g_end);
    agent.status(End, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, special_s1a_end);
    agent.status(End, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, special_s2g_end);
    agent.status(End, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, special_s2a_end);
}