use super::*;

unsafe extern "C" fn game_hit_lb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 100, 40, 0, 14.0, 0.0, 6.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 28.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 4.0, 361, 200, 0, 40, 14.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_hitair_lb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 100, 40, 0, 14.0, 0.0, 6.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 28.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 4.0, 361, 200, 0, 40, 14.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_regular(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let owner_boma = boma.get_owner_boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        if VarModule::get_float(owner_boma.object(), vars::iceclimbers::instance::LIMIT_GAUGE) == 100.0 {
            VarModule::set_float(owner_boma.object(), vars::iceclimbers::instance::LIMIT_GAUGE, 0.0);
            KineticModule::mul_speed(boma, &Vector3f{x: 2.0, y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("regular_lb"), -1.0, 1.0, 0.0, false, false);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 100, 100, 0, 4.0, 0.0, 12.0, -2.0, Some(0.0), Some(4.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 100, 100, 0, 4.0, 0.0, 2.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
}

unsafe extern "C" fn game_regularair(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let owner_boma = boma.get_owner_boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        if VarModule::get_float(owner_boma.object(), vars::iceclimbers::instance::LIMIT_GAUGE) == 100.0 {
            VarModule::set_float(owner_boma.object(), vars::iceclimbers::instance::LIMIT_GAUGE, 0.0);
            KineticModule::mul_speed(boma, &Vector3f{x: 2.0, y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("regular_air_lb"), -1.0, 1.0, 0.0, false, false);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 100, 100, 0, 4.0, 0.0, 12.0, -2.0, Some(0.0), Some(4.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 100, 100, 0, 4.0, 0.0, 2.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
}

unsafe extern "C" fn game_regular_lb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 366, 100, 100, 0, 6.0, 0.0, 14.0, -8.0, Some(0.0), Some(4.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 366, 100, 100, 0, 6.0, 0.0, 0.0, -6.0, Some(0.0), Some(4.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn game_regularair_lb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 366, 100, 100, 0, 6.0, 0.0, 14.0, -8.0, Some(0.0), Some(4.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 366, 100, 100, 0, 6.0, 0.0, 0.0, -6.0, Some(0.0), Some(4.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_regular", game_regular, Priority::Low);
    agent.acmd("game_regularair", game_regularair, Priority::Low);
    agent.acmd("game_regular_lb", game_regular_lb, Priority::Low);
    agent.acmd("game_regularair_lb", game_regularair_lb, Priority::Low);
    agent.acmd("game_hit_lb", game_hit_lb, Priority::Low);
    agent.acmd("game_hitair_lb", game_hitair_lb, Priority::Low);
}
