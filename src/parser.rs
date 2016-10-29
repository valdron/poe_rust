use {JsonSite, Stash, Item, Property, Socket, Requirement};
use deser;
use regex::Regex;
use serde_json::Value;
use std::str::FromStr;

#[derive(Debug)]
pub struct RustStash {
    acc_name: String,
    last_char_name: String,
    stash_id: String,
    stash_type: String,
    is_public: bool
}

#[derive(Debug)]
enum PropValue {
    UnqJewels(String),
    Normal(Vec<(f32, f32)>),
    Nothing
}

#[derive(Debug)]
pub struct RustItem {
    contained_in: String,
    item_id: String,
    league: String,
    note: String,
    verified: bool,
    identified: bool,
    corrupted: bool,
    locked_to_char: bool,
    width: i16,
    height: i16,
    item_level: i16,
    icon: String,
    support: Option<bool>,
    // Save as Color links with - nonlinks with |
    sockets: String,
    socket_nr: u8,
    socket_li: u8,
    name: String,
    base_item: String,
    // only parse relevant Name and value
    properties: Vec<(String, PropValue)>,
    // only parse relevant Name and value
    requirements: Vec<(String, i16)>,
    implicit_mods: Vec<(String, i16, i16)>,
    explicit_mods: Vec<(String, i16, i16)>,
    crafted_mods: Vec<(String, i16, i16)>,
    enchanted_mods: Vec<(String, i16, i16)>,
    frame_type: i16,
    x: i16,
    y: i16,
    socketed_items: bool,
}

pub struct Parser {
    re: Vec<Regex>,
    re_for_text: Regex,
    re_for_props: Regex,
}

impl Parser {
    pub fn new(v: Vec<Regex>, t: Regex, p: Regex) -> Parser {
        Parser {
            re: v,
            re_for_text: t,
            re_for_props: p,
        }
    }
    pub fn parse_item(&self, item: Item, s_id: &String) -> Result<RustItem, &str> {
        let rx: i16;
        match item.x {
            Some(x) => rx = x,
            None => return Err("could not parse: no Coords")
        }
        let ry: i16 = item.y.unwrap();

        let note: String = match item.note {
            Some(s) => s,
            None => String::new()
        };
        let (sockets, socket_nr, socket_li): (String, u8, u8) = self.parse_socket(item.sockets);

        let socketed_items: bool = match item.socketed_items.len() {
            0 => false,
            _ => true
        };


        let requirements: Vec<(String, i16)> = match self.parse_requirements(item.requirements) {
            Ok(x) => x,
            Err(y) => return Err(y),
        };

        let implicit_mods: Vec<(String, i16, i16)> = match self.parse_mods(item.implicit_mods) {
            Ok(x) => x,
            Err(y) => return Err(y),
        };
        let explicit_mods: Vec<(String, i16, i16)> = match self.parse_mods(item.explicit_mods) {
            Ok(x) => x,
            Err(y) => return Err(y),
        };
        let crafted_mods: Vec<(String, i16, i16)> = match self.parse_mods(item.crafted_mods) {
            Ok(x) => x,
            Err(y) => return Err(y),
        };
        let properties: Vec<(String, PropValue)> = match self.parse_props(item.properties) {
            Ok(x) => x,
            Err(y) => {
                print!("Frametype: {}", item.frame_type);
                return Err(y)
            },
        };
        let enchanted_mods: Vec<(String, i16, i16)> = match self.parse_mods(item.enchanted_mods) {
            Ok(x) => x,
            Err(y) => return Err(y),
        };


        Ok(RustItem {
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
            icon: item.icon,
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
    fn parse_socket(&self, s: Vec<Socket>) -> (String, u8, u8) {
        match s.len() {
            0 => return ("".to_string(), 0, 0),
            _ => {
                let mut number: u8 = 1;
                let mut counter: u8 = 0;
                let mut max: u8 = 0;
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

    fn parse_requirements(&self, r: Option<Vec<Requirement>>) -> Result<Vec<(String, i16)>, &'static str> {
        match r {
            Some(v) => {
                let mut result: Vec<(String, i16)> = Vec::new();
                for req in v {
                    let value: i16 = match ((req.values[0])[0]) {
                        Value::String(ref x) => i16::from_str_radix(x.as_str(), 10).unwrap(),
                        _ => return Err("could not parse requirement"),
                    };
                    result.push((req.name, value));
                }
                Ok(result)
            },
            None => Ok(Vec::new())
        }
    }

    fn parse_mods(&self, mods: Option<Vec<String>>) -> Result<Vec<(String, i16, i16)>, &str> {
        match mods {
            Some(v) => {
                let mut result: Vec<(String, i16, i16)> = Vec::new();
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
                                result.push((text, val1, val2));
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

    fn parse_props(&self, props: Option<Vec<Property>>) -> Result<Vec<(String, PropValue)>, &str> {
        match props {
            Some(x) => {
                let mut result: Vec<(String, PropValue)> = Vec::new();
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
                            result.push((name, PropValue::Nothing));
                            break;
                        },
                        _ => {},
                    }
                    let mut val = PropValue::Nothing;
                    let name = p.name.clone();

                    for v in p.values {
                        match val {
                            PropValue::Nothing => {
                                let caps: Option<super::regex::Captures> = match v[0] {
                                    Value::String(ref s) => {
                                        self.re_for_props.captures(s.as_str())
                                    }
                                    _ => return Err("none string value in property")
                                };
                                match caps {
                                    Some(x) => {
                                        let val1 = f32::from_str(x.at(1).unwrap_or("0.0")).unwrap();
                                        let val2 = f32::from_str(x.at(2).unwrap_or("0.0")).unwrap();
                                        val = PropValue::Normal(vec![(val1, val2)]);
                                    },

                                    None => {
                                        let s = match v[0] {
                                            Value::String(ref s) => s.clone(),
                                            _ => { return Err("very weird check mod"); }
                                        };
                                        val = PropValue::UnqJewels(s)
                                    },
                                }
                            },
                            PropValue::Normal(ref mut n) => {
                                let caps: Option<super::regex::Captures> = match v[0] {
                                    Value::String(ref s) => {
                                        self.re_for_props.captures(s.as_str())
                                    }
                                    _ => return Err("none string value in property")
                                };
                                match caps {
                                    Some(x) => {
                                        let val1 = f32::from_str(x.at(1).unwrap_or("0.0")).unwrap();
                                        let val2 = f32::from_str(x.at(2).unwrap_or("0.0")).unwrap();
                                        n.push((val1, val2))

                                    },

                                    None => {
                                        return Err("expected another normal Porperty")
                                    },
                                }
                            }
                            PropValue::UnqJewels(s) => { return Err("there should be no other value in this property :/") }
                        }
                    }
                result.push((name, val))
            }
            Ok(result)
        },
        None => return Ok(Vec::new()),
    }
}
}