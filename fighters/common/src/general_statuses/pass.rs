use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_sub_set_pass)]
unsafe extern "C" fn sub_set_pass(fighter: &mut L2CFighterCommon) {
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&0xFE.into());

    GroundModule::pass_floor(fighter.module_accessor);
    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
    let prev_situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    fighter.global_table[PREV_SITUATION_KIND].assign(&L2CValue::I32(prev_situation_kind));
    fighter.global_table[SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_AIR));
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

    let pass_speed_y = fighter.get_param_float("common", "pass_speed_y");
    KineticModule::add_speed(fighter.module_accessor, &Vector3f::new(0.0, pass_speed_y, 0.0));

    let curr_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let air_speed_x_stable = fighter.get_param_float("air_speed_x_stable", "");
    if curr_speed_x.abs() > air_speed_x_stable {
        let jump_speed_x_mul = fighter.get_param_float("jump_speed_x_mul", "").sqrt(); // normalized
        let pass_air_speed_x_max_mul = 2.0;
        let new_speed_x = (curr_speed_x.abs() * jump_speed_x_mul).clamp(air_speed_x_stable, air_speed_x_stable * pass_air_speed_x_max_mul) * curr_speed_x.signum();
        let adjust_speed_x = (new_speed_x - curr_speed_x) * PostureModule::lr(fighter.module_accessor);
        KineticModule::add_speed(fighter.module_accessor, &Vector3f::new(adjust_speed_x, 0.0, 0.0));
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_set_pass
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}