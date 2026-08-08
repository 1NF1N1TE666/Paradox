use super::*;
use globals::*;

unsafe extern "C" fn shoot_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = weapon.get_param_int("param_cshot", "life");
    weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if weapon.is_flag(*WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED)
    && !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        effect!(weapon, MA_MSC_EFFECT_REQUEST_FOLLOW, Hash40::new("samus_cshot_bullet"), Hash40::new("top"), 7.98004, -0.50584, -0.25092, -91.2728, -1.7974, 176.373, 1.0, false, 0, 0, 0);
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        weapon.set_int(handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET);
    }
    let lr = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_SHOOT_LR);
    let charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let angle = VarModule::get_float(weapon.get_owner_boma().object(), vars::samus::instance::AIM_ANGLE);
    let min_speed = weapon.get_param_float("param_cshot", "min_speed");
    let max_speed = weapon.get_param_float("param_cshot", "max_speed");
    let speed = (max_speed - min_speed) * charge + min_speed;
    let speed_x = angle.to_radians().cos() * speed * lr;
    let speed_y = angle.to_radians().sin() * speed;
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -1.0, -1.0);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        let min_scale = weapon.get_param_float("param_cshot", "min_scale");
        let max_scale = weapon.get_param_float("param_cshot", "max_scale");
        let scale = (max_scale - min_scale) * charge + min_scale;
        if (0.3..1.0).contains(&scale) {
            effect!(weapon, MA_MSC_EFFECT_REQUEST_FOLLOW, Hash40::new("samus_cshot_bullet_sub_a"), Hash40::new("top"), 7.98004, -0.50584, -0.25092, -91.2728, -1.7974, 176.373, scale, false, 0, 0, 0);
        } else {
            effect!(weapon, MA_MSC_EFFECT_REQUEST_FOLLOW, Hash40::new("samus_cshot_bullet_sub_b"), Hash40::new("top"), 7.98004, -0.50584, -0.25092, -91.2728, -1.7974, 176.373, scale, false, 0, 0, 0);
        }
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        weapon.set_int(handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW);
        effect!(weapon, MA_MSC_EFFECT_REQUEST_FOLLOW, Hash40::new("samus_cshot_bullet_sub"), Hash40::new("top"), 7.98004, -0.50584, -0.25092, -91.2728, -1.7974, 176.373, scale, false, 0, 0, 0);
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        weapon.set_int(handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW_SUB);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT, shoot_init);
}