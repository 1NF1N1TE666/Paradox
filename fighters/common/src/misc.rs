use smash::app::lua_bind::*;
use smash::app::*;
use smash::phx::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use utils::consts::*;
use utils::ext::*;
use utils::*;
use smashline::*;
use smash_script::macros::{EFFECT_FOLLOW, EFFECT_FOLLOW_FLIP, LAST_EFFECT_SET_COLOR};
use globals::*;

pub fn install() {
    smashline::Agent::new("fighter")
        .on_start(init)
        .on_line(Main, opff)
        .install();
}

pub unsafe extern "C" fn init(fighter: &mut L2CFighterCommon) {
    VarModule::off_flag(fighter.object(), vars::common::instance::BURST_LIMIT);
}

pub unsafe extern "C" fn opff(fighter: &mut L2CFighterCommon) {
    fighter.check_paradox_funcs();
    fighter.check_airdash();

    if !smashball::is_training_mode() {
        if fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_REBIRTH,
            *FIGHTER_STATUS_KIND_STANDBY,
            *FIGHTER_STATUS_KIND_WIN,
            *FIGHTER_STATUS_KIND_LOSE,
            *FIGHTER_STATUS_KIND_ENTRY,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_GUARD_DAMAGE
        ]) || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        || lua_bind::FighterManager::is_result_mode(utils::singletons::FighterManager()) 
        || !sv_information::is_ready_go() {
            VarModule::set_int(fighter.object(), vars::common::instance::STALL_TIMER, 0);
        } else {
            let articles = [
                *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT,
                *FIGHTER_SAMUS_GENERATE_ARTICLE_MISSILE,
                *FIGHTER_SAMUS_GENERATE_ARTICLE_SUPERMISSILE,
                *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB,
                *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET,
                *FIGHTER_FOX_GENERATE_ARTICLE_ILLUSION,
                *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH,
                *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE,
                *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET,
                *FIGHTER_FALCO_GENERATE_ARTICLE_ILLUSION,
                *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL,
                *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG,
                *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER_BULLET,
                *FIGHTER_WOLF_GENERATE_ARTICLE_ILLUSION,
                *FIGHTER_RIDLEY_GENERATE_ARTICLE_BREATH
            ];

            let mut hit = false;

            for i in articles {
                if ArticleModule::is_exist(fighter.module_accessor, i) {
                    let article = ArticleModule::get_article(fighter.module_accessor, i);
                    let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
                    let article_boma = sv_battle_object::module_accessor(article_id);
                    if AttackModule::is_infliction_status(article_boma, *COLLISION_KIND_MASK_ALL) {
                        VarModule::set_int(fighter.object(), vars::common::instance::STALL_TIMER, 0);
                        hit = true;
                    }
                }
            }

            if !hit {
                VarModule::add_int(fighter.object(), vars::common::instance::STALL_TIMER, 1);
            }
        }

        if VarModule::get_int(fighter.object(), vars::common::instance::STALL_TIMER) >= 1200 {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_DEAD, false);
        }

        if (KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL).abs() >= 4.0 && fighter.is_status(*FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR))
        || (KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL).abs() >= 4.0 && fighter.is_status(*FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U)) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_DEAD, false);
        }
    }

    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
        if fighter.get_num_used_jumps() == fighter.get_jump_count_max() {
            fighter.dec_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
        fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
    }
}