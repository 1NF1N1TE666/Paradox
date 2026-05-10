use super::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 90, 100, 0, 50, 5.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 5.0, 90, 100, 0, 50, 5.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_NONE);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 5.0, 90, 100, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let damage = if VarModule::is_flag(boma.get_owner_boma().object(), vars::ike::instance::STORED_AETHER) { 7.5 } else { 5.0 };
    let coll_attr = if VarModule::is_flag(boma.get_owner_boma().object(), vars::ike::instance::STORED_AETHER) { Hash40::new("collision_attr_aura") } else { Hash40::new("collision_attr_cutup") };
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), damage, 90, 100, 100, 0, 5.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 90, 100, 100, 0, 5.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("haver"), damage, 90, 100, 50, 0, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 90, 100, 50, 0, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_IKE_SWORD_STATUS_SPECIAL_HI_WORK_ID_FLAG_HAVE);
    }
}

unsafe extern "C" fn effect_specialhi2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        if VarModule::is_flag(boma.get_owner_boma().object(), vars::ike::instance::STORED_AETHER) {
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        } else {
            EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.get_owner_boma().object(), vars::ike::instance::STORED_AETHER) { 
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_tenku"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 1, true);
        } else {
            EFFECT_FOLLOW(agent, Hash40::new("ike_tenku_sword"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.get_owner_boma().object(), vars::ike::instance::STORED_AETHER) { 
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, false);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.get_owner_boma().object(), vars::ike::instance::STORED_AETHER) { 
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, false);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.get_owner_boma().object(), vars::ike::instance::STORED_AETHER) { 
            EFFECT_OFF_KIND(agent, Hash40::new("ike_final_sword_tenku"), true, true);
        } else {
            EFFECT_OFF_KIND(agent, Hash40::new("ike_tenku_sword"), true, true);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.get_owner_boma().object(), vars::ike::instance::STORED_AETHER) { 
            EFFECT_FOLLOW(agent, Hash40::new("ike_tenku_sword2"), Hash40::new("haver"), 0, 0, 0, 0, 180, 0, 1, true);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fly", game_fly, Priority::Low);
    agent.acmd("game_specialhi2", game_specialhi2, Priority::Low);
    agent.acmd("game_specialairhi2", game_specialhi2, Priority::Low);
    agent.acmd("effect_specialhi2", effect_specialhi2, Priority::Low);
    agent.acmd("effect_specialairhi2", effect_specialhi2, Priority::Low);
}