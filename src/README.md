

item_tables: 
a9a42a5dbda657f71b077ecd0692acce8d1d29c7dff3437e5ed8708f6cb8838f

CREATE TABLE body_armour (
id              varchar  PRIMARY KEY,
stash_id        varchar,
item_type       item_type,
league          varchar,
price           price,
note            varchar,
verified        bool,
identified      bool,
corrupted       bool,
locked_to_char  bool,
width           smallint,
height          smallint,
item_level      smallint,
support         bool,
sockets         sockets,     
name            varchar,
base_item       varchar,
properties      properties[],
requirements    requirement[],
implicit_mods   mods[],
explicit_mods   mods[],
enchanted_mods  mods[],
crafted_mods    mods[],
frame_type      smallint,
x               smallint,
y               smallint,
socketed_items  bool,
armour          smallint,
energy_shield   smallint,
evasion         smallint,
resistances     smallint,
ele_res         smallint,
maxlife         smallint);

sockets{
    string
    nr
    li_nr
}

properties{
    name varchar,
    mod modvalue[],
}

requirement{
    name varchar,
    value smallint
}

mods{
    name varchar,
    val1 smallint,
    val2 smallint,
}

price{
    comp varchar,
    pref varchar,
    amount real,
    
}

mod_value{
    val1 real,
    val2 real,
}