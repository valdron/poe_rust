




#[derive(Serialize, Deserialize)]
pub struct Requirement {
    name: String,
    values: Vec<Vec<serde_json::Value>>,
    #[serde(rename = "displayMode")]
    display_mode: i16

}
#[derive(Serialize, Deserialize)]
pub struct Property {
    name: String,
    values: Vec<Vec<serde_json::Value>>,
    #[serde(rename = "displayMode")]
    display_mode: i16
}
#[derive(Serialize, Deserialize)]
pub struct Socket {
    group: i16,
    #[serde(rename = "attr")]
    attribute: String,
}
#[derive(Serialize, Deserialize)]
pub struct Item {
    verified: bool,
    #[serde(rename = "w")]
    width: i16,
    #[serde(rename = "h")]
    height: i16,
    #[serde(rename = "ilvl")]
    item_level: i16,
    icon: String,
    support: Option<bool>,
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
    frame_type: i16, // 0 normal 1 magic 2 rare 3 unique 4 gems 5 currency 6 div cards 8 prophecies
    x: Option<i16>,
    y: Option<i16>,
    #[serde(rename = "socketedItems")]
    socketed_items: Vec<Item>

}


#[derive(Serialize, Deserialize)]
pub struct Stash {
    #[serde(rename = "accountName")]
    acc_name: serde_json::Value,
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
pub struct JsonSite {
    next_change_id: String,
    stashes: Vec<Stash>
}


