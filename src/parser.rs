use serde_types::{JsonSite, Stash, Item, Property, Socket, Requirement};
use regex::Regex;
use serde_json::Value;
use std::str::FromStr;
use std::sync::mpsc;
use std::time::Instant;
use time;

#[derive(Debug)]
pub struct RustStash {
    pub stash_id: String,
    pub acc_name: String,
    pub last_char_name: String,
    pub stash_type: String,
    pub stash_name: String,
    pub is_public: bool,
    pub item_nr: i16,
    pub items: Vec<RustItem>
}

#[derive(Debug,FromSql,ToSql)]
#[postgres(name = "itemtype")]
pub enum ItemType {
    Unknown,
    DivinationCard,
    Currency,
    Prophecy,
    Gem,
    Jewel,
    Flask,
    Map,
    MapPiece,
    Axe1H,
    Axe2H,
    Mace1H,
    Mace2H,
    Sceptre,
    Bow,
    Dagger,
    Claw,
    Staff,
    Sword1H,
    Sword2H,
    Rapier,
    Wand,
    Amulet,
    Belt,
    Ring,
    Helm,
    Body,
    Boots,
    Gloves,
    Shield,
    Quiver
}

#[derive(Debug, FromSql, ToSql)]
#[postgres(name = "poemod")]
pub struct RustMod{
    name: String,
    #[postgres(name = "value1")]
    val1: i16,
    #[postgres(name = "value2")]
    val2: i16
}

#[derive(Debug, FromSql, ToSql)]
#[postgres(name = "requirement")]
pub struct RustReq{
    name: String,
    #[postgres(name = "value")]
    val: i16,
}

#[derive(Debug, Clone, FromSql, ToSql)]
#[postgres(name = "price")]
pub struct Price {
    #[postgres(name = "prefix")]
    prefix: String,
    #[postgres(name = "value")]
    value: f32,
    #[postgres(name = "suffix")]
    suffix: String,
}

#[derive(Debug, FromSql, ToSql)]
#[postgres(name = "property")]
pub struct RustProperty{
    name: String,
    values: Option<Vec<PropValues>>
}

#[derive(Debug, FromSql, ToSql)]
#[postgres(name = "propertyvalue")]
pub struct PropValues {
    #[postgres(name = "value1")]
    val1: f32,
    #[postgres(name = "value2")]
    val2: f32
}

#[derive(Debug, FromSql,ToSql)]
pub struct RustItem {
    pub contained_in: String,
    pub item_id: String,
    pub item_type: ItemType,
    pub league: String,
    pub price: Option<Price>,
    pub note: String,
    pub verified: bool,
    pub identified: bool,
    pub corrupted: bool,
    pub locked_to_char: bool,
    pub width: i16,
    pub height: i16,
    pub item_level: i16,
    pub support: Option<bool>,
    // Save as Color links with - nonlinks with |
    pub sockets: String,
    pub socket_nr: i16,
    pub socket_li: i16,
    pub name: String,
    pub base_item: String,
    // only parse relevant Name and value
    pub properties: Vec<RustProperty>,
    // only parse relevant Name and value
    pub requirements: Vec<RustReq>,
    pub implicit_mods: Vec<RustMod>,
    pub explicit_mods: Vec<RustMod>,
    pub crafted_mods: Vec<RustMod>,
    pub enchanted_mods: Vec<RustMod>,
    pub frame_type: i16,
    pub x: i16,
    pub y: i16,
    pub socketed_items: bool,

    //additional calculations
    //base values
    pub armour: i16,
    pub energy_s: i16,
    pub evasion: i16,

    //pseudo mods
    pub resistance: i16,
    pub ele_resistance: i16,
    pub max_life: i16,
}

//
// TODO: REFACTOR REGEX IN LAZYSTATICS
//

pub struct Parser {
    re: Vec<Regex>,
    re_for_text: Regex,
    re_for_props: Regex,
    re_for_jewels: Regex,
    re_for_flasks: Regex,
    re_for_jewelry: Regex,
    re_for_weapons: Regex,
    re_for_armour: Regex,
    re_for_map: Regex,
    re_for_mappiece: Regex,
    re_for_price: Regex,
    receive_from_deser: mpsc::Receiver<JsonSite>,
    send_to_dbwriter: mpsc::Sender<RustStash>,
    to_logger: mpsc::Sender<String>
}

impl Parser {

