use super::*;

unsafe extern "C" fn special_hi_charge_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_charge_hi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let decide_stick_x = VarModule::get_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_HOVER_DECIDE_STICK_X);
    let decide_stick_y = VarModule::get_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_HOVER_DECIDE_STICK_Y);
    let mut rad = 0.0_f32.to_radians();
    let charge_speed_hi = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_speed_hi"));
    let lr = PostureModule::lr(fighter.module_accessor);

    let stick_added = decide_stick_x.abs() + decide_stick_y.abs();

    if stick_added >= 0.5 {
        let atan = (decide_stick_x * lr).atan2(decide_stick_y);
        rad = atan;
    }

    let rush_speed_x = charge_speed_hi * rad.sin() * lr;
    let rush_speed_y = charge_speed_hi * rad.cos();

    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, rush_speed_x, rush_speed_y);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    fighter.set_joint_rotate("rot", Vector3f{x: rad.to_degrees(), y: 0.0, z: 0.0});

    VarModule::set_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_CHARGE_DIR, rad);

    0.into()
}

unsafe extern "C" fn special_hi_charge_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_CHARGE_DECCEL);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_COUNT);

    let charge_degree_lw = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_degree_lw"));
    let dir = VarModule::get_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_CHARGE_DIR);
    let degrees = dir.to_degrees();
        
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_STATUS);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_hi_charge_hi"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    let charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_frame_hi")) as f32;
    let rate = MotionModule::end_frame(fighter.module_accessor) / charge_frame;
    MotionModule::set_rate(fighter.module_accessor, rate);
    
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
    fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());

    fighter.main_shift(special_hi_charge_hi_main_loop)
}

unsafe extern "C" fn special_hi_charge_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return 0.into();
    }

    0.into()
}

unsafe extern "C" fn special_hi_charge_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_COUNT);

    let dir = VarModule::get_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_CHARGE_DIR);
    fighter.set_joint_rotate("rot", Vector3f{x: dir.to_degrees(), y: 0.0, z: 0.0});

    VarModule::set_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_CHARGE_DIR, dir);

    let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_COUNT);
    let passable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("passable_frame"));
    let passable = frame <= passable_frame;
    GroundModule::set_passable_check(fighter.module_accessor, passable);

    let deccel_start_frame_hi = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("deccel_start_frame_hi"));

    if frame <= deccel_start_frame_hi {
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        return 0.into();
    }

    let charge_deccel_hi = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_deccel_hi"));
    let mut deccel_x = (dir.sin() * charge_deccel_hi).abs();
    let mut deccel_y = (dir.cos() * charge_deccel_hi).abs();

    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, deccel_x, deccel_y);

    0.into()
}

unsafe extern "C" fn special_hi_charge_hi_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dir = VarModule::get_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_CHARGE_DIR);
    fighter.set_joint_rotate("rot", Vector3f{x: dir.to_degrees(), y: 0.0, z: 0.0});

    0.into()
}

unsafe extern "C" fn special_hi_charge_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] != FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END {
        fighter.set_joint_rotate("rot", Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }

    0.into()
}

unsafe extern "C" fn special_hi_charge_hi_fix_pos_slow(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}


pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, special_hi_charge_hi_pre);
    agent.status(Init, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, special_hi_charge_hi_init);
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, special_hi_charge_hi_main);
    agent.status(Exec, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, special_hi_charge_hi_exec);
    agent.status(ExecStop, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, special_hi_charge_hi_exec_stop);
    agent.status(End, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, special_hi_charge_hi_end);
    agent.status(FixPosSlow, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, special_hi_charge_hi_fix_pos_slow);
}