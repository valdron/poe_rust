use std::vec::Vec;
use std::string::String;

struct Property {
    name: String,
    value: Vec<Vec<String>>,
    display_mode: i8
}

struct Socket {
    
}

struct Item {
    verified: bool,
    width: i8,
    height: i8,
    item_level: i8,
    icon: String,
    league: String,
    item_id: String,
    sockets: Vec<Sockets>,
    name: String,
    base_item: String,
    identified: bool,
    corrupted: bool,
    locked_to_char: bool,
    note: String,
    properties: Vec<Property>,
    requirements: Vec<Requirement>,
    implicit_mods: Vec<String>,
    explicit_mods: Vec<String>,
    crafted_mods: Vec<String>,
    enchanted_mods: Vec<String>,
    descr_text: String,
    frame_type: i8, // 0 normal 1 magic 2 rare 3 unique 4 gems 5 currency 6 div cards 8 prophecies
    x: i8,
    y: i8,
    socketed_items: Vec<Item>

}

struct Stash {
    acc_name: String,
    last_char_name: String,
    stash_id: String,
    stash_name: String,
    stash_type: String,
    items: Vec<Item>,
    is_public: bool

}

struct JsonSite {
    next_id: String,
    stashes: Vec<Stash>
}