    //Constructor
    pub fn new(send: mpsc::Sender<RustStash>, recv: mpsc::Receiver<JsonSite>, to_logger: mpsc::Sender<String>) -> Parser {
        let v = vec![Regex::new("^\\+?([0-9]+)%?.*").unwrap(),
                     Regex::new(".*([0-9]+).*([0-9]+)?.*").unwrap(),
                     Regex::new(".*").unwrap()];
        Parser {
            to_logger: to_logger,
            re_for_price: Regex::new("^~([a-z/]+)\\s([0-9\\.]+)\\s([a-z]{3,})$").unwrap(),
            receive_from_deser: recv,
            send_to_dbwriter: send,
            re_for_map: Regex::new("Map").unwrap(),
            re_for_mappiece: Regex::new("(^Sacrifice)|(^Mortal)|('s Key)|(^Offering to the Goddess)|(^Fragment of the)").unwrap(),
            re_for_armour: Regex::new("(Armour)|(Quiver)").unwrap(),
            re_for_jewels: Regex::new("Jewel").unwrap(),
            re_for_flasks: Regex::new("Flask").unwrap(),
            re_for_jewelry: Regex::new("(Amulet(\\s|$))|(Ring(\\s|$))|(Belt(\\s|$))|(Sash(\\s|$))|(Talisman(\\s|$))").unwrap(),
            re_for_weapons: Regex::new("Weapon").unwrap(),
            re: v,
            re_for_text: Regex::new("[0-9]+").unwrap(),
            re_for_props: Regex::new("([0-9.]+)(?:-([0-9]+))?").unwrap(),
        }
    }


    pub fn start_parsing(&mut self){
        loop {
            let site = self.receive_from_deser.recv();
            match site {
                Ok(x) => {
                    let now = Instant::now();
                    for st in x.stashes {
                        match self.parse_stash(st) {
                            Ok(stash) => {let _ = self.send_to_dbwriter.send(stash);}
                            Err(y) => {let _= self.to_logger.send(format!("{} ",y));}
                        }
                    }

                    let _= self.to_logger.send(format!("{} | Parser\t\t\t--> Site {} parsed successfully {}.{}",
                                                time::at(time::get_time()).ctime(),
                                                x.next_change_id,
                                                now.elapsed().as_secs(),
                                                now.elapsed().subsec_nanos()));
                },
                Err(e) => {
                    let _= self.to_logger.send(format!("{} | Parser\t\t\t--> Error receiving next site: {:?}",
                                                time::at(time::get_time()).ctime(),
                                                e));

                }
            }

        }
    }

    fn parse_stash(&self, stash: Stash) -> Result<RustStash, &str> {

        let acc = match stash.acc_name{
            Value::String(s) => s,
            _ => {String::new()}
        };

        let s_name = match stash.stash_name{
            Some(x) => x,
            None => String::new(),
        };

        let mut itm: Vec<RustItem> = Vec::new();
        let price: Option<Price> = match self.parse_price(&s_name) {
            Ok(x) => Some(x),
            Err(_) => None,
        };

        for i in stash.items{
            match self.parse_item(i,&stash.stash_id, &price) {
                Ok(x) => itm.push(x),
                Err(y) => {
                    return Err(y)
                }
            }
        }

        Ok(RustStash{
            item_nr: itm.len() as i16,
            stash_name: s_name,
            items: itm,
            acc_name: acc,
            last_char_name: stash.last_char_name,
            stash_id: stash.stash_id,
            stash_type: stash.stash_type,
            is_public: stash.is_public,
        })
    }

    //
    // Parse Price out of string
    //

    fn parse_price(&self,s: &String) -> Result<Price, &str>{
        match self.re_for_price.captures(s.as_str()){
            Some(c) => Ok(Price{prefix: String::from(c.at(1).unwrap()),
                           value: f32::from_str(c.at(2).unwrap()).unwrap(),
                            suffix: String::from(c.at(3).unwrap()),}),
            None => Err("no price")
        }
    }

    // TODO: REFACTORING move pseudo and total mods in own method
    // Parse Item and return the rust-native one
    //

