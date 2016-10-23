




#[derive(Serialize, Deserialize)]
struct Requirement {
    name: String,
    values: Vec<Vec<serde_json::Value>>,
    #[serde(rename = "displayMode")]
    display_mode: i8

}
#[derive(Serialize, Deserialize)]
struct Property {
    name: String,
    values: Vec<Vec<serde_json::Value>>,
    #[serde(rename = "displayMode")]
    display_mode: i8
}
#[derive(Serialize, Deserialize)]
struct Socket {
    group: i8,
    #[serde(rename = "attr")]
    attribute: String,
}
#[derive(Serialize, Deserialize)]
struct Item {
    verified: bool,
    #[serde(rename = "w")]
    width: i8,
    #[serde(rename = "h")]
    height: i8,
    #[serde(rename = "ilvl")]
    item_level: i8,
    icon: String,
    league: String,
    #[serde(rename = "id")]
    item_id: String,
    sockets: Vec<Socket>,
    name: String,
    #[serde(rename = "typeLine")]
    base_item: String,
    identified: bool,
    corrupted: bool,
    #[serde(rename = "lockedToCharacter")]
    locked_to_char: bool,
    note: Option<String>,
    properties: Option<Vec<Property>>,
    requirements: Option<Vec<Requirement>>,
    #[serde(rename = "implicitMods")]
    implicit_mods: Option<Vec<String>>,
    #[serde(rename = "explicitMods")]
    explicit_mods: Option<Vec<String>>,
    #[serde(rename = "craftedMods")]
    crafted_mods: Option<Vec<String>>,
    #[serde(rename = "enchantedMods")]
    enchanted_mods: Option<Vec<String>>,
    #[serde(rename = "descrText")]
    descr_text: Option<String>,
    #[serde(rename = "frameType")]
    frame_type: i8, // 0 normal 1 magic 2 rare 3 unique 4 gems 5 currency 6 div cards 8 prophecies
    x: Option<i8>,
    y: Option<i8>,
    #[serde(rename = "socketedItems")]
    socketed_items: Vec<Item>

}
#[derive(Serialize, Deserialize)]
struct Stash {
    #[serde(rename = "accountName")]
    acc_name: String,
    #[serde(rename = "lastCharacterName")]
    last_char_name: String,
    #[serde(rename = "id")]
    stash_id: String,
    #[serde(rename = "stash")]
    stash_name: String,
    #[serde(rename = "stashType")]
    stash_type: String,
    items: Vec<Item>,
    #[serde(rename = "public")]
    is_public: bool


}
#[derive(Serialize, Deserialize)]
struct JsonSite {
    next_change_id: String,
    stashes: Vec<Stash>
}

#[derive(Serialize, Deserialize)]
struct NextId {
    next_change_id: String,
    stashes: Vec<Stash>
}

