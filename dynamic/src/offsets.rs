extern "C" {
    #[link_name = "offsets_force_linear_histun"]
    fn offsets_force_linear_histun() -> usize;

    #[link_name = "offsets_get_param_int_impl"]
    fn offsets_get_param_int_impl() -> usize;

    #[link_name = "offsets_get_param_float_impl"]
    fn offsets_get_param_float_impl() -> usize;

    #[link_name = "offsets_set_fighter_vtable"]
    fn offsets_set_fighter_vtable() -> usize;

    #[link_name = "offsets_set_weapon_vtable"]
    fn offsets_set_weapon_vtable() -> usize;

    #[link_name = "offsets_set_item_vtable"]
    fn offsets_set_item_vtable() -> usize;

    #[link_name = "offsets_get_battle_object_from_id"]
    fn offsets_get_battle_object_from_id() -> usize;
}

pub fn force_linear_histun() -> usize {
    unsafe {
        offsets_force_linear_histun()
    }
}

pub fn get_param_int_impl() -> usize {
    unsafe {
        offsets_get_param_int_impl()
    }
}

pub fn get_param_float_impl() -> usize {
    unsafe {
        offsets_get_param_float_impl()
    }
}

pub fn set_fighter_vtable() -> usize {
    unsafe {
        offsets_set_fighter_vtable()
    }
}

pub fn set_weapon_vtable() -> usize {
    unsafe {
        offsets_set_weapon_vtable()
    }
}

pub fn set_item_vtable() -> usize {
    unsafe {
        offsets_set_item_vtable()
    }
}

pub fn get_battle_object_from_id() -> usize {
    unsafe {
        offsets_get_battle_object_from_id()
    }
}