    fn parse_item(&self, item: Item, s_id: &String, s_price: &Option<Price>) -> Result<RustItem, &str> {
        let item_type: ItemType = match self.get_item_type(&item) {
            Ok(x) => x,
            Err(x) => {
                return Err(x);
            },
        };

        let rx: i16;
        match item.x {
            Some(x) => rx = x,
            None => return Err("could not parse: no Coords")
        }
        let ry: i16 = item.y.unwrap();

        let price: Option<Price> = match *s_price {
            Some(ref x) => Some(x.clone()),
            None => match item.note {
                Some(ref y) => match self.parse_price(y) {
                    Ok(z) => Some(z),
                    Err(_) => None,
                },
                None => None,
            }
        };

        let note: String = match item.note {
            Some(s) => s,
            None => String::new()
        };
        let (sockets, socket_nr, socket_li): (String, i16, i16) = self.parse_socket(item.sockets);

        let socketed_items: bool = match item.socketed_items.len() {
            0 => false,
            _ => true
        };


        let requirements: Vec<RustReq> = match self.parse_requirements(item.requirements) {
            Ok(x) => x,
            Err(y) => return Err(y),
        };

        let implicit_mods: Vec<RustMod> = match self.parse_mods(item.implicit_mods) {
            Ok(x) => x,
            Err(y) => return Err(y),
        };
        let explicit_mods: Vec<RustMod> = match self.parse_mods(item.explicit_mods) {
            Ok(x) => x,
            Err(y) => return Err(y),
        };
        let crafted_mods: Vec<RustMod> = match self.parse_mods(item.crafted_mods) {
            Ok(x) => x,
            Err(y) => return Err(y),
        };
        let properties: Vec<RustProperty> = match self.parse_props(item.properties) {
            Ok(x) => x,
            Err(x) => {
                return Err(x);
            },
        };
        let enchanted_mods: Vec<RustMod> = match self.parse_mods(item.enchanted_mods) {
            Ok(x) => x,
            Err(y) => return Err(y),
        };


        let mut arm: i16 = 0;
        let mut energy_s: i16 = 0;
        let mut evasion: i16 = 0;


                for prop in &properties {
                    match prop.name {
                        ref x if x == "Armour" => match prop.values {
                            Some(ref v) => arm = v[0].val1 as i16,
                            _ => {},
                         },
                        ref x if x == "Energy Shield" => match prop.values {
                            Some(ref v) => energy_s = v[0].val1 as i16,
                            _ => {},
                        },
                        ref x if x == "Evasion" => match prop.values {
                            Some(ref v) => evasion = v[0].val1 as i16,
                            _ => {},
                        },
                        _=>{}
                    }
                }



        let mut resistance: i16  = 0;
        let mut ele_resistance: i16 = 0;
        let mut max_life: i16 = 0;


        // REGEX for pseudo/total mods
        lazy_static!{
            static ref SINGLE_ELERES: Regex = Regex::new("to\\s(Fire)|(Cold)|(Lightning)\\sResistance$").unwrap();
            static ref DOUBLE_ELERES: Regex = Regex::new("to\\s(Fire)|(Cold)|(Lightning)\\sand\\s(Fire)|(Cold)|(Lightning)\\sResistances$").unwrap();
            static ref ALL_RES: Regex = Regex::new("to\\sall\\sElemental\\sResistances$").unwrap();
            static ref CHAOS_RES: Regex = Regex::new("to\\sChaos\\sResistance$").unwrap();
            static ref MAX_L:  Regex = Regex::new("to\\smaximum\\sLife").unwrap();
            static ref STR:  Regex = Regex::new("to\\sStrength$|(\\sand)").unwrap();
        }

        for mods in &[&explicit_mods, &implicit_mods, &crafted_mods] {
            for mo in *mods {
                match mo.name {
                    ref x if SINGLE_ELERES.is_match(x.as_str()) => {
                        ele_resistance += mo.val1;
                        resistance += mo.val1;
                    },
                    ref x if DOUBLE_ELERES.is_match(x.as_str()) => {
                        ele_resistance += 2 * mo.val1;
                        resistance += 2 * mo.val1;
                    },
                    ref x if ALL_RES.is_match(x.as_str()) => {
                        ele_resistance += 3 * mo.val1;
                        resistance += 3 * mo.val1;
                    },
                    ref x if CHAOS_RES.is_match(x.as_str()) => {
                        resistance += mo.val1;
                    },
                    ref x if MAX_L.is_match(x.as_str()) => {
                        max_life += mo.val1;
                    },
                    ref x if STR.is_match(x.as_str()) => {
                        max_life += mo.val1 / 2;
                    },
                    _ => {}
                }
            }
        }
        //old code
        /*
        for mo in &implicit_mods {
            match *mo {
                (ref x , v1, _) if SINGLE_ELERES.is_match(x.as_str()) => {
                    ele_resistance += v1;
                    resistance += v1;
                },
                (ref x , v1, _) if DOUBLE_ELERES.is_match(x.as_str()) => {
                    ele_resistance += 2*v1;
                    resistance += 2*v1;
                },
                (ref x , v1, _) if ALL_RES.is_match(x.as_str()) => {
                    ele_resistance += 3*v1;
                    resistance += 3*v1;
                },
                (ref x , v1, _) if CHAOS_RES.is_match(x.as_str()) => {
                    resistance += v1;
                },
                (ref x , v1, _) if MAX_L.is_match(x.as_str()) => {
                    max_life += v1;
                },
                (ref x , v1, _) if STR.is_match(x.as_str()) => {
                    max_life += v1/2;
                },
                _=>{}
            }
        }
        for mo in &crafted_mods {
            match *mo {
                (ref x , v1, _) if SINGLE_ELERES.is_match(x.as_str()) => {
                    ele_resistance += v1;
                    resistance += v1;
                },
                (ref x , v1, _) if DOUBLE_ELERES.is_match(x.as_str()) => {
                    ele_resistance += 2*v1;
                    resistance += 2*v1;
                },
                (ref x , v1, _) if ALL_RES.is_match(x.as_str()) => {
                    ele_resistance += 3*v1;
                    resistance += 3*v1;
                },
                (ref x , v1, _) if CHAOS_RES.is_match(x.as_str()) => {
                    resistance += v1;
                },
                (ref x , v1, _) if MAX_L.is_match(x.as_str()) => {
                    max_life += v1;
                },
                (ref x , v1, _) if STR.is_match(x.as_str()) => {
                    max_life += v1/2;
                },
                _=>{}
            }

        }*/

        Ok(RustItem {
            armour: arm,
            energy_s: energy_s,
            evasion: evasion,
            resistance: resistance,
            ele_resistance: ele_resistance,
            max_life: max_life,
            price: price,
            item_type: item_type,
            contained_in: s_id.clone(),
            item_id: item.item_id,
            league: item.league,
            note: note,
            verified: item.verified,
            identified: item.identified,
            corrupted: item.corrupted,
            locked_to_char: item.locked_to_char,
            width: item.width,
            height: item.height,
            item_level: item.item_level,
            support: item.support,
            sockets: sockets,
            socket_nr: socket_nr,
            socket_li: socket_li,
            name: item.name,
            base_item: item.base_item,
            properties: properties,
            requirements: requirements,
            implicit_mods: implicit_mods,
            explicit_mods: explicit_mods,
            crafted_mods: crafted_mods,
            enchanted_mods: enchanted_mods,
            frame_type: item.frame_type,
            x: rx,
            y: ry,
            socketed_items: socketed_items,
        })
    }

