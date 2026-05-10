use super::*;
use globals::*;

mod special_n;
mod special_s;
mod special_hi_hover;
mod special_hi_charge;
mod special_lw;

unsafe extern "C" fn guard_on_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x_tilt = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_x"));
    let stick_y_tilt = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_y"));
    if fighter.is_button_on(Buttons::Guard)
    && fighter.is_button_on(Buttons::Special)
    && fighter.stick_x().abs() < stick_x_tilt 
    && fighter.stick_y().abs() < stick_y_tilt {
        VarModule::on_flag(fighter.object(), vars::ridley::instance::SPECIAL_LW_IS_SKEWER);
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        return true.into()
    }
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_GUARD_ON)(fighter)
}
unsafe extern "C" fn guard_on_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x_tilt = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_x"));
    let stick_y_tilt = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_y"));
    if fighter.is_button_on(Buttons::Guard)
    && fighter.is_button_on(Buttons::Special)
    && fighter.stick_x().abs() < stick_x_tilt 
    && fighter.stick_y().abs() < stick_y_tilt {
        VarModule::on_flag(fighter.object(), vars::ridley::instance::SPECIAL_LW_IS_SKEWER);
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        0.into()
    }else {
        smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_GUARD_ON)(fighter)
    }
}
unsafe extern "C" fn guard_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x_tilt = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_x"));
    let stick_y_tilt = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_y"));
    if fighter.is_button_on(Buttons::Guard)
    && fighter.is_button_on(Buttons::Special)
    && fighter.stick_x().abs() < stick_x_tilt 
    && fighter.stick_y().abs() < stick_y_tilt {
        VarModule::on_flag(fighter.object(), vars::ridley::instance::SPECIAL_LW_IS_SKEWER);
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        return true.into()
    }
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_GUARD)(fighter)
}
unsafe extern "C" fn guard_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x_tilt = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_x"));
    let stick_y_tilt = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_y"));
    if fighter.is_button_on(Buttons::Guard)
    && fighter.is_button_on(Buttons::Special)
    && fighter.stick_x().abs() < stick_x_tilt 
    && fighter.stick_y().abs() < stick_y_tilt {
        VarModule::on_flag(fighter.object(), vars::ridley::instance::SPECIAL_LW_IS_SKEWER);
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        0.into()
    } else {
        smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_GUARD)(fighter)
    }
}

unsafe extern "C" fn escape_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x_tilt = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_x"));
    let stick_y_tilt = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_y"));
    if fighter.is_button_on(Buttons::Guard)
    && fighter.is_button_on(Buttons::Special) 
    && fighter.stick_x().abs() < stick_x_tilt 
    && fighter.stick_y().abs() < stick_y_tilt {
        VarModule::on_flag(fighter.object(), vars::ridley::instance::SPECIAL_LW_IS_SKEWER);
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        return true.into()
    }
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ESCAPE_AIR)(fighter)
}
unsafe extern "C" fn escape_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x_tilt = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_x"));
    let stick_y_tilt = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_y"));
    if !fighter.is_flag(*FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) 
    && fighter.status_frame() <= 2
    && fighter.is_button_on(Buttons::Guard)
    && fighter.is_button_on(Buttons::Special)
    && fighter.stick_x().abs() < stick_x_tilt 
    && fighter.stick_y().abs() < stick_y_tilt {
        fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
        VarModule::on_flag(fighter.object(), vars::ridley::instance::SPECIAL_LW_IS_SKEWER);
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    special_n::install(agent);
    special_s::install(agent);
    special_hi_hover::install(agent);
    special_hi_charge::install(agent);
    special_lw::install(agent);
    agent.status(Main, *FIGHTER_STATUS_KIND_GUARD_ON, guard_on_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_GUARD_ON, guard_on_exec);
    agent.status(Main, *FIGHTER_STATUS_KIND_GUARD, guard_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_GUARD, guard_exec);
    agent.status(Main, *FIGHTER_STATUS_KIND_ESCAPE_AIR, escape_air_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_ESCAPE_AIR, escape_air_exec);
}