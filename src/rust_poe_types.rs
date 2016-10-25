pub struct RustStash {
    acc_name: String,
    last_char_name: String,
    stash_id: String,
    stash_type: String,
    is_public: bool
}

pub struct RustItem {
    contained_in: String,
    item_id: String,
    league: String,
    note: String,
    verified: bool,
    identified: bool,
    corrupted: bool,
    locked_to_char:bool,
    width: i8,
    height: i8,
    item_level: i8,
    icon: String,
    // Save as Color links with - nonlinks with |
    sockets: String,
    name: String,
    base_item: String,
    // only parse relevant Name and value
    properties: Vec<(String, i8)>,
    // only parse relevant Name and value
    requirements: Vec<(String, i8)>,
    implicit_mods: Vec<(String, i8, i8)>,
    explicit_mods: Vec<(String, i8, i8)>,
    crafted_mods: Vec<(String, i8, i8)>,
    enchanted_mods: Vec<(String, i8, i8)>,
    frame_type: i8,
    x: i8,
    y: i8,
    socketed_items: bool,
}