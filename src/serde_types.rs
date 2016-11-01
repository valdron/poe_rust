use serde_json;



#[derive(Serialize, Deserialize, Debug)]
pub struct Requirement {
    pub name: String,
    pub values: Vec<Vec<serde_json::Value>>,
    #[serde(rename = "displayMode")]
    pub display_mode: i16

}
#[derive(Serialize, Deserialize, Debug)]
pub struct Property {
    pub name: String,
    pub values: Vec<Vec<serde_json::Value>>,
    #[serde(rename = "displayMode")]
    pub display_mode: i16
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Socket {
    pub group: i16,
    #[serde(rename = "attr")]
    pub attribute: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub verified: bool,
    #[serde(rename = "w")]
    pub width: i16,
    #[serde(rename = "h")]
    pub height: i16,
    #[serde(rename = "ilvl")]
    pub item_level: i16,
    pub icon: String,
    pub support: Option<bool>,
    pub league: String,
    #[serde(rename = "id")]
    pub item_id: String,
    pub sockets: Vec<Socket>,
    pub name: String,
    #[serde(rename = "typeLine")]
    pub base_item: String,
    pub identified: bool,
    pub corrupted: bool,
    #[serde(rename = "lockedToCharacter")]
    pub locked_to_char: bool,
    pub note: Option<String>,
    pub properties: Option<Vec<Property>>,
    pub requirements: Option<Vec<Requirement>>,
    #[serde(rename = "implicitMods")]
    pub implicit_mods: Option<Vec<String>>,
    #[serde(rename = "explicitMods")]
    pub explicit_mods: Option<Vec<String>>,
    #[serde(rename = "craftedMods")]
    pub crafted_mods: Option<Vec<String>>,
    #[serde(rename = "enchantedMods")]
    pub enchanted_mods: Option<Vec<String>>,
    #[serde(rename = "descrText")]
    pub descr_text: Option<String>,
    #[serde(rename = "frameType")]
    pub frame_type: i16, // 0 normal 1 magic 2 rare 3 unique 4 gems 5 currency 6 div cards 8 prophecies
    pub x: Option<i16>,
    pub y: Option<i16>,
    #[serde(rename = "socketedItems")]
    pub socketed_items: Vec<Item>

}


#[derive(Serialize, Deserialize, Debug)]
pub struct Stash {
    #[serde(rename = "accountName")]
    pub acc_name: serde_json::Value,
    #[serde(rename = "lastCharacterName")]
    pub last_char_name: String,
    #[serde(rename = "id")]
    pub stash_id: String,
    #[serde(rename = "stash")]
    pub stash_name: Option<String>,
    #[serde(rename = "stashType")]
    pub stash_type: String,
    pub items: Vec<Item>,
    #[serde(rename = "public")]
    pub is_public: bool


}
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonSite {
    pub next_change_id: String,
    pub stashes: Vec<Stash>
}


