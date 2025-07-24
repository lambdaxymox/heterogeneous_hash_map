use heterogeneous_hash_map::{
    HeterogeneousHashMap,
    HomogeneousHashMap,
    Key,
};

use core::any;
use core::fmt;
use alloc_crate::string::{String, ToString};
use std::hash;

use hashbrown::hash_map;

#[derive(Clone, Debug)]
struct WrappingBuildHasher<S> {
    build_hasher: S,
}

impl<S> WrappingBuildHasher<S> {
    fn new(build_hasher: S) -> Self {
        Self { build_hasher }
    }
}

impl<S> hash::BuildHasher for WrappingBuildHasher<S>
where
    S: hash::BuildHasher,
{
    type Hasher = S::Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        self.build_hasher.build_hasher()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PlayerName(String);

impl From<&str> for PlayerName {
    fn from(name: &str) -> Self {
        PlayerName(String::from(name))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CharacterName(String);

impl From<&str> for CharacterName {
    fn from(st: &str) -> Self {
        CharacterName(String::from(st))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Race(String);

impl From<&str> for Race {
    fn from(st: &str) -> Self {
        Race(String::from(st))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Class(String);

impl From<&str> for Class {
    fn from(st: &str) -> Self {
        Class(String::from(st))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Job(String);

impl From<&str> for Job {
    fn from(st: &str) -> Self {
        Job(String::from(st))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Description(String);

impl From<&str> for Description {
    fn from(st: &str) -> Self {
        Description(String::from(st))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Age(u32);

impl From<u32> for Age {
    fn from(age: u32) -> Self {
        Age(age)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Stats {
    strength: u32,
    dexterity: u32,
    agility: u32,
    intelligence: u32,
    perception: u32,
    mind: u32,
    luck: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct HitPoints(u32);

impl From<u32> for HitPoints {
    fn from(value: u32) -> Self {
        HitPoints(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MagicPoints(u32);

impl From<u32> for MagicPoints {
    fn from(value: u32) -> Self {
        MagicPoints(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Chuunibyou(u32);

impl From<u32> for Chuunibyou {
    fn from(value: u32) -> Self {
        Chuunibyou(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Status(String);

impl From<&str> for Status {
    fn from(st: &str) -> Self {
        Status(String::from(st))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Equipment(String);

impl From<&str> for Equipment {
    fn from(st: &str) -> Self {
        Equipment(String::from(st))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Familiar(String);

impl From<&str> for Familiar {
    fn from(st: &str) -> Self {
        Familiar(String::from(st))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AbilityClass(String);

impl From<&str> for AbilityClass {
    fn from(st: &str) -> Self {
        AbilityClass(String::from(st))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Ability {
    class: AbilityClass,
    name: String,
    cost: u32,
}

impl Ability {
    fn new(class: AbilityClass, name: &str, cost: u32) -> Self {
        Self {
            class,
            name: String::from(name),
            cost,
        }
    }

    #[inline]
    fn class(&self) -> &AbilityClass {
        &self.class
    }

    #[inline]
    fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    const fn cost(&self) -> u32 {
        self.cost
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct InventoryItem {
    name: String,
    quantity: u32,
}

impl InventoryItem {
    #[inline]
    fn new(name: &str, quantity: u32) -> Self {
        InventoryItem {
            name: String::from(name),
            quantity,
        }
    }

    #[inline]
    fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    const fn quantity(&self) -> u32 {
        self.quantity
    }
}

#[rustfmt::skip]
fn get_character_map_kazuma() -> HeterogeneousHashMap<usize> {
    let mut character = HeterogeneousHashMap::new();
    character.insert::<CharacterName>(Key::new(1_usize), CharacterName::from("Kazuma Satou"));
    character.insert::<CharacterName>(Key::new(2_usize), CharacterName::from("Kazutrash"));
    character.insert::<PlayerName>(Key::new(1_usize), PlayerName::from("I'm Kazuma"));
    character.insert::<PlayerName>(Key::new(2_usize), PlayerName::from("That's My Name"));
    character.insert::<PlayerName>(Key::new(3_usize), PlayerName::from("Kazuma Satou"));
    character.insert::<Age>(Key::new(1_usize), Age::from(17));
    character.insert::<Race>(Key::new(1_usize), Race::from("Human"));
    character.insert::<Class>(Key::new(1_usize), Class::from("Adventurer"));
    character.insert::<Job>(Key::new(1_usize), Job::from("Jack of All Trades, Master Of Dumb Luck"));
    character.insert::<Job>(Key::new(2_usize), Job::from("Definitely Not A Harem Protagonist"));
    character.insert::<Job>(Key::new(3_usize), Job::from("Strategic Coward"));
    character.insert::<Status>(Key::new(1_usize), Status::from("Alive"));
    character.insert::<Description>(Key::new(1_usize), Description::from("\
        Slovenly shut-in NEET with questionable morals and a surprisingly sharp wit.\
    "));
    character.insert::<Stats>(Key::new(1_usize), Stats {
        strength: 12,
        dexterity: 14,
        agility: 13,
        intelligence: 18,
        perception: 14,
        mind: 10,
        luck: 99,
    });
    character.insert::<HitPoints>(Key::new(1_usize), HitPoints(40));
    character.insert::<MagicPoints>(Key::new(1_usize), MagicPoints(20));
    character.insert::<Chuunibyou>(Key::new(1_usize), Chuunibyou(0));
    character.insert::<Equipment>(Key::new(1_usize), Equipment::from("Chunchunmaru"));
    character.insert::<Equipment>(Key::new(2_usize), Equipment::from("Mass-Produced Vanir Mask"));
    character.insert::<Equipment>(Key::new(3_usize), Equipment::from("Cursed Ring"));
    character.insert::<InventoryItem>(Key::new(1_usize), InventoryItem::new("Adventurer Card", 1));
    character.insert::<InventoryItem>(Key::new(2_usize), InventoryItem::new("Green Tracksuit", 1));
    character.insert::<InventoryItem>(Key::new(3_usize), InventoryItem::new("Bottle Of Poison", 5));
    character.insert::<InventoryItem>(Key::new(4_usize), InventoryItem::new("Tinymite", 2));
    character.insert::<InventoryItem>(Key::new(5_usize), InventoryItem::new("Silver Arrows", 20));
    character.insert::<InventoryItem>(Key::new(6_usize), InventoryItem::new("Dream Consultation Form", 99));
    character.insert::<Ability>(Key::new(1_usize),  Ability::new(AbilityClass::from("Wizard"), "Create Water", 1));
    character.insert::<Ability>(Key::new(2_usize),  Ability::new(AbilityClass::from("Wizard"), "Freeze", 2));
    character.insert::<Ability>(Key::new(3_usize),  Ability::new(AbilityClass::from("Undead"), "Drain Touch", 3));
    character.insert::<Ability>(Key::new(4_usize),  Ability::new(AbilityClass::from("Thief"), "Steal", 1));
    character.insert::<Ability>(Key::new(5_usize),  Ability::new(AbilityClass::from("Thief"), "Lurk", 1));
    character.insert::<Ability>(Key::new(6_usize),  Ability::new(AbilityClass::from("Thief"), "Enemy Detection", 1));
    character.insert::<Ability>(Key::new(7_usize),  Ability::new(AbilityClass::from("Thief"), "Trap Detection", 1));
    character.insert::<Ability>(Key::new(8_usize),  Ability::new(AbilityClass::from("Thief"), "Disarm Trap", 1));
    character.insert::<Ability>(Key::new(9_usize),  Ability::new(AbilityClass::from("Thief"), "Flee", 1));
    character.insert::<Ability>(Key::new(10_usize), Ability::new(AbilityClass::from("Thief"), "Bind", 1));

    character
}

fn get_character_map_megumin() -> HeterogeneousHashMap<usize> {
    let mut character = HeterogeneousHashMap::new();
    character.insert::<CharacterName>(Key::new(1_usize), CharacterName::from("Megumin"));
    character.insert::<CharacterName>(Key::new(2_usize), CharacterName::from("Explosion Maniac"));
    character.insert::<CharacterName>(Key::new(3_usize), CharacterName::from("Crazy Explosion Girl"));
    character.insert::<Age>(Key::new(1_usize), Age::from(14));
    character.insert::<Race>(Key::new(1_usize), Race::from("Human"));
    character.insert::<Race>(Key::new(2_usize), Race::from("Crimson Magic Clan"));
    character.insert::<Class>(Key::new(1_usize), Class::from("Arch Wizard"));
    character.insert::<Status>(Key::new(1_usize), Status::from("Alive"));
    character.insert::<Description>(Key::new(1_usize), Description::from("\
        I am MEGUMIN! The greatest wizard of the CRIMSON DEMON CLAN! The user of EXPLOSION magic!\
    "));
    character.insert::<Description>(Key::new(2_usize), Description::from("\
        NOTE (Luna): Adventurer refused to provide a standard description. She forced this one in all caps.\
    "));
    character.insert::<Stats>(Key::new(1_usize), Stats {
        strength: 14,
        dexterity: 10,
        agility: 10,
        intelligence: 25,
        perception: 14,
        mind: 24,
        luck: 10,
    });
    character.insert::<HitPoints>(Key::new(1_usize), HitPoints(20));
    character.insert::<MagicPoints>(Key::new(1_usize), MagicPoints(999));
    character.insert::<Chuunibyou>(Key::new(1_usize), Chuunibyou(u32::MAX));
    character.insert::<Equipment>(Key::new(1_usize), Equipment::from("Magic Rod"));
    character.insert::<Equipment>(Key::new(2_usize), Equipment::from("Big Floppy Wizard Hat"));
    character.insert::<Equipment>(Key::new(3_usize), Equipment::from("Adventurer's Cloak"));
    character.insert::<Equipment>(Key::new(4_usize), Equipment::from("Demon Ring"));
    character.insert::<Equipment>(Key::new(5_usize), Equipment::from("Eye Patch"));
    character.insert::<InventoryItem>(Key::new(1_usize), InventoryItem::new("Light Of Reflection Scroll", 1));
    character.insert::<InventoryItem>(Key::new(2_usize), InventoryItem::new("Sword Of Shack The Ripper", 1));
    character.insert::<InventoryItem>(Key::new(3_usize), InventoryItem::new("Highest-Quality Manatites", 3));
    character.insert::<Familiar>(Key::new(1_usize), Familiar::from("Chomusuke"));
    character.insert::<Status>(Key::new(1_usize), Status::from("Alive"));
    character.insert::<Ability>(Key::new(1_usize), Ability::new(AbilityClass::from("Wizard"), "EXPLOSION!!!", 999));

    character
}

#[rustfmt::skip]
fn get_character_map_aqua() -> HeterogeneousHashMap<usize> {
    let mut character = HeterogeneousHashMap::new();
    character.insert::<CharacterName>(Key::new(1_usize), CharacterName::from("Aqua"));
    character.insert::<CharacterName>(Key::new(2_usize), CharacterName::from("Lady Aqua"));
    character.insert::<CharacterName>(Key::new(3_usize), CharacterName::from("Goddess Of Party Tricks"));
    character.insert::<PlayerName>(Key::new(1_usize), PlayerName::from("Aqua"));
    character.insert::<Age>(Key::new(1_usize), Age::from(16));
    character.insert::<Age>(Key::new(2_usize), Age::from(u32::MAX));
    character.insert::<Race>(Key::new(1_usize), Race::from("God"));
    character.insert::<Class>(Key::new(1_usize), Class::from("Arch Priest"));
    character.insert::<Job>(Key::new(1_usize), Job::from("Self-Proclaimed Goddess"));
    character.insert::<Status>(Key::new(1_usize), Status::from("Alive"));
    character.insert::<Description>(Key::new(1_usize), Description::from("\
        Useless water goddess of the Axis church. Somehow, her followers are even crazier than she is.\
    "));
    character.insert::<Description>(Key::new(2_usize), Description::from("\
        Self-proclaimed goddess who specializes in getting us into trouble and drinking all the party’s funds. Blessings included, probably.\
    "));
    character.insert::<Description>(Key::new(3_usize), Description::from("\
        A self-proclaimed goddess notorious for causing disasters and drinking all the booze. Worship at your own risk.\
    "));
    character.insert::<Description>(Key::new(4_usize), Description::from("\
        Patron deity of purification, renewal, and... occasional self-sabotage. Pray hard, avoid the frogs.\
    "));
    character.insert::<Description>(Key::new(5_usize), Description::from("\
        Revered water goddess of the Axis Church, renowned for her unparalleled purity and unwavering dedication to her followers' prosperity. \
        Her divine blessings ensure the flourishing of faith and the cleansing of corruption across the realm.\
    "));

    character.insert::<Stats>(Key::new(1_usize), Stats {
        strength: 18,
        dexterity: 11,
        agility: 14,
        intelligence: 14,
        perception: 9,
        mind: u32::MAX,
        luck: 1,
    });
    character.insert::<HitPoints>(Key::new(1_usize), HitPoints(60));
    character.insert::<MagicPoints>(Key::new(1_usize), MagicPoints(u32::MAX));
    character.insert::<Chuunibyou>(Key::new(1_usize), Chuunibyou(0));
    character.insert::<Equipment>(Key::new(1_usize), Equipment::from("Feather Mantle"));
    character.insert::<Equipment>(Key::new(2_usize), Equipment::from("Scepter"));
    character.insert::<InventoryItem>(Key::new(1_usize), InventoryItem::new("Jarred Snow Sprite", 1));
    character.insert::<InventoryItem>(Key::new(2_usize), InventoryItem::new("Bubbly", 0));
    character.insert::<InventoryItem>(Key::new(3_usize), InventoryItem::new("Coins", 0));
    character.insert::<Ability>(Key::new(1_usize),  Ability::new(AbilityClass::from("Party Trick"), "Nature's Beauty", 1));
    character.insert::<Ability>(Key::new(2_usize),  Ability::new(AbilityClass::from("Water Magic"), "Create Water", 1));
    character.insert::<Ability>(Key::new(3_usize),  Ability::new(AbilityClass::from("Water Magic"), "Sacred Create Water", 5));
    character.insert::<Ability>(Key::new(4_usize),  Ability::new(AbilityClass::from("Water Magic"), "Purification", 1));
    character.insert::<Ability>(Key::new(5_usize),  Ability::new(AbilityClass::from("Water Magic"), "Holy Water", 1));
    character.insert::<Ability>(Key::new(6_usize),  Ability::new(AbilityClass::from("Holy Magic"), "Heal", 5));
    character.insert::<Ability>(Key::new(7_usize),  Ability::new(AbilityClass::from("Holy Magic"), "Sacred Highness Heal", 20));
    character.insert::<Ability>(Key::new(8_usize),  Ability::new(AbilityClass::from("Holy Magic"), "Turn Undead", 5));
    character.insert::<Ability>(Key::new(9_usize),  Ability::new(AbilityClass::from("Holy Magic"), "Sacred Turn Undead", 20));
    character.insert::<Ability>(Key::new(10_usize), Ability::new(AbilityClass::from("Holy Magic"), "Exorcism", 5));
    character.insert::<Ability>(Key::new(11_usize), Ability::new(AbilityClass::from("Holy Magic"), "Sacred Exorcism", 20));
    character.insert::<Ability>(Key::new(12_usize), Ability::new(AbilityClass::from("Holy Magic"), "Break Spell", 7));
    character.insert::<Ability>(Key::new(13_usize), Ability::new(AbilityClass::from("Holy Magic"), "Sacred Break Spell", 24));
    character.insert::<Ability>(Key::new(14_usize), Ability::new(AbilityClass::from("Holy Magic"), "Cure Poison", 2));
    character.insert::<Ability>(Key::new(15_usize), Ability::new(AbilityClass::from("Holy Magic"), "Refresh", 1));
    character.insert::<Ability>(Key::new(16_usize), Ability::new(AbilityClass::from("Holy Magic"), "Blessing", 1));
    character.insert::<Ability>(Key::new(17_usize), Ability::new(AbilityClass::from("Holy Magic"), "Powered", 1));
    character.insert::<Ability>(Key::new(18_usize), Ability::new(AbilityClass::from("Holy Magic"), "Haste", 5));
    character.insert::<Ability>(Key::new(19_usize), Ability::new(AbilityClass::from("Holy Magic"), "Protection", 10));
    character.insert::<Ability>(Key::new(20_usize), Ability::new(AbilityClass::from("Holy Magic"), "Resistance", 10));
    character.insert::<Ability>(Key::new(21_usize), Ability::new(AbilityClass::from("Holy Magic"), "Versatile Entertainer", 0));
    character.insert::<Ability>(Key::new(22_usize), Ability::new(AbilityClass::from("Holy Magic"), "Eyes of Providence", 0));
    character.insert::<Ability>(Key::new(23_usize), Ability::new(AbilityClass::from("Holy Magic"), "Reflect", 30));
    character.insert::<Ability>(Key::new(24_usize), Ability::new(AbilityClass::from("Holy Magic"), "Force Fire", 30));
    character.insert::<Ability>(Key::new(25_usize), Ability::new(AbilityClass::from("Holy Magic"), "Magic Seal", 50));

    character
}

#[rustfmt::skip]
fn get_character_map_darkness() -> HeterogeneousHashMap<usize> {
    let mut character = HeterogeneousHashMap::new();
    character.insert::<CharacterName>(Key::new(1_usize), CharacterName::from("Darkness"));
    character.insert::<CharacterName>(Key::new(2_usize), CharacterName::from("Lalatina Ford Dustiness"));
    character.insert::<Age>(Key::new(1_usize), Age::from(18));
    character.insert::<Race>(Key::new(1_usize), Race::from("Human"));
    character.insert::<Class>(Key::new(1_usize), Class::from("Crusader"));
    character.insert::<Job>(Key::new(1_usize), Job::from("Noble"));
    character.insert::<Status>(Key::new(1_usize), Status::from("Alive"));
    character.insert::<Description>(Key::new(1_usize), Description::from("\
        A noble crusader who intercepts every blow with unwavering resolve. None of her attacks ever hit their mark.\
    "));
    character.insert::<Stats>(Key::new(1_usize), Stats {
        strength: 22,
        dexterity: 4,
        agility: 25,
        intelligence: 10,
        perception: 6,
        mind: 25,
        luck: 10,
    });
    character.insert::<HitPoints>(Key::new(1_usize), HitPoints(150));
    character.insert::<MagicPoints>(Key::new(1_usize), MagicPoints(0));
    character.insert::<Chuunibyou>(Key::new(1_usize), Chuunibyou(0));
    character.insert::<Equipment>(Key::new(1_usize), Equipment::from("Adamantite Armor"));
    character.insert::<Equipment>(Key::new(2_usize), Equipment::from("Long Sword"));
    character.insert::<Ability>(Key::new(1_usize), Ability::new(AbilityClass::from("Crusader"), "Physical Resistance", 0));
    character.insert::<Ability>(Key::new(2_usize), Ability::new(AbilityClass::from("Crusader"), "Magic Resistance", 0));
    character.insert::<Ability>(Key::new(3_usize), Ability::new(AbilityClass::from("Crusader"), "Debuff Resistance (All Types)", 0));
    character.insert::<Ability>(Key::new(4_usize), Ability::new(AbilityClass::from("Crusader"), "Decoy", 0));
    character.insert::<Ability>(Key::new(5_usize), Ability::new(AbilityClass::from("Crusader"), "Side Slash", 0));

    character
}

#[rustfmt::skip]
fn get_character_map_yunyun() -> HeterogeneousHashMap<usize> {
    let mut character = HeterogeneousHashMap::new();
    character.insert::<CharacterName>(Key::new(1_usize), CharacterName::from("Yunyun"));
    character.insert::<Age>(Key::new(1_usize), Age::from(14));
    character.insert::<Race>(Key::new(1_usize), Race::from("Human"));
    character.insert::<Race>(Key::new(2_usize), Race::from("Crimson Magic Clan"));
    character.insert::<Class>(Key::new(1_usize), Class::from("Arch Wizard"));
    character.insert::<Status>(Key::new(1_usize), Status::from("Alive"));
    character.insert::<Description>(Key::new(1_usize), Description::from("\
        Crimson Demon honor student. Megumin’s arch rival (not that anyone else is competing). \
        Would really like it if someone talked to her.\
    "));
    character.insert::<Stats>(Key::new(1_usize), Stats {
        strength: 10,
        dexterity: 12,
        agility: 12,
        intelligence: 24,
        perception: 18,
        mind: 22,
        luck: 12,
    });
    character.insert::<HitPoints>(Key::new(1_usize), HitPoints(30));
    character.insert::<MagicPoints>(Key::new(1_usize), MagicPoints(400));
    character.insert::<Chuunibyou>(Key::new(1_usize), Chuunibyou(1));
    character.insert::<Equipment>(Key::new(1_usize), Equipment::from("Short Sword"));
    character.insert::<Equipment>(Key::new(2_usize), Equipment::from("Magic Rod"));
    character.insert::<InventoryItem>(Key::new(1_usize), InventoryItem::new("Magic Canceller Scroll", 1));
    character.insert::<InventoryItem>(Key::new(2_usize), InventoryItem::new("Manatites", 3));
    character.insert::<InventoryItem>(Key::new(3_usize), InventoryItem::new("Paralyze Booster Potion", 3));
    character.insert::<InventoryItem>(Key::new(4_usize), InventoryItem::new("Yunyun's Spellbook", 1));
    character.insert::<InventoryItem>(Key::new(5_usize), InventoryItem::new("Coins", 1000));
    character.insert::<Ability>(Key::new(1_usize),  Ability::new(AbilityClass::from("Wizard"), "Lightning", 10));
    character.insert::<Ability>(Key::new(2_usize),  Ability::new(AbilityClass::from("Wizard"), "Fireball", 10));
    character.insert::<Ability>(Key::new(3_usize),  Ability::new(AbilityClass::from("Wizard"), "Blade Of Wind", 10));
    character.insert::<Ability>(Key::new(4_usize),  Ability::new(AbilityClass::from("Wizard"), "Freeze Gust", 10));
    character.insert::<Ability>(Key::new(5_usize),  Ability::new(AbilityClass::from("Wizard"), "Sleep", 10));
    character.insert::<Ability>(Key::new(6_usize),  Ability::new(AbilityClass::from("Wizard"), "Unlock", 5));
    character.insert::<Ability>(Key::new(7_usize),  Ability::new(AbilityClass::from("Wizard"), "Flash", 8));
    character.insert::<Ability>(Key::new(8_usize),  Ability::new(AbilityClass::from("Wizard"), "Paralyze", 10));
    character.insert::<Ability>(Key::new(9_usize),  Ability::new(AbilityClass::from("Wizard"), "Teleport", 20));
    character.insert::<Ability>(Key::new(10_usize), Ability::new(AbilityClass::from("Wizard"), "Enemy Search", 10));
    character.insert::<Ability>(Key::new(11_usize), Ability::new(AbilityClass::from("Wizard"), "Light Of Saber", 20));
    character.insert::<Ability>(Key::new(12_usize), Ability::new(AbilityClass::from("Wizard"), "Lightning Strike", 20));
    character.insert::<Ability>(Key::new(13_usize), Ability::new(AbilityClass::from("Wizard"), "Energy Ignition", 20));
    character.insert::<Ability>(Key::new(14_usize), Ability::new(AbilityClass::from("Wizard"), "Bottomless Swamp", 20));
    character.insert::<Ability>(Key::new(15_usize), Ability::new(AbilityClass::from("Wizard"), "Cursed Lightning", 20));
    character.insert::<Ability>(Key::new(16_usize), Ability::new(AbilityClass::from("Wizard"), "Cursed Crystal Prison", 20));
    character.insert::<Ability>(Key::new(17_usize), Ability::new(AbilityClass::from("Wizard"), "Inferno", 20));
    character.insert::<Ability>(Key::new(18_usize), Ability::new(AbilityClass::from("Wizard"), "Tornado", 20));
    character.insert::<Ability>(Key::new(19_usize), Ability::new(AbilityClass::from("Wizard"), "Silent", 20));
    character.insert::<Ability>(Key::new(20_usize), Ability::new(AbilityClass::from("Wizard"), "Light Of Reflection", 20));
    character.insert::<Ability>(Key::new(21_usize), Ability::new(AbilityClass::from("Wizard"), "Control Of Weather", 30));

    character
}

#[rustfmt::skip]
fn get_character_map_wiz() -> HeterogeneousHashMap<usize> {
    let mut character = HeterogeneousHashMap::new();
    character.insert::<CharacterName>(Key::new(1_usize), CharacterName::from("Wiz"));
    character.insert::<CharacterName>(Key::new(2_usize), CharacterName::from("Ice Witch"));
    character.insert::<CharacterName>(Key::new(3_usize), CharacterName::from("Queen Of The Undead"));
    character.insert::<Age>(Key::new(1_usize), Age::from(20_u32));
    character.insert::<Race>(Key::new(1_usize), Race::from("Lich"));
    character.insert::<Race>(Key::new(2_usize), Race::from("Human"));
    character.insert::<Class>(Key::new(1_usize), Class::from("Arch Wizard"));
    character.insert::<Job>(Key::new(1_usize), Job::from("Devil King's General"));
    character.insert::<Job>(Key::new(2_usize), Job::from("Shopkeeper"));
    character.insert::<Status>(Key::new(1_usize), Status::from("Undead"));
    character.insert::<Description>(Key::new(1_usize), Description::from("\
        Benevolent lich, retired adventurer, and former Devil King's general. Now fighting her greatest battle: running a small business.\
    "));
    character.insert::<Stats>(Key::new(1_usize), Stats {
        strength: 10,
        dexterity: 10,
        agility: 10,
        intelligence: 29,
        perception: 12,
        mind: 25,
        luck: 8,
    });
    character.insert::<HitPoints>(Key::new(1_usize), HitPoints(60));
    character.insert::<MagicPoints>(Key::new(1_usize), MagicPoints(700));
    character.insert::<Chuunibyou>(Key::new(1_usize), Chuunibyou(0));
    character.insert::<Equipment>(Key::new(1_usize), Equipment::from("Rosary"));
    character.insert::<InventoryItem>(Key::new(1_usize), InventoryItem::new("Forced Teleport Scroll", 1));
    character.insert::<InventoryItem>(Key::new(2_usize), InventoryItem::new("Barrier Tool", 1));
    character.insert::<InventoryItem>(Key::new(3_usize), InventoryItem::new("Forbidden Crystal", 1));
    character.insert::<Ability>(Key::new(1_usize),  Ability::new(AbilityClass::from("Undead"), "Drain Touch", 3));
    character.insert::<Ability>(Key::new(2_usize),  Ability::new(AbilityClass::from("Undead"), "Hand Of The Immortal King", 10));
    character.insert::<Ability>(Key::new(3_usize),  Ability::new(AbilityClass::from("Undead"), "Physical Resistance", 0));
    character.insert::<Ability>(Key::new(4_usize),  Ability::new(AbilityClass::from("Undead"), "Magic Resistance", 0));
    character.insert::<Ability>(Key::new(5_usize),  Ability::new(AbilityClass::from("Undead"), "Cursed Petrification", 10));
    character.insert::<Ability>(Key::new(6_usize),  Ability::new(AbilityClass::from("Undead"), "Cursed Necromancy", 10));
    character.insert::<Ability>(Key::new(7_usize),  Ability::new(AbilityClass::from("Undead"), "Night Vision", 10));
    character.insert::<Ability>(Key::new(8_usize),  Ability::new(AbilityClass::from("Wizard"), "Anti-Devil Curses", 4));
    character.insert::<Ability>(Key::new(9_usize),  Ability::new(AbilityClass::from("Wizard"), "Freeze", 2));
    character.insert::<Ability>(Key::new(10_usize), Ability::new(AbilityClass::from("Wizard"), "Freeze Gust", 12));
    character.insert::<Ability>(Key::new(11_usize), Ability::new(AbilityClass::from("Wizard"), "Sleep", 10));
    character.insert::<Ability>(Key::new(12_usize), Ability::new(AbilityClass::from("Wizard"), "Crystal Prison", 10));
    character.insert::<Ability>(Key::new(13_usize), Ability::new(AbilityClass::from("Wizard"), "Cursed Crystal Prison", 10));
    character.insert::<Ability>(Key::new(14_usize), Ability::new(AbilityClass::from("Wizard"), "Bottomless Swamp", 10));
    character.insert::<Ability>(Key::new(15_usize), Ability::new(AbilityClass::from("Wizard"), "Cursed Lightning", 10));
    character.insert::<Ability>(Key::new(16_usize), Ability::new(AbilityClass::from("Wizard"), "Inferno", 10));
    character.insert::<Ability>(Key::new(17_usize), Ability::new(AbilityClass::from("Wizard"), "Light Of Saber", 10));
    character.insert::<Ability>(Key::new(18_usize), Ability::new(AbilityClass::from("Wizard"), "Lightning Strike", 10));
    character.insert::<Ability>(Key::new(19_usize), Ability::new(AbilityClass::from("Wizard"), "Create Earth Golem", 10));
    character.insert::<Ability>(Key::new(20_usize), Ability::new(AbilityClass::from("Wizard"), "Create Earth Wall", 10));
    character.insert::<Ability>(Key::new(21_usize), Ability::new(AbilityClass::from("Wizard"), "Enemy Search", 10));
    character.insert::<Ability>(Key::new(22_usize), Ability::new(AbilityClass::from("Wizard"), "Trap Search", 10));
    character.insert::<Ability>(Key::new(23_usize), Ability::new(AbilityClass::from("Wizard"), "Teleport", 10));
    character.insert::<Ability>(Key::new(24_usize), Ability::new(AbilityClass::from("Wizard"), "Random Teleport", 10));
    character.insert::<Ability>(Key::new(25_usize), Ability::new(AbilityClass::from("Wizard"), "Explosion", 100));

    character
}

#[rustfmt::skip]
fn get_character_map_chris() -> HeterogeneousHashMap<usize> {
    let mut character = HeterogeneousHashMap::new();
    character.insert::<CharacterName>(Key::new(1_usize), CharacterName::from("Chris"));
    character.insert::<CharacterName>(Key::new(2_usize), CharacterName::from("Noble Thief"));
    character.insert::<PlayerName>(Key::new(1_usize), PlayerName::from("Eris"));
    character.insert::<Age>(Key::new(1_usize), Age::from(15));
    character.insert::<Age>(Key::new(2_usize), Age::from(u32::MAX));
    character.insert::<Race>(Key::new(1_usize), Race::from("Human"));
    character.insert::<Race>(Key::new(2_usize), Race::from("God"));
    character.insert::<Class>(Key::new(1_usize), Class::from("Thief"));
    character.insert::<Job>(Key::new(1_usize), Job::from("Goddess"));
    character.insert::<Job>(Key::new(2_usize), Job::from("Aqua's Janitor"));
    character.insert::<Job>(Key::new(3_usize), Job::from("Currently Cleaning Up Aqua's Messes. Send Help."));
    character.insert::<Status>(Key::new(1_usize), Status::from("Alive"));
    character.insert::<Description>(Key::new(1_usize), Description::from("\
        She’s got a strong sense of justice and an even stronger sense of when a locked chest needs opening.\
    "));
    character.insert::<Stats>(Key::new(1_usize), Stats {
        strength: 11,
        dexterity: 25,
        agility: 25,
        intelligence: 25,
        perception: 25,
        mind: 25,
        luck: u32::MAX,
    });
    character.insert::<HitPoints>(Key::new(1_usize), HitPoints(100));
    character.insert::<MagicPoints>(Key::new(1_usize), MagicPoints(0));
    character.insert::<Chuunibyou>(Key::new(1_usize), Chuunibyou(0));
    character.insert::<Equipment>(Key::new(1_usize), Equipment::from("Magic Dagger"));
    character.insert::<InventoryItem>(Key::new(1_usize), InventoryItem::new("Barrier Breaker", 1));
    character.insert::<InventoryItem>(Key::new(2_usize), InventoryItem::new("Rock", 8));
    character.insert::<Ability>(Key::new(1_usize),  Ability::new(AbilityClass::from("Thief"), "Steal", 1));
    character.insert::<Ability>(Key::new(2_usize),  Ability::new(AbilityClass::from("Thief"), "Lurk", 1));
    character.insert::<Ability>(Key::new(3_usize),  Ability::new(AbilityClass::from("Thief"), "Enemy Detection", 1));
    character.insert::<Ability>(Key::new(4_usize),  Ability::new(AbilityClass::from("Thief"), "Trap Detection", 1));
    character.insert::<Ability>(Key::new(5_usize),  Ability::new(AbilityClass::from("Thief"), "Disarm Trap", 1));
    character.insert::<Ability>(Key::new(6_usize),  Ability::new(AbilityClass::from("Thief"), "Flee", 1));
    character.insert::<Ability>(Key::new(7_usize),  Ability::new(AbilityClass::from("Thief"), "Bind", 1));
    character.insert::<Ability>(Key::new(8_usize),  Ability::new(AbilityClass::from("Thief"), "Skill Bind", 1));
    character.insert::<Ability>(Key::new(9_usize),  Ability::new(AbilityClass::from("Thief"), "Wire Trap", 1));
    character.insert::<Ability>(Key::new(10_usize), Ability::new(AbilityClass::from("Thief"), "Detect Treasure", 1));
    character.insert::<Ability>(Key::new(11_usize), Ability::new(AbilityClass::from("Thief"), "Lockpick", 1));

    character
}

#[rustfmt::skip]
fn get_character_map_mitsurugi() -> HeterogeneousHashMap<usize> {
    let mut character = HeterogeneousHashMap::new();
    character.insert::<CharacterName>(Key::new(1_usize), CharacterName::from("Kyouya Mitsurugi"));
    character.insert::<CharacterName>(Key::new(2_usize), CharacterName::from("Cursed Sword Hero"));
    character.insert::<CharacterName>(Key::new(3_usize), CharacterName::from("Magic Sword Guy"));
    character.insert::<PlayerName>(Key::new(1_usize), PlayerName::from("Kyouya Mitsurugi"));
    character.insert::<Age>(Key::new(1_usize), Age::from(17));
    character.insert::<Race>(Key::new(1_usize), Race::from("Human"));
    character.insert::<Class>(Key::new(1_usize), Class::from("Swordmaster"));
    character.insert::<Job>(Key::new(1_usize), Job::from("Isekai Protagonist"));
    character.insert::<Status>(Key::new(1_usize), Status::from("Alive"));
    character.insert::<Description>(Key::new(1_usize), Description::from("\
        Textbook case of main character syndrome. He thinks he is the real deal Isekai hero. \
        Constantly played like a side quest by Kazuma.\
    "));
    character.insert::<Stats>(Key::new(1_usize), Stats {
        strength: 25,
        dexterity: 20,
        agility: 20,
        intelligence: 10,
        perception: 4,
        mind: 15,
        luck: 3,
    });
    character.insert::<HitPoints>(Key::new(1_usize), HitPoints(200));
    character.insert::<MagicPoints>(Key::new(1_usize), MagicPoints(0));
    character.insert::<Chuunibyou>(Key::new(1_usize), Chuunibyou(100));
    character.insert::<Equipment>(Key::new(1_usize), Equipment::from("Cursed Sword Gram"));
    character.insert::<Ability>(Key::new(1_usize), Ability::new(AbilityClass::from("Swordmaster"), "Rune Of Saber", 10));

    character
}

fn get_character_map() -> HomogeneousHashMap<String, HeterogeneousHashMap<usize>> {
    let kazuma = get_character_map_kazuma();
    let megumin = get_character_map_megumin();
    let aqua = get_character_map_aqua();
    let darkness = get_character_map_darkness();
    let yunyun = get_character_map_yunyun();
    let wiz = get_character_map_wiz();
    let chris = get_character_map_chris();
    let mitsurugi = get_character_map_mitsurugi();

    let mut characters = HomogeneousHashMap::new();
    characters.insert(Key::new(String::from("Kazuma")), kazuma);
    characters.insert(Key::new(String::from("Megumin")), megumin);
    characters.insert(Key::new(String::from("Aqua")), aqua);
    characters.insert(Key::new(String::from("Darkness")), darkness);
    characters.insert(Key::new(String::from("Yunyun")), yunyun);
    characters.insert(Key::new(String::from("Wiz")), wiz);
    characters.insert(Key::new(String::from("Chris")), chris);
    characters.insert(Key::new(String::from("Mitsurugi")), mitsurugi);

    characters
}

fn run_test_heterogeneous_hash_map_accessors<T, I1, I2>(
    het_map: &HeterogeneousHashMap<usize>,
    expected_map: &hash_map::HashMap<Key<usize, T>, T>,
    pre_range: I1, post_range: I2
)
where
    T: any::Any + fmt::Debug + Clone + PartialEq,
    I1: IntoIterator<Item = Key<usize, T>>,
    I2: IntoIterator<Item = Key<usize, T>>,
{
    for key in pre_range.into_iter() {
        assert_eq!(het_map.get::<T, _>(&key), None);
    }

    for (key, value) in expected_map.iter() {
        let expected = Some(value.clone());
        let result = het_map.get::<T, _>(key).cloned();

        assert_eq!(result, expected);
    }

    for key in post_range.into_iter() {
        assert_eq!(het_map.get::<T, _>(&key), None);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_character_name() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<CharacterName>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(CharacterName::from("Kazuma Satou"))),
        (String::from("Megumin"),   Some(CharacterName::from("Megumin"))),
        (String::from("Aqua"),      Some(CharacterName::from("Aqua"))),
        (String::from("Darkness"),  Some(CharacterName::from("Darkness"))),
        (String::from("Yunyun"),    Some(CharacterName::from("Yunyun"))),
        (String::from("Wiz"),       Some(CharacterName::from("Wiz"))),
        (String::from("Chris"),     Some(CharacterName::from("Chris"))),
        (String::from("Mitsurugi"), Some(CharacterName::from("Kyouya Mitsurugi"))),
    ]);
    let key = Key::new(1_usize);

    for (name, character_name) in expected_map.iter() {
        let expected = character_name.clone();
        let result = characters
            .get_unchecked(name)
            .get::<CharacterName, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_character_name_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(2)),
        (String::from("Megumin"),   Some(3)),
        (String::from("Aqua"),      Some(3)),
        (String::from("Darkness"),  Some(2)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(3)),
        (String::from("Chris"),     Some(2)),
        (String::from("Mitsurugi"), Some(3)),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<CharacterName>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<CharacterName>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<CharacterName>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<CharacterName>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<CharacterName>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<CharacterName>()),
        (String::from("Chris"),     get_character_map_chris().len::<CharacterName>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<CharacterName>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_character_name_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(2)),
        (String::from("Megumin"),   Some(3)),
        (String::from("Aqua"),      Some(3)),
        (String::from("Darkness"),  Some(2)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(3)),
        (String::from("Chris"),     Some(2)),
        (String::from("Mitsurugi"), Some(3)),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<CharacterName>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_character_name1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, CharacterName>, CharacterName> = hash_map::HashMap::from([
        (Key::new(1_usize), CharacterName::from("Kazuma Satou")),
        (Key::new(2_usize), CharacterName::from("Kazutrash")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (3_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_character_name2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, CharacterName>, CharacterName> = hash_map::HashMap::from([
        (Key::new(1_usize), CharacterName::from("Megumin")),
        (Key::new(2_usize), CharacterName::from("Explosion Maniac")),
        (Key::new(3_usize), CharacterName::from("Crazy Explosion Girl")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (4_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_character_name3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, CharacterName>, CharacterName> = hash_map::HashMap::from([
        (Key::new(1_usize), CharacterName::from("Aqua")),
        (Key::new(2_usize), CharacterName::from("Lady Aqua")),
        (Key::new(3_usize), CharacterName::from("Goddess Of Party Tricks")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (4_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_character_name4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, CharacterName>, CharacterName> = hash_map::HashMap::from([
        (Key::new(1_usize), CharacterName::from("Darkness")),
        (Key::new(2_usize), CharacterName::from("Lalatina Ford Dustiness")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (3_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_character_name5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, CharacterName>, CharacterName> = hash_map::HashMap::from([
        (Key::new(1_usize), CharacterName::from("Yunyun")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_character_name6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, CharacterName>, CharacterName> = hash_map::HashMap::from([
        (Key::new(1_usize), CharacterName::from("Wiz")),
        (Key::new(2_usize), CharacterName::from("Ice Witch")),
        (Key::new(3_usize), CharacterName::from("Queen Of The Undead")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (4_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_character_name7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, CharacterName>, CharacterName> = hash_map::HashMap::from([
        (Key::new(1_usize), CharacterName::from("Chris")),
        (Key::new(2_usize), CharacterName::from("Noble Thief")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (3_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_character_name8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, CharacterName>, CharacterName> = hash_map::HashMap::from([
        (Key::new(1_usize), CharacterName::from("Kyouya Mitsurugi")),
        (Key::new(2_usize), CharacterName::from("Cursed Sword Hero")),
        (Key::new(3_usize), CharacterName::from("Magic Sword Guy")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (4_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_player_name() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<PlayerName>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(PlayerName::from("I'm Kazuma"))),
        (String::from("Megumin"),   None),
        (String::from("Aqua"),      Some(PlayerName::from("Aqua"))),
        (String::from("Darkness"),  None),
        (String::from("Yunyun"),    None),
        (String::from("Wiz"),       None),
        (String::from("Chris"),     Some(PlayerName::from("Eris"))),
        (String::from("Mitsurugi"), Some(PlayerName::from("Kyouya Mitsurugi"))),
    ]);
    let key = Key::new(1_usize);

    for (name, player_name) in expected_map.iter() {
        let expected = player_name.clone();
        let result = characters
            .get_unchecked(name)
            .get::<PlayerName, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_player_name_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(3)),
        (String::from("Megumin"),   None),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  None),
        (String::from("Yunyun"),    None),
        (String::from("Wiz"),       None),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<PlayerName>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<PlayerName>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<PlayerName>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<PlayerName>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<PlayerName>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<PlayerName>()),
        (String::from("Chris"),     get_character_map_chris().len::<PlayerName>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<PlayerName>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_player_name_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(3)),
        (String::from("Megumin"),   None),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  None),
        (String::from("Yunyun"),    None),
        (String::from("Wiz"),       None),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<PlayerName>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_player_name1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, PlayerName>, PlayerName> = hash_map::HashMap::from([
        (Key::new(1_usize), PlayerName::from("I'm Kazuma")),
        (Key::new(2_usize), PlayerName::from("That's My Name")),
        (Key::new(3_usize), PlayerName::from("Kazuma Satou")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (4_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_player_name2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, PlayerName>, PlayerName> = hash_map::HashMap::new();

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..0_usize).map(Key::new), (0_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_player_name3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, PlayerName>, PlayerName> = hash_map::HashMap::from([
        (Key::new(1_usize), PlayerName::from("Aqua")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_player_name4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, PlayerName>, PlayerName> = hash_map::HashMap::new();

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..0_usize).map(Key::new), (0_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_player_name5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, PlayerName>, PlayerName> = hash_map::HashMap::new();

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..0_usize).map(Key::new), (0_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_player_name6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, PlayerName>, PlayerName> = hash_map::HashMap::new();

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..0_usize).map(Key::new), (0_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_player_name7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, PlayerName>, PlayerName> = hash_map::HashMap::from([
        (Key::new(1_usize), PlayerName::from("Eris")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_player_name8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, PlayerName>, PlayerName> = hash_map::HashMap::from([
        (Key::new(1_usize), PlayerName::from("Kyouya Mitsurugi")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_age() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<Age>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(Age::from(17))),
        (String::from("Megumin"),   Some(Age::from(14))),
        (String::from("Aqua"),      Some(Age::from(16))),
        (String::from("Darkness"),  Some(Age::from(18))),
        (String::from("Yunyun"),    Some(Age::from(14))),
        (String::from("Wiz"),       Some(Age::from(20_u32))),
        (String::from("Chris"),     Some(Age::from(15))),
        (String::from("Mitsurugi"), Some(Age::from(17))),
    ]);
    let key = Key::new(1_usize);

    for (name, age) in expected_map.iter() {
        let expected = age.clone();
        let result = characters
            .get_unchecked(name)
            .get::<Age, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_age_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(2)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(2)),
        (String::from("Mitsurugi"), Some(1)),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<Age>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<Age>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<Age>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<Age>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<Age>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<Age>()),
        (String::from("Chris"),     get_character_map_chris().len::<Age>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<Age>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_age_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(2)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(2)),
        (String::from("Mitsurugi"), Some(1)),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<Age>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_age1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(17)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_age2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(14)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_age3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(16)),
        (Key::new(2_usize), Age::from(u32::MAX)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (3_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_age4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(18)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_age5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(14)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_age6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(20_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_age7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(15)),
        (Key::new(2_usize), Age::from(u32::MAX))
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (3_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_age8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(17)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_race() {
    let mut characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<Race>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(Race::from("Human"))),
        (String::from("Megumin"),   Some(Race::from("Human"))),
        (String::from("Aqua"),      Some(Race::from("God"))),
        (String::from("Darkness"),  Some(Race::from("Human"))),
        (String::from("Yunyun"),    Some(Race::from("Human"))),
        (String::from("Wiz"),       Some(Race::from("Lich"))),
        (String::from("Chris"),     Some(Race::from("Human"))),
        (String::from("Mitsurugi"), Some(Race::from("Human"))),
    ]);
    let key = Key::new(1_usize);

    for (name, race) in expected_map.iter() {
        let expected = race.clone();
        let result = characters
            .get_unchecked(name)
            .get::<Race, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_race_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(2)),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(2)),
        (String::from("Wiz"),       Some(2)),
        (String::from("Chris"),     Some(2)),
        (String::from("Mitsurugi"), Some(1)),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<Race>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<Race>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<Race>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<Race>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<Race>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<Race>()),
        (String::from("Chris"),     get_character_map_chris().len::<Race>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<Race>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_race_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(2)),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(2)),
        (String::from("Wiz"),       Some(2)),
        (String::from("Chris"),     Some(2)),
        (String::from("Mitsurugi"), Some(1)),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<Race>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_race1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("Human")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_race2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("Human")),
        (Key::new(2_usize), Race::from("Crimson Magic Clan")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (3_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_race3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("God")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_race4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("Human")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_race5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("Human")),
        (Key::new(2_usize), Race::from("Crimson Magic Clan")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (3_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_race6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("Lich")),
        (Key::new(2_usize), Race::from("Human")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (3_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_race7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("Human")),
        (Key::new(2_usize), Race::from("God"))
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (3_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_race8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("Human")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_class() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<Class>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(Class::from("Adventurer"))),
        (String::from("Megumin"),   Some(Class::from("Arch Wizard"))),
        (String::from("Aqua"),      Some(Class::from("Arch Priest"))),
        (String::from("Darkness"),  Some(Class::from("Crusader"))),
        (String::from("Yunyun"),    Some(Class::from("Arch Wizard"))),
        (String::from("Wiz"),       Some(Class::from("Arch Wizard"))),
        (String::from("Chris"),     Some(Class::from("Thief"))),
        (String::from("Mitsurugi"), Some(Class::from("Swordmaster"))),
    ]);
    let key = Key::new(1_usize);

    for (name, class) in expected_map.iter() {
        let expected = class.clone();
        let result = characters
            .get_unchecked(name)
            .get::<Class, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_class_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<Class>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<Class>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<Class>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<Class>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<Class>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<Class>()),
        (String::from("Chris"),     get_character_map_chris().len::<Class>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<Class>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_class_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<Class>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_class1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Adventurer")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_class2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Arch Wizard")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_class3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Arch Priest")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_class4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Crusader")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_class5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Arch Wizard")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_class6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Arch Wizard")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_class7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Thief")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_class8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Swordmaster")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_job() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<Job>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(Job::from("Jack of All Trades, Master Of Dumb Luck"))),
        (String::from("Megumin"),   None),
        (String::from("Aqua"),      Some(Job::from("Self-Proclaimed Goddess"))),
        (String::from("Darkness"),  Some(Job::from("Noble"))),
        (String::from("Yunyun"),    None),
        (String::from("Wiz"),       Some(Job::from("Devil King's General"))),
        (String::from("Chris"),     Some(Job::from("Goddess"))),
        (String::from("Mitsurugi"), Some(Job::from("Isekai Protagonist"))),
    ]);
    let key = Key::new(1_usize);

    for (name, job) in expected_map.iter() {
        let expected = job.clone();
        let result = characters
            .get_unchecked(name)
            .get::<Job, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_job_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(3)),
        (String::from("Megumin"),   None),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    None),
        (String::from("Wiz"),       Some(2)),
        (String::from("Chris"),     Some(3)),
        (String::from("Mitsurugi"), Some(1)),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<Job>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<Job>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<Job>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<Job>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<Job>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<Job>()),
        (String::from("Chris"),     get_character_map_chris().len::<Job>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<Job>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_job_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(3)),
        (String::from("Megumin"),   None),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    None),
        (String::from("Wiz"),       Some(2)),
        (String::from("Chris"),     Some(3)),
        (String::from("Mitsurugi"), Some(1)),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<Job>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_job1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Job>, Job> = hash_map::HashMap::from([
        (Key::new(1_usize), Job::from("Jack of All Trades, Master Of Dumb Luck")),
        (Key::new(2_usize), Job::from("Definitely Not A Harem Protagonist")),
        (Key::new(3_usize), Job::from("Strategic Coward")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (4_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_job2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, Job>, Job> = hash_map::HashMap::new();

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..0_usize).map(Key::new), (0_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_job3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Job>, Job> = hash_map::HashMap::from([
        (Key::new(1_usize), Job::from("Self-Proclaimed Goddess")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_job4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Job>, Job> = hash_map::HashMap::from([
        (Key::new(1_usize), Job::from("Noble")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_job5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, Job>, Job> = hash_map::HashMap::new();

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..0_usize).map(Key::new), (0_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_job6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Job>, Job> = hash_map::HashMap::from([
        (Key::new(1_usize), Job::from("Devil King's General")),
        (Key::new(2_usize), Job::from("Shopkeeper")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (3_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_job7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, Job>, Job> = hash_map::HashMap::from([
        (Key::new(1_usize), Job::from("Goddess")),
        (Key::new(2_usize), Job::from("Aqua's Janitor")),
        (Key::new(3_usize), Job::from("Currently Cleaning Up Aqua's Messes. Send Help.")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (4_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_job8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Job>, Job> = hash_map::HashMap::from([
        (Key::new(1_usize), Job::from("Isekai Protagonist")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_status() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<Status>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(Status::from("Alive"))),
        (String::from("Megumin"),   Some(Status::from("Alive"))),
        (String::from("Aqua"),      Some(Status::from("Alive"))),
        (String::from("Darkness"),  Some(Status::from("Alive"))),
        (String::from("Yunyun"),    Some(Status::from("Alive"))),
        (String::from("Wiz"),       Some(Status::from("Undead"))),
        (String::from("Chris"),     Some(Status::from("Alive"))),
        (String::from("Mitsurugi"), Some(Status::from("Alive"))),
    ]);
    let key = Key::new(1_usize);

    for (name, status) in expected_map.iter() {
        let expected = status.clone();
        let result = characters
            .get_unchecked(name)
            .get::<Status, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_status_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<Status>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<Status>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<Status>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<Status>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<Status>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<Status>()),
        (String::from("Chris"),     get_character_map_chris().len::<Status>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<Status>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_status_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<Status>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_status1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Alive")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_status2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Alive")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_status3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Alive")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_status4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Alive")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_status5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Alive")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_status6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Undead")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_status7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Alive")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_status8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Alive")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_description() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<Description>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(Description::from("\
            Slovenly shut-in NEET with questionable morals and a surprisingly sharp wit.\
        "))),
        (String::from("Megumin"),   Some(Description::from("\
            I am MEGUMIN! The greatest wizard of the CRIMSON DEMON CLAN! The user of EXPLOSION magic!\
        "))),
        (String::from("Aqua"),      Some(Description::from("\
            Useless water goddess of the Axis church. Somehow, her followers are even crazier than she is.\
        "))),
        (String::from("Darkness"),  Some(Description::from("\
            A noble crusader who intercepts every blow with unwavering resolve. \
            None of her attacks ever hit their mark.\
        "))),
        (String::from("Yunyun"),    Some(Description::from("\
            Crimson Demon honor student. Megumin’s arch rival (not that anyone else is competing). \
            Would really like it if someone talked to her.\
        "))),
        (String::from("Wiz"),       Some(Description::from("\
            Benevolent lich, retired adventurer, and former Devil King's general. Now fighting her \
            greatest battle: running a small business.\
        "))),
        (String::from("Chris"),     Some(Description::from("\
            She’s got a strong sense of justice and an even stronger sense of when a locked chest needs opening.\
        "))),
        (String::from("Mitsurugi"), Some(Description::from("\
            Textbook case of main character syndrome. He thinks he is the real deal Isekai hero. \
            Constantly played like a side quest by Kazuma.\
        "))),
    ]);
    let key = Key::new(1_usize);

    for (name, description) in expected_map.iter() {
        let expected = description.clone();
        let result = characters
            .get_unchecked(name)
            .get::<Description, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_description_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(2)),
        (String::from("Aqua"),      Some(5)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<Description>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<Description>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<Description>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<Description>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<Description>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<Description>()),
        (String::from("Chris"),     get_character_map_chris().len::<Description>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<Description>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_description_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(2)),
        (String::from("Aqua"),      Some(5)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<Description>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_description1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Description>, Description> = hash_map::HashMap::from([
        (Key::new(1_usize), Description::from("Slovenly shut-in NEET with questionable morals and a surprisingly sharp wit.")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_description2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, Description>, Description> = hash_map::HashMap::from([
        (Key::new(1_usize), Description::from("I am MEGUMIN! The greatest wizard of the CRIMSON DEMON CLAN! The user of EXPLOSION magic!")),
        (Key::new(2_usize), Description::from("NOTE (Luna): Adventurer refused to provide a standard description. She forced this one in all caps.")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (3_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_description3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Description>, Description> = hash_map::HashMap::from([
        (Key::new(1_usize), Description::from("Useless water goddess of the Axis church. Somehow, her followers are even crazier than she is.")),
        (Key::new(2_usize), Description::from("Self-proclaimed goddess who specializes in getting us into trouble and drinking all the party’s funds. Blessings included, probably.")),
        (Key::new(3_usize), Description::from("A self-proclaimed goddess notorious for causing disasters and drinking all the booze. Worship at your own risk.")),
        (Key::new(4_usize), Description::from("Patron deity of purification, renewal, and... occasional self-sabotage. Pray hard, avoid the frogs.")),
        (Key::new(5_usize), Description::from("\
            Revered water goddess of the Axis Church, renowned for her unparalleled purity and unwavering dedication to her followers' prosperity. \
            Her divine blessings ensure the flourishing of faith and the cleansing of corruption across the realm.\
        ")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (6_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_description4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Description>, Description> = hash_map::HashMap::from([
        (Key::new(1_usize), Description::from("\
            A noble crusader who intercepts every blow with unwavering resolve. None of her attacks ever hit their mark.\
        ")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_description5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, Description>, Description> = hash_map::HashMap::from([
        (Key::new(1_usize), Description::from("\
            Crimson Demon honor student. Megumin’s arch rival (not that anyone else is competing). \
            Would really like it if someone talked to her.\
        ")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_description6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Description>, Description> = hash_map::HashMap::from([
        (Key::new(1_usize), Description::from("\
            Benevolent lich, retired adventurer, and former Devil King's general. Now fighting her greatest battle: running a small business.\
        ")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_description7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, Description>, Description> = hash_map::HashMap::from([
        (Key::new(1_usize), Description::from("\
            She’s got a strong sense of justice and an even stronger sense of when a locked chest needs opening.\
        ")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_description8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Description>, Description> = hash_map::HashMap::from([
        (Key::new(1_usize), Description::from("\
            Textbook case of main character syndrome. He thinks he is the real deal Isekai hero. \
            Constantly played like a side quest by Kazuma.\
        ")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_stats() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<Stats>> = hash_map::HashMap::from([
        (String::from("Kazuma"), Some(Stats {
            strength: 12,
            dexterity: 14,
            agility: 13,
            intelligence: 18,
            perception: 14,
            mind: 10,
            luck: 99,
        })),
        (String::from("Megumin"), Some(Stats {
            strength: 14,
            dexterity: 10,
            agility: 10,
            intelligence: 25,
            perception: 14,
            mind: 24,
            luck: 10,
        })),
        (String::from("Aqua"), Some(Stats {
            strength: 18,
            dexterity: 11,
            agility: 14,
            intelligence: 14,
            perception: 9,
            mind: u32::MAX,
            luck: 1,
        })),
        (String::from("Darkness"), Some(Stats {
            strength: 22,
            dexterity: 4,
            agility: 25,
            intelligence: 10,
            perception: 6,
            mind: 25,
            luck: 10,
        })),
        (String::from("Yunyun"), Some(Stats {
            strength: 10,
            dexterity: 12,
            agility: 12,
            intelligence: 24,
            perception: 18,
            mind: 22,
            luck: 12,
        })),
        (String::from("Wiz"), Some(Stats {
            strength: 10,
            dexterity: 10,
            agility: 10,
            intelligence: 29,
            perception: 12,
            mind: 25,
            luck: 8,
        })),
        (String::from("Chris"), Some(Stats {
            strength: 11,
            dexterity: 25,
            agility: 25,
            intelligence: 25,
            perception: 25,
            mind: 25,
            luck: u32::MAX,
        })),
        (String::from("Mitsurugi"), Some(Stats {
            strength: 25,
            dexterity: 20,
            agility: 20,
            intelligence: 10,
            perception: 4,
            mind: 15,
            luck: 3,
        })),
    ]);
    let key = Key::new(1_usize);

    for (name, stats) in expected_map.iter() {
        let expected = stats.clone();
        let result = characters
            .get_unchecked(name)
            .get::<Stats, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_stats_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<Stats>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<Stats>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<Stats>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<Stats>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<Stats>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<Stats>()),
        (String::from("Chris"),     get_character_map_chris().len::<Stats>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<Stats>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_stats_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<Stats>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_stats1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Stats>, Stats> = hash_map::HashMap::from([
        (Key::new(1_usize), Stats {
            strength: 12,
            dexterity: 14,
            agility: 13,
            intelligence: 18,
            perception: 14,
            mind: 10,
            luck: 99,
        }),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_stats2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, Stats>, Stats> = hash_map::HashMap::from([
        (Key::new(1_usize), Stats {
            strength: 14,
            dexterity: 10,
            agility: 10,
            intelligence: 25,
            perception: 14,
            mind: 24,
            luck: 10,
        }),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_stats3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Stats>, Stats> = hash_map::HashMap::from([
        (Key::new(1_usize), Stats {
            strength: 18,
            dexterity: 11,
            agility: 14,
            intelligence: 14,
            perception: 9,
            mind: u32::MAX,
            luck: 1,
        }),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_stats4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Stats>, Stats> = hash_map::HashMap::from([
        (Key::new(1_usize), Stats {
            strength: 22,
            dexterity: 4,
            agility: 25,
            intelligence: 10,
            perception: 6,
            mind: 25,
            luck: 10,
        }),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_stats5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, Stats>, Stats> = hash_map::HashMap::from([
        (Key::new(1_usize), Stats {
            strength: 10,
            dexterity: 12,
            agility: 12,
            intelligence: 24,
            perception: 18,
            mind: 22,
            luck: 12,
        }),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_stats6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Stats>, Stats> = hash_map::HashMap::from([
        (Key::new(1_usize), Stats {
            strength: 10,
            dexterity: 10,
            agility: 10,
            intelligence: 29,
            perception: 12,
            mind: 25,
            luck: 8,
        }),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_stats7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, Stats>, Stats> = hash_map::HashMap::from([
        (Key::new(1_usize), Stats {
            strength: 11,
            dexterity: 25,
            agility: 25,
            intelligence: 25,
            perception: 25,
            mind: 25,
            luck: u32::MAX,
        }),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_stats8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Stats>, Stats> = hash_map::HashMap::from([
        (Key::new(1_usize), Stats {
            strength: 25,
            dexterity: 20,
            agility: 20,
            intelligence: 10,
            perception: 4,
            mind: 15,
            luck: 3,
        }),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_hit_points() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<HitPoints>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(HitPoints::from(40_u32))),
        (String::from("Megumin"),   Some(HitPoints::from(20_u32))),
        (String::from("Aqua"),      Some(HitPoints::from(60_u32))),
        (String::from("Darkness"),  Some(HitPoints::from(150))),
        (String::from("Yunyun"),    Some(HitPoints::from(30_u32))),
        (String::from("Wiz"),       Some(HitPoints::from(60_u32))),
        (String::from("Chris"),     Some(HitPoints::from(100_u32))),
        (String::from("Mitsurugi"), Some(HitPoints::from(200_u32))),
    ]);
    let key = Key::new(1_usize);

    for (name, hit_points) in expected_map.iter() {
        let expected = hit_points.clone();
        let result = characters
            .get_unchecked(name)
            .get::<HitPoints, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_hit_points_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<HitPoints>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<HitPoints>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<HitPoints>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<HitPoints>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<HitPoints>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<HitPoints>()),
        (String::from("Chris"),     get_character_map_chris().len::<HitPoints>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<HitPoints>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_hit_points_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<HitPoints>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_hit_points1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, HitPoints>, HitPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), HitPoints::from(40_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_hit_points2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, HitPoints>, HitPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), HitPoints::from(20_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_hit_points3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, HitPoints>, HitPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), HitPoints::from(60_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_hit_points4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, HitPoints>, HitPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), HitPoints::from(150)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_hit_points5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, HitPoints>, HitPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), HitPoints::from(30_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_hit_points6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, HitPoints>, HitPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), HitPoints::from(60_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_hit_points7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, HitPoints>, HitPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), HitPoints::from(100_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_hit_points8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, HitPoints>, HitPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), HitPoints::from(200_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_magic_points() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<MagicPoints>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(MagicPoints::from(20_u32))),
        (String::from("Megumin"),   Some(MagicPoints::from(999_u32))),
        (String::from("Aqua"),      Some(MagicPoints::from(u32::MAX))),
        (String::from("Darkness"),  Some(MagicPoints::from(0_u32))),
        (String::from("Yunyun"),    Some(MagicPoints::from(400_u32))),
        (String::from("Wiz"),       Some(MagicPoints::from(700))),
        (String::from("Chris"),     Some(MagicPoints::from(0_u32))),
        (String::from("Mitsurugi"), Some(MagicPoints::from(0_u32))),
    ]);
    let key = Key::new(1_usize);

    for (name, magic_points) in expected_map.iter() {
        let expected = magic_points.clone();
        let result = characters
            .get_unchecked(name)
            .get::<MagicPoints, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_magic_points_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<MagicPoints>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<MagicPoints>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<MagicPoints>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<MagicPoints>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<MagicPoints>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<MagicPoints>()),
        (String::from("Chris"),     get_character_map_chris().len::<MagicPoints>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<MagicPoints>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_magic_points_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<MagicPoints>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_magic_points1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, MagicPoints>, MagicPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), MagicPoints::from(20_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_magic_points2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, MagicPoints>, MagicPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), MagicPoints::from(999_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_magic_points3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, MagicPoints>, MagicPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), MagicPoints::from(u32::MAX)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_magic_points4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, MagicPoints>, MagicPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), MagicPoints::from(0_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_magic_points5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, MagicPoints>, MagicPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), MagicPoints::from(400_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_magic_points6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, MagicPoints>, MagicPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), MagicPoints::from(700)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_magic_points7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, MagicPoints>, MagicPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), MagicPoints::from(0_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_magic_points8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, MagicPoints>, MagicPoints> = hash_map::HashMap::from([
        (Key::new(1_usize), MagicPoints::from(0_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_chuunibyou() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<Chuunibyou>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(Chuunibyou::from(0_u32))),
        (String::from("Megumin"),   Some(Chuunibyou::from(u32::MAX))),
        (String::from("Aqua"),      Some(Chuunibyou::from(0_u32))),
        (String::from("Darkness"),  Some(Chuunibyou::from(0_u32))),
        (String::from("Yunyun"),    Some(Chuunibyou::from(1_u32))),
        (String::from("Wiz"),       Some(Chuunibyou::from(0_u32))),
        (String::from("Chris"),     Some(Chuunibyou::from(0_u32))),
        (String::from("Mitsurugi"), Some(Chuunibyou::from(100_u32))),
    ]);
    let key = Key::new(1_usize);

    for (name, chuunibyou) in expected_map.iter() {
        let expected = chuunibyou.clone();
        let result = characters
            .get_unchecked(name)
            .get::<Chuunibyou, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_chuunibyou_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<Chuunibyou>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<Chuunibyou>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<Chuunibyou>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<Chuunibyou>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<Chuunibyou>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<Chuunibyou>()),
        (String::from("Chris"),     get_character_map_chris().len::<Chuunibyou>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<Chuunibyou>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_chuunibyou_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(1)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(1)),
        (String::from("Darkness"),  Some(1)),
        (String::from("Yunyun"),    Some(1)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<Chuunibyou>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_chuunibyou1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Chuunibyou>, Chuunibyou> = hash_map::HashMap::from([
        (Key::new(1_usize), Chuunibyou::from(0_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_chuunibyou2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, Chuunibyou>, Chuunibyou> = hash_map::HashMap::from([
        (Key::new(1_usize), Chuunibyou::from(u32::MAX)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_chuunibyou3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Chuunibyou>, Chuunibyou> = hash_map::HashMap::from([
        (Key::new(1_usize), Chuunibyou::from(0_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_chuunibyou4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Chuunibyou>, Chuunibyou> = hash_map::HashMap::from([
        (Key::new(1_usize), Chuunibyou::from(0_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_chuunibyou5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, Chuunibyou>, Chuunibyou> = hash_map::HashMap::from([
        (Key::new(1_usize), Chuunibyou::from(1_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_chuunibyou6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Chuunibyou>, Chuunibyou> = hash_map::HashMap::from([
        (Key::new(1_usize), Chuunibyou::from(0_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_chuunibyou7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, Chuunibyou>, Chuunibyou> = hash_map::HashMap::from([
        (Key::new(1_usize), Chuunibyou::from(0_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_chuunibyou8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Chuunibyou>, Chuunibyou> = hash_map::HashMap::from([
        (Key::new(1_usize), Chuunibyou::from(100_u32)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_equipment() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<Equipment>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(Equipment::from("Chunchunmaru"))),
        (String::from("Megumin"),   Some(Equipment::from("Magic Rod"))),
        (String::from("Aqua"),      Some(Equipment::from("Feather Mantle"))),
        (String::from("Darkness"),  Some(Equipment::from("Adamantite Armor"))),
        (String::from("Yunyun"),    Some(Equipment::from("Short Sword"))),
        (String::from("Wiz"),       Some(Equipment::from("Rosary"))),
        (String::from("Chris"),     Some(Equipment::from("Magic Dagger"))),
        (String::from("Mitsurugi"), Some(Equipment::from("Cursed Sword Gram"))),
    ]);
    let key = Key::new(1_usize);

    for (name, equipment) in expected_map.iter() {
        let expected = equipment.clone();
        let result = characters
            .get_unchecked(name)
            .get::<Equipment, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_equipment_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(3)),
        (String::from("Megumin"),   Some(5)),
        (String::from("Aqua"),      Some(2)),
        (String::from("Darkness"),  Some(2)),
        (String::from("Yunyun"),    Some(2)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<Equipment>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<Equipment>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<Equipment>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<Equipment>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<Equipment>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<Equipment>()),
        (String::from("Chris"),     get_character_map_chris().len::<Equipment>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<Equipment>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_equipment_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(3)),
        (String::from("Megumin"),   Some(5)),
        (String::from("Aqua"),      Some(2)),
        (String::from("Darkness"),  Some(2)),
        (String::from("Yunyun"),    Some(2)),
        (String::from("Wiz"),       Some(1)),
        (String::from("Chris"),     Some(1)),
        (String::from("Mitsurugi"), Some(1)),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<Equipment>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_equipment1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Equipment>, Equipment> = hash_map::HashMap::from([
        (Key::new(1_usize), Equipment::from("Chunchunmaru")),
        (Key::new(2_usize), Equipment::from("Mass-Produced Vanir Mask")),
        (Key::new(3_usize), Equipment::from("Cursed Ring")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (4_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_equipment2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, Equipment>, Equipment> = hash_map::HashMap::from([
        (Key::new(1_usize), Equipment::from("Magic Rod")),
        (Key::new(2_usize), Equipment::from("Big Floppy Wizard Hat")),
        (Key::new(3_usize), Equipment::from("Adventurer's Cloak")),
        (Key::new(4_usize), Equipment::from("Demon Ring")),
        (Key::new(5_usize), Equipment::from("Eye Patch"))
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (6_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_equipment3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Equipment>, Equipment> = hash_map::HashMap::from([
        (Key::new(1_usize), Equipment::from("Feather Mantle")),
        (Key::new(2_usize), Equipment::from("Scepter")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (3_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_equipment4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Equipment>, Equipment> = hash_map::HashMap::from([
        (Key::new(1_usize), Equipment::from("Adamantite Armor")),
        (Key::new(2_usize), Equipment::from("Long Sword")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (3_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_equipment5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, Equipment>, Equipment> = hash_map::HashMap::from([
        (Key::new(1_usize), Equipment::from("Short Sword")),
        (Key::new(2_usize), Equipment::from("Magic Rod")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (3_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_equipment6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Equipment>, Equipment> = hash_map::HashMap::from([
        (Key::new(1_usize), Equipment::from("Rosary")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_equipment7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, Equipment>, Equipment> = hash_map::HashMap::from([
        (Key::new(1_usize), Equipment::from("Magic Dagger")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_equipment8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Equipment>, Equipment> = hash_map::HashMap::from([
        (Key::new(1_usize), Equipment::from("Cursed Sword Gram")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_inventory_item() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<InventoryItem>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(InventoryItem::new("Adventurer Card", 1))),
        (String::from("Megumin"),   Some(InventoryItem::new("Light Of Reflection Scroll", 1))),
        (String::from("Aqua"),      Some(InventoryItem::new("Jarred Snow Sprite", 1))),
        (String::from("Darkness"),  None),
        (String::from("Yunyun"),    Some(InventoryItem::new("Magic Canceller Scroll", 1))),
        (String::from("Wiz"),       Some(InventoryItem::new("Forced Teleport Scroll", 1))),
        (String::from("Chris"),     Some(InventoryItem::new("Barrier Breaker", 1))),
        (String::from("Mitsurugi"), None),
    ]);
    let key = Key::new(1_usize);

    for (name, inventory_item) in expected_map.iter() {
        let expected = inventory_item.clone();
        let result = characters
            .get_unchecked(name)
            .get::<InventoryItem, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_inventory_item_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(6)),
        (String::from("Megumin"),   Some(3)),
        (String::from("Aqua"),      Some(3)),
        (String::from("Darkness"),  None),
        (String::from("Yunyun"),    Some(5)),
        (String::from("Wiz"),       Some(3)),
        (String::from("Chris"),     Some(2)),
        (String::from("Mitsurugi"), None),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<InventoryItem>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<InventoryItem>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<InventoryItem>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<InventoryItem>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<InventoryItem>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<InventoryItem>()),
        (String::from("Chris"),     get_character_map_chris().len::<InventoryItem>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<InventoryItem>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_inventory_item_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(6)),
        (String::from("Megumin"),   Some(3)),
        (String::from("Aqua"),      Some(3)),
        (String::from("Darkness"),  None),
        (String::from("Yunyun"),    Some(5)),
        (String::from("Wiz"),       Some(3)),
        (String::from("Chris"),     Some(2)),
        (String::from("Mitsurugi"), None),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<InventoryItem>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_inventory_item1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, InventoryItem>, InventoryItem> = hash_map::HashMap::from([
        (Key::new(1_usize), InventoryItem::new("Adventurer Card", 1)),
        (Key::new(2_usize), InventoryItem::new("Green Tracksuit", 1)),
        (Key::new(3_usize), InventoryItem::new("Bottle Of Poison", 5)),
        (Key::new(4_usize), InventoryItem::new("Tinymite", 2)),
        (Key::new(5_usize), InventoryItem::new("Silver Arrows", 20)),
        (Key::new(6_usize), InventoryItem::new("Dream Consultation Form", 99)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (7_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_inventory_item2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, InventoryItem>, InventoryItem> = hash_map::HashMap::from([
        (Key::new(1_usize), InventoryItem::new("Light Of Reflection Scroll", 1)),
        (Key::new(2_usize), InventoryItem::new("Sword Of Shack The Ripper", 1)),
        (Key::new(3_usize), InventoryItem::new("Highest-Quality Manatites", 3)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (4_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_inventory_item3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, InventoryItem>, InventoryItem> = hash_map::HashMap::from([
        (Key::new(1_usize), InventoryItem::new("Jarred Snow Sprite", 1)),
        (Key::new(2_usize), InventoryItem::new("Bubbly", 0)),
        (Key::new(3_usize), InventoryItem::new("Coins", 0)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (4_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_inventory_item4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, InventoryItem>, InventoryItem> = hash_map::HashMap::new();

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..0_usize).map(Key::new), (0_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_inventory_item5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, InventoryItem>, InventoryItem> = hash_map::HashMap::from([
        (Key::new(1_usize), InventoryItem::new("Magic Canceller Scroll", 1)),
        (Key::new(2_usize), InventoryItem::new("Manatites", 3)),
        (Key::new(3_usize), InventoryItem::new("Paralyze Booster Potion", 3)),
        (Key::new(4_usize), InventoryItem::new("Yunyun's Spellbook", 1)),
        (Key::new(5_usize), InventoryItem::new("Coins", 1000)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (6_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_inventory_item6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, InventoryItem>, InventoryItem> = hash_map::HashMap::from([
        (Key::new(1_usize), InventoryItem::new("Forced Teleport Scroll", 1)),
        (Key::new(2_usize), InventoryItem::new("Barrier Tool", 1)),
        (Key::new(3_usize), InventoryItem::new("Forbidden Crystal", 1)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (4_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_inventory_item7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, InventoryItem>, InventoryItem> = hash_map::HashMap::from([
        (Key::new(1_usize), InventoryItem::new("Barrier Breaker", 1)),
        (Key::new(2_usize), InventoryItem::new("Rock", 8)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (3_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_inventory_item8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, InventoryItem>, InventoryItem> = hash_map::HashMap::new();

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..0_usize).map(Key::new), (0_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_ability_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(10)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(25)),
        (String::from("Darkness"),  Some(5)),
        (String::from("Yunyun"),    Some(21)),
        (String::from("Wiz"),       Some(25)),
        (String::from("Chris"),     Some(11)),
        (String::from("Mitsurugi"), Some(1)),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<Ability>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<Ability>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<Ability>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<Ability>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<Ability>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<Ability>()),
        (String::from("Chris"),     get_character_map_chris().len::<Ability>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<Ability>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_ability_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(10)),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      Some(25)),
        (String::from("Darkness"),  Some(5)),
        (String::from("Yunyun"),    Some(21)),
        (String::from("Wiz"),       Some(25)),
        (String::from("Chris"),     Some(11)),
        (String::from("Mitsurugi"), Some(1)),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<Ability>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_ability() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<Ability>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Some(Ability::new(AbilityClass::from("Wizard"), "Create Water", 1))),
        (String::from("Megumin"),   Some(Ability::new(AbilityClass::from("Wizard"), "EXPLOSION!!!", 999))),
        (String::from("Aqua"),      Some(Ability::new(AbilityClass::from("Party Trick"), "Nature's Beauty", 1))),
        (String::from("Darkness"),  Some(Ability::new(AbilityClass::from("Crusader"), "Physical Resistance", 0))),
        (String::from("Yunyun"),    Some(Ability::new(AbilityClass::from("Wizard"), "Lightning", 10))),
        (String::from("Wiz"),       Some(Ability::new(AbilityClass::from("Undead"), "Drain Touch", 3))),
        (String::from("Chris"),     Some(Ability::new(AbilityClass::from("Thief"), "Steal", 1))),
        (String::from("Mitsurugi"), Some(Ability::new(AbilityClass::from("Swordmaster"), "Rune Of Saber", 10))),
    ]);
    let key = Key::new(1_usize);

    for (name, ability) in expected_map.iter() {
        let expected = ability.clone();
        let result = characters
            .get_unchecked(name)
            .get::<Ability, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_ability1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Ability>, Ability> = hash_map::HashMap::from([
        (Key::new(1_usize),  Ability::new(AbilityClass::from("Wizard"), "Create Water", 1)),
        (Key::new(2_usize),  Ability::new(AbilityClass::from("Wizard"), "Freeze", 2)),
        (Key::new(3_usize),  Ability::new(AbilityClass::from("Undead"), "Drain Touch", 3)),
        (Key::new(4_usize),  Ability::new(AbilityClass::from("Thief"), "Steal", 1)),
        (Key::new(5_usize),  Ability::new(AbilityClass::from("Thief"), "Lurk", 1)),
        (Key::new(6_usize),  Ability::new(AbilityClass::from("Thief"), "Enemy Detection", 1)),
        (Key::new(7_usize),  Ability::new(AbilityClass::from("Thief"), "Trap Detection", 1)),
        (Key::new(8_usize),  Ability::new(AbilityClass::from("Thief"), "Disarm Trap", 1)),
        (Key::new(9_usize),  Ability::new(AbilityClass::from("Thief"), "Flee", 1)),
        (Key::new(10_usize), Ability::new(AbilityClass::from("Thief"), "Bind", 1)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (11_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_ability2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, Ability>, Ability> = hash_map::HashMap::from([
        (Key::new(1_usize), Ability::new(AbilityClass::from("Wizard"), "EXPLOSION!!!", 999)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_ability3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Ability>, Ability> = hash_map::HashMap::from([
        (Key::new(1_usize),  Ability::new(AbilityClass::from("Party Trick"), "Nature's Beauty", 1)),
        (Key::new(2_usize),  Ability::new(AbilityClass::from("Water Magic"), "Create Water", 1)),
        (Key::new(3_usize),  Ability::new(AbilityClass::from("Water Magic"), "Sacred Create Water", 5)),
        (Key::new(4_usize),  Ability::new(AbilityClass::from("Water Magic"), "Purification", 1)),
        (Key::new(5_usize),  Ability::new(AbilityClass::from("Water Magic"), "Holy Water", 1)),
        (Key::new(6_usize),  Ability::new(AbilityClass::from("Holy Magic"), "Heal", 5)),
        (Key::new(7_usize),  Ability::new(AbilityClass::from("Holy Magic"), "Sacred Highness Heal", 20)),
        (Key::new(8_usize),  Ability::new(AbilityClass::from("Holy Magic"), "Turn Undead", 5)),
        (Key::new(9_usize),  Ability::new(AbilityClass::from("Holy Magic"), "Sacred Turn Undead", 20)),
        (Key::new(10_usize), Ability::new(AbilityClass::from("Holy Magic"), "Exorcism", 5)),
        (Key::new(11_usize), Ability::new(AbilityClass::from("Holy Magic"), "Sacred Exorcism", 20)),
        (Key::new(12_usize), Ability::new(AbilityClass::from("Holy Magic"), "Break Spell", 7)),
        (Key::new(13_usize), Ability::new(AbilityClass::from("Holy Magic"), "Sacred Break Spell", 24)),
        (Key::new(14_usize), Ability::new(AbilityClass::from("Holy Magic"), "Cure Poison", 2)),
        (Key::new(15_usize), Ability::new(AbilityClass::from("Holy Magic"), "Refresh", 1)),
        (Key::new(16_usize), Ability::new(AbilityClass::from("Holy Magic"), "Blessing", 1)),
        (Key::new(17_usize), Ability::new(AbilityClass::from("Holy Magic"), "Powered", 1)),
        (Key::new(18_usize), Ability::new(AbilityClass::from("Holy Magic"), "Haste", 5)),
        (Key::new(19_usize), Ability::new(AbilityClass::from("Holy Magic"), "Protection", 10)),
        (Key::new(20_usize), Ability::new(AbilityClass::from("Holy Magic"), "Resistance", 10)),
        (Key::new(21_usize), Ability::new(AbilityClass::from("Holy Magic"), "Versatile Entertainer", 0)),
        (Key::new(22_usize), Ability::new(AbilityClass::from("Holy Magic"), "Eyes of Providence", 0)),
        (Key::new(23_usize), Ability::new(AbilityClass::from("Holy Magic"), "Reflect", 30)),
        (Key::new(24_usize), Ability::new(AbilityClass::from("Holy Magic"), "Force Fire", 30)),
        (Key::new(25_usize), Ability::new(AbilityClass::from("Holy Magic"), "Magic Seal", 50)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (26_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_ability4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Ability>, Ability> = hash_map::HashMap::from([
        (Key::new(1_usize), Ability::new(AbilityClass::from("Crusader"), "Physical Resistance", 0)),
        (Key::new(2_usize), Ability::new(AbilityClass::from("Crusader"), "Magic Resistance", 0)),
        (Key::new(3_usize), Ability::new(AbilityClass::from("Crusader"), "Debuff Resistance (All Types)", 0)),
        (Key::new(4_usize), Ability::new(AbilityClass::from("Crusader"), "Decoy", 0)),
        (Key::new(5_usize), Ability::new(AbilityClass::from("Crusader"), "Side Slash", 0)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (6_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_ability5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, Ability>, Ability> = hash_map::HashMap::from([
        (Key::new(1_usize),  Ability::new(AbilityClass::from("Wizard"), "Lightning", 10)),
        (Key::new(2_usize),  Ability::new(AbilityClass::from("Wizard"), "Fireball", 10)),
        (Key::new(3_usize),  Ability::new(AbilityClass::from("Wizard"), "Blade Of Wind", 10)),
        (Key::new(4_usize),  Ability::new(AbilityClass::from("Wizard"), "Freeze Gust", 10)),
        (Key::new(5_usize),  Ability::new(AbilityClass::from("Wizard"), "Sleep", 10)),
        (Key::new(6_usize),  Ability::new(AbilityClass::from("Wizard"), "Unlock", 5)),
        (Key::new(7_usize),  Ability::new(AbilityClass::from("Wizard"), "Flash", 8)),
        (Key::new(8_usize),  Ability::new(AbilityClass::from("Wizard"), "Paralyze", 10)),
        (Key::new(9_usize),  Ability::new(AbilityClass::from("Wizard"), "Teleport", 20)),
        (Key::new(10_usize), Ability::new(AbilityClass::from("Wizard"), "Enemy Search", 10)),
        (Key::new(11_usize), Ability::new(AbilityClass::from("Wizard"), "Light Of Saber", 20)),
        (Key::new(12_usize), Ability::new(AbilityClass::from("Wizard"), "Lightning Strike", 20)),
        (Key::new(13_usize), Ability::new(AbilityClass::from("Wizard"), "Energy Ignition", 20)),
        (Key::new(14_usize), Ability::new(AbilityClass::from("Wizard"), "Bottomless Swamp", 20)),
        (Key::new(15_usize), Ability::new(AbilityClass::from("Wizard"), "Cursed Lightning", 20)),
        (Key::new(16_usize), Ability::new(AbilityClass::from("Wizard"), "Cursed Crystal Prison", 20)),
        (Key::new(17_usize), Ability::new(AbilityClass::from("Wizard"), "Inferno", 20)),
        (Key::new(18_usize), Ability::new(AbilityClass::from("Wizard"), "Tornado", 20)),
        (Key::new(19_usize), Ability::new(AbilityClass::from("Wizard"), "Silent", 20)),
        (Key::new(20_usize), Ability::new(AbilityClass::from("Wizard"), "Light Of Reflection", 20)),
        (Key::new(21_usize), Ability::new(AbilityClass::from("Wizard"), "Control Of Weather", 30)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (22_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_ability6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Ability>, Ability> = hash_map::HashMap::from([
        (Key::new(1_usize),  Ability::new(AbilityClass::from("Undead"), "Drain Touch", 3)),
        (Key::new(2_usize),  Ability::new(AbilityClass::from("Undead"), "Hand Of The Immortal King", 10)),
        (Key::new(3_usize),  Ability::new(AbilityClass::from("Undead"), "Physical Resistance", 0)),
        (Key::new(4_usize),  Ability::new(AbilityClass::from("Undead"), "Magic Resistance", 0)),
        (Key::new(5_usize),  Ability::new(AbilityClass::from("Undead"), "Cursed Petrification", 10)),
        (Key::new(6_usize),  Ability::new(AbilityClass::from("Undead"), "Cursed Necromancy", 10)),
        (Key::new(7_usize),  Ability::new(AbilityClass::from("Undead"), "Night Vision", 10)),
        (Key::new(8_usize),  Ability::new(AbilityClass::from("Wizard"), "Anti-Devil Curses", 4)),
        (Key::new(9_usize),  Ability::new(AbilityClass::from("Wizard"), "Freeze", 2)),
        (Key::new(10_usize), Ability::new(AbilityClass::from("Wizard"), "Freeze Gust", 12)),
        (Key::new(11_usize), Ability::new(AbilityClass::from("Wizard"), "Sleep", 10)),
        (Key::new(12_usize), Ability::new(AbilityClass::from("Wizard"), "Crystal Prison", 10)),
        (Key::new(13_usize), Ability::new(AbilityClass::from("Wizard"), "Cursed Crystal Prison", 10)),
        (Key::new(14_usize), Ability::new(AbilityClass::from("Wizard"), "Bottomless Swamp", 10)),
        (Key::new(15_usize), Ability::new(AbilityClass::from("Wizard"), "Cursed Lightning", 10)),
        (Key::new(16_usize), Ability::new(AbilityClass::from("Wizard"), "Inferno", 10)),
        (Key::new(17_usize), Ability::new(AbilityClass::from("Wizard"), "Light Of Saber", 10)),
        (Key::new(18_usize), Ability::new(AbilityClass::from("Wizard"), "Lightning Strike", 10)),
        (Key::new(19_usize), Ability::new(AbilityClass::from("Wizard"), "Create Earth Golem", 10)),
        (Key::new(20_usize), Ability::new(AbilityClass::from("Wizard"), "Create Earth Wall", 10)),
        (Key::new(21_usize), Ability::new(AbilityClass::from("Wizard"), "Enemy Search", 10)),
        (Key::new(22_usize), Ability::new(AbilityClass::from("Wizard"), "Trap Search", 10)),
        (Key::new(23_usize), Ability::new(AbilityClass::from("Wizard"), "Teleport", 10)),
        (Key::new(24_usize), Ability::new(AbilityClass::from("Wizard"), "Random Teleport", 10)),
        (Key::new(25_usize), Ability::new(AbilityClass::from("Wizard"), "Explosion", 100)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (26_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_ability7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, Ability>, Ability> = hash_map::HashMap::from([
        (Key::new(1_usize),  Ability::new(AbilityClass::from("Thief"), "Steal", 1)),
        (Key::new(2_usize),  Ability::new(AbilityClass::from("Thief"), "Lurk", 1)),
        (Key::new(3_usize),  Ability::new(AbilityClass::from("Thief"), "Enemy Detection", 1)),
        (Key::new(4_usize),  Ability::new(AbilityClass::from("Thief"), "Trap Detection", 1)),
        (Key::new(5_usize),  Ability::new(AbilityClass::from("Thief"), "Disarm Trap", 1)),
        (Key::new(6_usize),  Ability::new(AbilityClass::from("Thief"), "Flee", 1)),
        (Key::new(7_usize),  Ability::new(AbilityClass::from("Thief"), "Bind", 1)),
        (Key::new(8_usize),  Ability::new(AbilityClass::from("Thief"), "Skill Bind", 1)),
        (Key::new(9_usize),  Ability::new(AbilityClass::from("Thief"), "Wire Trap", 1)),
        (Key::new(10_usize), Ability::new(AbilityClass::from("Thief"), "Detect Treasure", 1)),
        (Key::new(11_usize), Ability::new(AbilityClass::from("Thief"), "Lockpick", 1)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (12_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_ability8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Ability>, Ability> = hash_map::HashMap::from([
        (Key::new(1_usize), Ability::new(AbilityClass::from("Swordmaster"), "Rune Of Saber", 10)),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_familiar() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<Familiar>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    None),
        (String::from("Megumin"),   Some(Familiar::from("Chomusuke"))),
        (String::from("Aqua"),      None),
        (String::from("Darkness"),  None),
        (String::from("Yunyun"),    None),
        (String::from("Wiz"),       None),
        (String::from("Chris"),     None),
        (String::from("Mitsurugi"), None),
    ]);
    let key = Key::new(1_usize);

    for (name, familiar) in expected_map.iter() {
        let expected = familiar.clone();
        let result = characters
            .get_unchecked(name)
            .get::<Familiar, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_familiar_len1() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    None),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      None),
        (String::from("Darkness"),  None),
        (String::from("Yunyun"),    None),
        (String::from("Wiz"),       None),
        (String::from("Chris"),     None),
        (String::from("Mitsurugi"), None),
    ]);
    let result_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    get_character_map_kazuma().len::<Familiar>()),
        (String::from("Megumin"),   get_character_map_megumin().len::<Familiar>()),
        (String::from("Aqua"),      get_character_map_aqua().len::<Familiar>()),
        (String::from("Darkness"),  get_character_map_darkness().len::<Familiar>()),
        (String::from("Yunyun"),    get_character_map_yunyun().len::<Familiar>()),
        (String::from("Wiz"),       get_character_map_wiz().len::<Familiar>()),
        (String::from("Chris"),     get_character_map_chris().len::<Familiar>()),
        (String::from("Mitsurugi"), get_character_map_mitsurugi().len::<Familiar>()),
    ]);

    for name in expected_map.keys() {
        let expected = expected_map.get(name).unwrap();
        let result = result_map.get(name).unwrap();

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_familiar_len2() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Option<usize>> = hash_map::HashMap::from([
        (String::from("Kazuma"),    None),
        (String::from("Megumin"),   Some(1)),
        (String::from("Aqua"),      None),
        (String::from("Darkness"),  None),
        (String::from("Yunyun"),    None),
        (String::from("Wiz"),       None),
        (String::from("Chris"),     None),
        (String::from("Mitsurugi"), None),
    ]);

    for (name, len) in expected_map.iter() {
        let result = characters.get_unchecked(name).len::<Familiar>();
        let expected = expected_map[name];

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_familiar1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Familiar>, Familiar> = hash_map::HashMap::new();

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..0_usize).map(Key::new), (0_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_familiar2() {
    let character = get_character_map_megumin();
    let expected_map: hash_map::HashMap<Key<usize, Familiar>, Familiar> = hash_map::HashMap::from([
        (Key::new(1_usize), Familiar::from("Chomusuke")),
    ]);

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..1_usize).map(Key::new), (2_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_familiar3() {
    let character = get_character_map_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Familiar>, Familiar> = hash_map::HashMap::new();

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..0_usize).map(Key::new), (0_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_familiar4() {
    let character = get_character_map_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Familiar>, Familiar> = hash_map::HashMap::new();

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..0_usize).map(Key::new), (0_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_familiar5() {
    let character = get_character_map_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, Familiar>, Familiar> = hash_map::HashMap::new();

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..0_usize).map(Key::new), (0_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_familiar6() {
    let character = get_character_map_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Familiar>, Familiar> = hash_map::HashMap::new();

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..0_usize).map(Key::new), (0_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_familiar7() {
    let character = get_character_map_chris();
    let expected_map: hash_map::HashMap<Key<usize, Familiar>, Familiar> = hash_map::HashMap::new();

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..0_usize).map(Key::new), (0_usize..=1024_usize).map(Key::new))
}

#[test]
fn test_heterogeneous_hash_map_familiar8() {
    let character = get_character_map_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Familiar>, Familiar> = hash_map::HashMap::new();

    run_test_heterogeneous_hash_map_accessors(&character, &expected_map, (0_usize..0_usize).map(Key::new), (0_usize..=1024_usize).map(Key::new))
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_inventory_item_insert_remove1() {
    let mut characters = get_character_map();
    let character = characters.get_mut_unchecked("Wiz");
    let max_key = character.get_map_unchecked::<InventoryItem>().keys().max().unwrap().clone();

    assert_eq!(character.len::<CharacterName>(), Some(3));
    assert_eq!(character.len::<Age>(),           Some(1));
    assert_eq!(character.len::<Race>(),          Some(2));
    assert_eq!(character.len::<Class>(),         Some(1));
    assert_eq!(character.len::<Job>(),           Some(2));
    assert_eq!(character.len::<Status>(),        Some(1));
    assert_eq!(character.len::<Description>(),   Some(1));
    assert_eq!(character.len::<Stats>(),         Some(1));
    assert_eq!(character.len::<HitPoints>(),     Some(1));
    assert_eq!(character.len::<MagicPoints>(),   Some(1));
    assert_eq!(character.len::<Familiar>(),      None);
    assert_eq!(character.len::<Chuunibyou>(),    Some(1));
    assert_eq!(character.len::<Equipment>(),     Some(1));
    assert_eq!(character.len::<Ability>(),       Some(25));
    assert_eq!(character.len::<InventoryItem>(), Some(3));

    for key in (1..=1024_usize).map(|i| Key::new(max_key.id() + i)) {
        assert!(!character.contains_key::<InventoryItem, _>(&key));
    }

    for key in (1..=1024_usize).map(|i| Key::new(max_key.id() + i)) {
        let item_name = std::format!("Mysterious Powerful Magical Artifact #{key}");
        character.insert::<InventoryItem>(key, InventoryItem::new(&item_name, 1));
    }

    assert_eq!(character.len::<CharacterName>(), Some(3));
    assert_eq!(character.len::<Age>(),           Some(1));
    assert_eq!(character.len::<Race>(),          Some(2));
    assert_eq!(character.len::<Class>(),         Some(1));
    assert_eq!(character.len::<Job>(),           Some(2));
    assert_eq!(character.len::<Status>(),        Some(1));
    assert_eq!(character.len::<Description>(),   Some(1));
    assert_eq!(character.len::<Stats>(),         Some(1));
    assert_eq!(character.len::<HitPoints>(),     Some(1));
    assert_eq!(character.len::<MagicPoints>(),   Some(1));
    assert_eq!(character.len::<Familiar>(),      None);
    assert_eq!(character.len::<Chuunibyou>(),    Some(1));
    assert_eq!(character.len::<Equipment>(),     Some(1));
    assert_eq!(character.len::<Ability>(),       Some(25));
    assert_eq!(character.len::<InventoryItem>(), Some(1027));

    for key in (1..=1024_usize).map(|i| Key::new(max_key.id() + i)) {
        assert!(character.contains_key::<InventoryItem, _>(&key));
    }

    for key in (1..=1024_usize).map(|i| Key::new(max_key.id() + i)) {
        let item_name = std::format!("Mysterious Powerful Magical Artifact #{key}");
        let expected = Some(InventoryItem::new(&item_name, 1));
        let result = character.remove::<InventoryItem, _>(&key);

        assert_eq!(result, expected);
    }

    assert_eq!(character.len::<CharacterName>(), Some(3));
    assert_eq!(character.len::<Age>(),           Some(1));
    assert_eq!(character.len::<Race>(),          Some(2));
    assert_eq!(character.len::<Class>(),         Some(1));
    assert_eq!(character.len::<Job>(),           Some(2));
    assert_eq!(character.len::<Status>(),        Some(1));
    assert_eq!(character.len::<Description>(),   Some(1));
    assert_eq!(character.len::<Stats>(),         Some(1));
    assert_eq!(character.len::<HitPoints>(),     Some(1));
    assert_eq!(character.len::<MagicPoints>(),   Some(1));
    assert_eq!(character.len::<Familiar>(),      None);
    assert_eq!(character.len::<Chuunibyou>(),    Some(1));
    assert_eq!(character.len::<Equipment>(),     Some(1));
    assert_eq!(character.len::<Ability>(),       Some(25));
    assert_eq!(character.len::<InventoryItem>(), Some(3));

    for key in (1..=1024_usize).map(|i| Key::new(max_key.id() + i)) {
        assert!(!character.contains_key::<InventoryItem, _>(&key));
    }
}