    //
    // Parse Sockets
    // Return String: Example: |D-D-S|I-D| => 2 socket-groups First: 2Green 1Red Second 1Blue 1Green

    fn parse_socket(&self, s: Vec<Socket>) -> (String, i16, i16) {
        match s.len() {
            0 => return ("".to_string(), 0, 0),
            _ => {
                let mut number: i16 = 1;
                let mut counter: i16 = 0;
                let mut max: i16 = 0;
                let mut curr_group: i16 = -1;
                let mut str = String::new();
                for s in s.iter() {
                    let (g, s) = (s.group, s.attribute.clone());
                    if g == curr_group {
                        str.push('-');
                        str.push_str(s.as_str());
                        counter += 1;
                    } else {
                        str.push('|');
                        str.push_str(s.as_str());
                        curr_group = g;
                        counter = 1;
                    }
                    if counter > max { max = counter; }
                    number += 1;
                }
                return (str, number, max);
            }
        }
    }

    //
    // Parse Requirements of the Item and return as a Vector
    //

    fn parse_requirements(&self, r: Option<Vec<Requirement>>) -> Result<Vec<RustReq>, &str> {
        match r {
            Some(v) => {
                let mut result: Vec<RustReq> = Vec::new();
                for req in v {
                    let value: i16 = match (req.values[0])[0] {
                        Value::String(ref x) => i16::from_str_radix(x.as_str(), 10).unwrap(),
                        _ => return Err("could not parse requirement"),
                    };
                    result.push(RustReq{name: req.name, val: value});
                }
                Ok(result)
            },
            None => Ok(Vec::new())
        }
    }

    // TODO: REFACTOR
    // Parse the mods of the Item and return as a Vector
    //

