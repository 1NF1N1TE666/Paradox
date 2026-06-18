use super::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let damage = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 18.0 } else { 12.0 };
    let coll_attr = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { Hash40::new("collision_attr_aura") } else { Hash40::new("collision_attr_cutup") };
    FT_MOTION_RATE(agent, 1.2);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword"), damage, 80, 100, 0, 0, 4.0, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword"), damage, 80, 100, 0, 0, 4.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword"), damage, 80, 100, 0, 0, 4.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            DamageModule::add_damage(boma, 6.0, 0);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackairn(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        } else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let damage = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 36.0 } else { 24.0 };
    let coll_attr = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { Hash40::new("collision_attr_aura") } else { Hash40::new("collision_attr_cutup") };
    FT_MOTION_RATE(agent, 2.2);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword"), damage, 361, 100, 0, 0, 6.0, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword"), damage, 361, 100, 0, 0, 6.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword"), damage, 361, 100, 0, 0, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            DamageModule::add_damage(boma, 12.0, 0);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackairf(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        } else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
         AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let damage = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 12.0 } else { 8.0 };
    let coll_attr = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { Hash40::new("collision_attr_aura") } else { Hash40::new("collision_attr_cutup") };
    FT_MOTION_RATE(agent, 1.15);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword"), damage, 60, 100, 0, 0, 8.0, 0.0, 8.0, -8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword"), damage, 60, 100, 0, 0, 6.0, 0.0, 0.0, -6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            DamageModule::add_damage(boma, 4.0, 0);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        } else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let damage = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 24.0 } else { 16.0 };
    let coll_attr = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { Hash40::new("collision_attr_aura") } else { Hash40::new("collision_attr_cutup") };
    FT_MOTION_RATE(agent, 1.25);
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword"), damage, 80, 100, 0, 0, 6.0, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword"), damage, 80, 100, 0, 0, 6.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword"), damage, 80, 100, 0, 0, 6.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword"), damage, 80, 100, 0, 0, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            DamageModule::add_damage(boma, 8.0, 0);
        }
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        } else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let damage1 = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 24.0 } else { 16.0 };
    let damage2 = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 18.0 } else { 12.0 };
    let coll_attr = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { Hash40::new("collision_attr_aura") } else { Hash40::new("collision_attr_cutup") };
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), damage1, 270, 100, 0, 0, 6.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), damage1, 270, 100, 0, 0, 6.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), damage1, 270, 100, 0, 0, 6.0, 0.0, -6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            DamageModule::add_damage(boma, 8.0, 0);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword"), damage2, 80, 100, 0, 0, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword"), damage2, 80, 100, 0, 0, 6.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword"), damage2, 80, 100, 0, 0, 4.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword"), damage2, 80, 100, 0, 0, 2.0, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.1);
        } else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.1);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("ike_attack_flash"), Hash40::new("sword"), 0, 12, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attackairn, Priority::Low);
    agent.acmd("effect_attackairn", effect_attackairn, Priority::Low);
    agent.acmd("game_attackairf", game_attackairf, Priority::Low);
    agent.acmd("effect_attackairf", effect_attackairf, Priority::Low);
    agent.acmd("game_attackairb", game_attackairb, Priority::Low);
    agent.acmd("effect_attackairb", effect_attackairb, Priority::Low);
    agent.acmd("game_attackairhi", game_attackairhi, Priority::Low);
    agent.acmd("effect_attackairhi", effect_attackairhi, Priority::Low);
    agent.acmd("game_attackairlw", game_attackairlw, Priority::Low);
    agent.acmd("effect_attackairlw", effect_attackairlw, Priority::Low);
}