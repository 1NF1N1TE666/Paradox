use super::*;
use globals::*;

unsafe fn scale(fighter: &mut L2CFighterCommon) {
    if ModelModule::scale(fighter.module_accessor) == WorkModule::get_param_float(fighter.module_accessor, hash40("scale"), 0) {
        ModelModule::set_scale(fighter.module_accessor, 0.9);
        AttackModule::set_attack_scale(fighter.module_accessor, 0.9, true);
        GrabModule::set_size_mul(fighter.module_accessor, 0.9);
    };
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    scale(fighter);
}

pub unsafe extern "C" fn koopa_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, koopa_frame);
}