    fn parse_mods(&self, mods: Option<Vec<String>>) -> Result<Vec<RustMod>, &str> {
        match mods {
            Some(v) => {
                let mut result: Vec<RustMod> = Vec::new();
                'mods: for m in v {
                    for r in &self.re {
                        match r.is_match(m.as_str()) {
                            true => {
                                let cap = r.captures(m.as_str()).unwrap();
                                let text = self.re_for_text.replace_all(cap.at(0).unwrap(), "##");
                                let val1 = match cap.at(1) {
                                    Some(x) => i16::from_str_radix(x, 10).unwrap(),
                                    None => 0
                                };
                                let val2 = match cap.at(2) {
                                    Some(x) => i16::from_str_radix(x, 10).unwrap(),
                                    None => 0,
                                };
                                result.push(RustMod{name: text, val1: val1, val2: val2});
                                continue 'mods;
                            },
                            false => continue,
                        }
                    }
                    println!("{}", m);
                    return Err("could not parse this mod")
                }
                Ok(result)
            }
            None => Ok(Vec::new())
        }
    }

    // TODO: REFACTOR THE PROPVALUE OUT
    // Parse the Properties of the Item and Return them as a Vector
    //

    fn parse_props(&self, props: Option<Vec<Property>>) -> Result<Vec<RustProperty>, &str> {
        match props {
            Some(x) => {
                let mut result: Vec<RustProperty> = Vec::new();
                for p in x {
                    match p.name.is_empty() {
                        true => {
                            let name = match p.values[0][0] {
                                Value::String(ref s) => s.clone(),
                                _ => {
                                    println!("{} ", p.name);
                                    return Err("weird layout check mod");
                                }
                            };
                            result.push(RustProperty{
                                name: name,
                                values: None,
                            });
                            break;
                        },
                        _ => {},
                    }

                    let name = p.name.clone();
                    let mut prop: RustProperty = RustProperty{
                        name: name,
                        values: None,
                    };

                    for v in p.values {
                        let caps = match v[0] {
                            Value::String(ref s) => {
                                self.re_for_props.captures(s.as_str())
                            }
                            _ => return Err("none string value in property")
                        };
                        match caps {
                            None => {
                                match prop.values {
                                    None => {}
                                    Some(_) => return Err("Found no caps after normal Propvalue on")
                                }
                                break;
                            }
                            Some(x) => {
                                let value1 = f32::from_str(x.at(1).unwrap_or("0.0")).unwrap();
                                let value2 = f32::from_str(x.at(2).unwrap_or("0.0")).unwrap();
                                match prop.values {
                                    None => {
                                        prop.values = Some(vec!(PropValues{val1: value1, val2: value2}));
                                    },
                                    Some( ref mut v) => {
                                        v.push(PropValues{val1: value1, val2: value2});
                                    }
                                }
                            },
                        }
                        }
                    result.push(prop)
                }
                Ok(result)
            },
        None => return Ok(Vec::new()),
        }
    }

    //
    // Trying to determine the type of item and returning the appropriate Variant
    //

    fn get_item_type(&self, item: &Item) -> Result<ItemType, &str> {

        //Check if determenible by frametype
        match item.frame_type{
            4 => return Ok(ItemType::Gem),
            5 => return Ok(ItemType::Currency),
            6 => return Ok(ItemType::DivinationCard),
            8 => return Ok(ItemType::Prophecy),
            _ => {}
        }

        //check if it is a jewel by type line
        if self.re_for_jewels.is_match(&item.base_item.as_str()) {return Ok(ItemType::Jewel)}

        //check if it is a flask by typeline
        if self.re_for_flasks.is_match(&item.base_item.as_str()) {return Ok(ItemType::Flask)}

        //check if it is jewelry by typeline and determine wich kind
        if self.re_for_jewelry.is_match(&item.base_item.as_str()) {
                match self.get_jewelry_type(&item.base_item) {
                    Ok(x) => return Ok(x),
                    Err(e) => return Err(e),
                }
        }

        //check if it is a map by typeline
        if self.re_for_map.is_match(&item.base_item.as_str()){return Ok(ItemType::Map)}

        //check if it is a mappiece by typeline
        if self.re_for_mappiece.is_match(&item.base_item.as_str()) {return Ok(ItemType::MapPiece)}

        //check if it is a weapen and when which kind
        if self.re_for_weapons.is_match(&item.icon) {
                match self.get_weapon_type(&item.icon) {
                    Ok(x) => return Ok(x),
                    Err(e) => return Err(e),
                }
        }

        //check if it is armour and when which kind
        if self.re_for_armour.is_match(&item.icon){
                match self.get_armour_type(&item.icon) {
                    Ok(x) => return Ok(x),
                    Err(e) => return Err(e),
                }
        }

        Ok(ItemType::Unknown)
    }

    //
    // Determine which kind of Jewelry it is by trying to match it with RegEx
    //

    fn get_jewelry_type(&self, s: &String) -> Result<ItemType, &str> {

        lazy_static!{
            static ref RING: Regex = Regex::new("Ring").unwrap();
            static ref AMULET: Regex = Regex::new("(Amulet)|(Talisman)").unwrap();
            static ref BELT: Regex = Regex::new("(Belt)|(Sash)").unwrap();
        }

        if RING.is_match(s.as_str()) {return Ok(ItemType::Ring)}
        if AMULET.is_match(s.as_str()) {return Ok(ItemType::Amulet)}
        if BELT.is_match(s.as_str()) {return Ok(ItemType::Belt)}

        Err("Amulet_type could not be determined")
    }

    //
    // Determine which kind of Weapon it is by trying to match it with RegEx
    //

    fn get_weapon_type(&self, s: &String) -> Result<ItemType, &str> {
        lazy_static!{
            static ref ONEH: Regex = Regex::new("OneHandWeapons").unwrap();
            static ref TWOH: Regex = Regex::new("TwoHandWeapons").unwrap();
            static ref AXE: Regex = Regex::new("Axe").unwrap();
            static ref SWORD: Regex = Regex::new("Sword").unwrap();
            static ref MACE: Regex = Regex::new("Mace").unwrap();
            static ref BOW: Regex = Regex::new("Bows").unwrap();
            static ref WAND: Regex = Regex::new("Wands").unwrap();
            static ref CLAW: Regex = Regex::new("Claws").unwrap();
            static ref STAFF: Regex = Regex::new("Staves").unwrap();
            static ref DAGGER: Regex = Regex::new("Daggers").unwrap();
            static ref SCEPTER: Regex = Regex::new("Scepter").unwrap();
            static ref RAPIER: Regex = Regex::new("Rapier").unwrap();
        }


        if ONEH.is_match(s.as_str()) {
            if AXE.is_match(s.as_str()) {return Ok(ItemType::Axe1H)}
            if MACE.is_match(s.as_str()) {return Ok(ItemType::Mace1H)}
            if SWORD.is_match(s.as_str()) {return Ok(ItemType::Sword1H)}
            if CLAW.is_match(s.as_str()) {return Ok(ItemType::Claw)}
            if DAGGER.is_match(s.as_str()) {return Ok(ItemType::Dagger)}
            if WAND.is_match(s.as_str()) {return Ok(ItemType::Wand)}
            if SCEPTER.is_match(s.as_str()) {return Ok(ItemType::Sceptre)}
            if RAPIER.is_match(s.as_str()) {return Ok(ItemType::Rapier)}


        }

        if TWOH.is_match(s.as_str()) {
            if MACE.is_match(s.as_str()) {return Ok(ItemType::Mace2H)}
            if AXE.is_match(s.as_str()) {return Ok(ItemType::Axe2H)}
            if SWORD.is_match(s.as_str()) {return Ok(ItemType::Sword2H)}
            if STAFF.is_match(s.as_str()) {return Ok(ItemType::Staff)}
            if BOW.is_match(s.as_str()) {return Ok(ItemType::Bow)}
        }

        Err("ItemType not found")

    }

    //
    // Determine which kind of Armour it is by trying to match it with RegEx
    //

    fn get_armour_type(&self, s: &String) -> Result<ItemType, &str> {
        lazy_static!{
            static ref BODY: Regex = Regex::new("BodyArmours").unwrap();
            static ref HELM: Regex = Regex::new("Helmets").unwrap();
            static ref SHIELD: Regex = Regex::new("Shields").unwrap();
            static ref GLOVES: Regex = Regex::new("Gloves").unwrap();
            static ref BOOTS: Regex = Regex::new("Boots").unwrap();
            static ref QUIVER: Regex = Regex::new("Quiver").unwrap();
        }

        if BODY.is_match(s.as_str()) {return Ok(ItemType::Body)}
        if HELM.is_match(s.as_str()) {return Ok(ItemType::Helm)}
        if SHIELD.is_match(s.as_str()) {return Ok(ItemType::Shield)}
        if GLOVES.is_match(s.as_str()) {return Ok(ItemType::Gloves)}
        if BOOTS.is_match(s.as_str()) {return Ok(ItemType::Boots)}
        if QUIVER.is_match(s.as_str()) {return Ok(ItemType::Quiver)}

        Err("Armour Type not found")

    }
}