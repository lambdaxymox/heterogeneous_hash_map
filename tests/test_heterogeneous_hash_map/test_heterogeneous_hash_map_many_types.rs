use heterogeneous_hash_map::{
    HeterogeneousHashMap,
    HomogeneousHashMap,
    Key,
};

use std::hash;
use std::string::{String, ToString};

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Chuunibyou(u32);

impl From<u32> for Chuunibyou {
    fn from(age: u32) -> Self {
        Chuunibyou(age)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct HitPoints(u32);

#[derive(Clone, Debug, PartialEq, Eq)]
struct MagicPoints(u32);

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
    character.insert::<Description>(Key::new(1_usize), Description::from("Slovenly shut-in NEET with questionable morals."));
    character.insert::<Stats>(Key::new(1_usize), Stats {
        strength: 12,
        dexterity: 14,
        agility: 13,
        intelligence: 18,
        perception: 14,
        mind: 10,
        luck: 99,
    });
    character.insert::<HitPoints>(Key::new(1_usize), HitPoints(30));
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

fn get_character_megumin() -> HeterogeneousHashMap<usize> {
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
    character.insert::<HitPoints>(Key::new(1_usize), HitPoints(10));
    character.insert::<MagicPoints>(Key::new(1_usize), MagicPoints(999));
    character.insert::<Chuunibyou>(Key::new(1_usize), Chuunibyou(u32::MAX));
    character.insert::<Equipment>(Key::new(1_usize), Equipment::from("Magic Rod"));
    character.insert::<Equipment>(Key::new(2_usize), Equipment::from("Big Floppy Wizard Hat"));
    character.insert::<Equipment>(Key::new(3_usize), Equipment::from("Adventurer's Cloak"));
    character.insert::<Equipment>(Key::new(4_usize), Equipment::from("Demon Ring"));
    character.insert::<InventoryItem>(Key::new(1_usize), InventoryItem::new("Light Of Reflection Scroll", 1));
    character.insert::<InventoryItem>(Key::new(2_usize), InventoryItem::new("Sword Of Shack The Ripper", 1));
    character.insert::<InventoryItem>(Key::new(3_usize), InventoryItem::new("Highest-Quality Manatites", 3));
    character.insert::<Familiar>(Key::new(1_usize), Familiar::from("Chomusuke"));
    character.insert::<Status>(Key::new(1_usize), Status::from("Alive"));
    character.insert::<Ability>(Key::new(1_usize), Ability::new(AbilityClass::from("Wizard"), "Explosion", 999));

    character
}

fn get_character_aqua() -> HeterogeneousHashMap<usize> {
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
        Useless water goddess of the Axis church. Her followers are even crazier than she is.\
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
    character.insert::<HitPoints>(Key::new(1_usize), HitPoints(40));
    character.insert::<MagicPoints>(Key::new(1_usize), MagicPoints(u32::MAX));
    character.insert::<Chuunibyou>(Key::new(1_usize), Chuunibyou(0));
    character.insert::<Equipment>(Key::new(1_usize), Equipment::from("Feather Mantle"));
    character.insert::<Equipment>(Key::new(2_usize), Equipment::from("Scepter"));
    character.insert::<InventoryItem>(Key::new(1_usize), InventoryItem::new("Jarred Snow Sprite", 1));
    character.insert::<InventoryItem>(Key::new(2_usize), InventoryItem::new("Bubbly", 0));
    character.insert::<InventoryItem>(Key::new(3_usize), InventoryItem::new("Coins", 0));
    character.insert::<Ability>(Key::new(1_usize), Ability::new(AbilityClass::from("Party Trick"), "Nature's Beauty", 1));
    character.insert::<Ability>(Key::new(2_usize), Ability::new(AbilityClass::from("Water Magic"), "Create Water", 1));
    character.insert::<Ability>(Key::new(3_usize), Ability::new(AbilityClass::from("Water Magic"), "Sacred Create Water", 5));
    character.insert::<Ability>(Key::new(4_usize), Ability::new(AbilityClass::from("Water Magic"), "Purification", 1));
    character.insert::<Ability>(Key::new(5_usize), Ability::new(AbilityClass::from("Water Magic"), "Holy Water", 1));
    character.insert::<Ability>(Key::new(6_usize), Ability::new(AbilityClass::from("Holy Magic"), "Heal", 5));
    character.insert::<Ability>(Key::new(7_usize), Ability::new(AbilityClass::from("Holy Magic"), "Sacred Highness Heal", 20));
    character.insert::<Ability>(Key::new(8_usize), Ability::new(AbilityClass::from("Holy Magic"), "Turn Undead", 5));
    character.insert::<Ability>(Key::new(9_usize), Ability::new(AbilityClass::from("Holy Magic"), "Sacred Turn Undead", 20));
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

fn get_character_darkness() -> HeterogeneousHashMap<usize> {
    let mut character = HeterogeneousHashMap::new();
    character.insert::<CharacterName>(Key::new(1_usize), CharacterName::from("Darkness"));
    character.insert::<CharacterName>(Key::new(2_usize), CharacterName::from("Lalatina Ford Dustiness"));
    character.insert::<Age>(Key::new(1_usize), Age::from(18));
    character.insert::<Race>(Key::new(1_usize), Race::from("Human"));
    character.insert::<Class>(Key::new(1_usize), Class::from("Crusader"));
    character.insert::<Job>(Key::new(1_usize), Job::from("Noble"));
    character.insert::<Status>(Key::new(1_usize), Status::from("Alive"));
    character.insert::<Description>(Key::new(1_usize), Description::from("\
            A noble crusader who intercepts every blow with unwavering resolve. None of her attacks \
            ever hit their mark.\
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
    character.insert::<Equipment>(Key::new(1_usize), Equipment::from("Long Sword"));
    character.insert::<Equipment>(Key::new(2_usize), Equipment::from("Adamantite Armor"));

    character
}

fn get_character_yunyun() -> HeterogeneousHashMap<usize> {
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

fn get_character_wiz() -> HeterogeneousHashMap<usize> {
    let mut character = HeterogeneousHashMap::new();
    character.insert::<CharacterName>(Key::new(1_usize), CharacterName::from("Wiz"));
    character.insert::<CharacterName>(Key::new(2_usize), CharacterName::from("Ice Witch"));
    character.insert::<CharacterName>(Key::new(3_usize), CharacterName::from("Queen Of The Undead"));
    character.insert::<Age>(Key::new(1_usize), Age::from(20));
    character.insert::<Race>(Key::new(1_usize), Race::from("Lich"));
    character.insert::<Race>(Key::new(2_usize), Race::from("Human"));
    character.insert::<Class>(Key::new(1_usize), Class::from("Arch Wizard"));
    character.insert::<Job>(Key::new(1_usize), Job::from("Devil King's General"));
    character.insert::<Job>(Key::new(2_usize), Job::from("Shopkeeper"));
    character.insert::<Status>(Key::new(1_usize), Status::from("Undead"));
    character.insert::<Description>(Key::new(1_usize), Description::from("\
            A benevolent lich and former general of the Demon King’s army. Now facing her true \
            enemy: small business ownership.\
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
    character.insert::<HitPoints>(Key::new(1_usize), HitPoints(30));
    character.insert::<MagicPoints>(Key::new(1_usize), MagicPoints(700));
    character.insert::<Chuunibyou>(Key::new(1_usize), Chuunibyou(0));
    character.insert::<Equipment>(Key::new(1_usize), Equipment::from("Rosary"));
    character.insert::<InventoryItem>(Key::new(1_usize), InventoryItem::new("Forced Teleport Scroll", 1));
    character.insert::<InventoryItem>(Key::new(1_usize), InventoryItem::new("Barrier Tool", 1));
    character.insert::<InventoryItem>(Key::new(1_usize), InventoryItem::new("Forbidden Crystal", 1));

    character
}

fn get_character_chris() -> HeterogeneousHashMap<usize> {
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
            A humble traveler devoted to justice, fairness, and guiding others through times of trial. \
            Quite good with locks too.\
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
    character.insert::<HitPoints>(Key::new(1_usize), HitPoints(50));
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
    character.insert::<Ability>(Key::new(7_usize), Ability::new(AbilityClass::from("Thief"), "Bind", 1));
    character.insert::<Ability>(Key::new(8_usize), Ability::new(AbilityClass::from("Thief"), "Skill Bind", 1));
    character.insert::<Ability>(Key::new(9_usize), Ability::new(AbilityClass::from("Thief"), "Wire Trap", 1));
    character.insert::<Ability>(Key::new(11_usize), Ability::new(AbilityClass::from("Thief"), "Detect Treasure", 1));
    character.insert::<Ability>(Key::new(11_usize), Ability::new(AbilityClass::from("Thief"), "Lockpick", 1));

    character
}

fn get_character_mitsurugi() -> HeterogeneousHashMap<usize> {
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
    character.insert::<HitPoints>(Key::new(1_usize), HitPoints(100));
    character.insert::<MagicPoints>(Key::new(1_usize), MagicPoints(0));
    character.insert::<Chuunibyou>(Key::new(1_usize), Chuunibyou(u32::MAX - 1));
    character.insert::<Equipment>(Key::new(1_usize), Equipment::from("Cursed Sword Gram"));
    character.insert::<Ability>(Key::new(1_usize), Ability::new(AbilityClass::from("Swordmaster"), "Rune Of Saber", 10));

    character
}

fn get_character_map() -> HomogeneousHashMap<String, HeterogeneousHashMap<usize>> {
    let kazuma = get_character_map_kazuma();
    let megumin = get_character_megumin();
    let aqua = get_character_aqua();
    let darkness = get_character_darkness();
    let yunyun = get_character_yunyun();
    let wiz = get_character_wiz();
    let chris = get_character_chris();
    let mitsurugi = get_character_mitsurugi();

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

#[test]
fn test_heterogeneous_hash_map_character_name() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, CharacterName> = hash_map::HashMap::from([
        (String::from("Kazuma"),    CharacterName::from("Kazuma Satou")),
        (String::from("Megumin"),   CharacterName::from("Megumin")),
        (String::from("Aqua"),      CharacterName::from("Aqua")),
        (String::from("Darkness"),  CharacterName::from("Darkness")),
        (String::from("Yunyun"),    CharacterName::from("Yunyun")),
        (String::from("Wiz"),       CharacterName::from("Wiz")),
        (String::from("Chris"),     CharacterName::from("Chris")),
        (String::from("Mitsurugi"), CharacterName::from("Kyouya Mitsurugi")),
    ]);
    let key = Key::new(1_usize);

    for (name, character_name) in expected_map.iter() {
        let expected = Some(character_name.clone());
        let result = characters
            .get_unchecked(name)
            .get::<CharacterName, _>(&key)
            .map(|s| s.clone());

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

    assert_eq!(character.get::<CharacterName, _>(&0_usize), None);

    for (key, character_name) in expected_map.iter() {
        let expected = Some(character_name.clone());
        let result = character.get::<CharacterName, _>(key).cloned();

        assert_eq!(result, expected);
    }

    for key in (3_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<CharacterName, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_character_name2() {
    let character = get_character_megumin();
    let expected_map: hash_map::HashMap<Key<usize, CharacterName>, CharacterName> = hash_map::HashMap::from([
        (Key::new(1_usize), CharacterName::from("Megumin")),
        (Key::new(2_usize), CharacterName::from("Explosion Maniac")),
        (Key::new(3_usize), CharacterName::from("Crazy Explosion Girl")),
    ]);

    assert_eq!(character.get::<CharacterName, _>(&0_usize), None);

    for (key, character_name) in expected_map.iter() {
        let expected = Some(character_name.clone());
        let result = character.get::<CharacterName, _>(key).cloned();

        assert_eq!(result, expected);
    }

    for key in (4_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<CharacterName, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_character_name3() {
    let character = get_character_aqua();
    let expected_map: hash_map::HashMap<Key<usize, CharacterName>, CharacterName> = hash_map::HashMap::from([
        (Key::new(1_usize), CharacterName::from("Aqua")),
        (Key::new(2_usize), CharacterName::from("Lady Aqua")),
        (Key::new(3_usize), CharacterName::from("Goddess Of Party Tricks")),
    ]);

    assert_eq!(character.get::<CharacterName, _>(&0_usize), None);

    for (key, character_name) in expected_map.iter() {
        let expected = Some(character_name.clone());
        let result = character.get::<CharacterName, _>(key).cloned();

        assert_eq!(result, expected);
    }

    for key in (4_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<CharacterName, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_character_name4() {
    let character = get_character_darkness();
    let expected_map: hash_map::HashMap<Key<usize, CharacterName>, CharacterName> = hash_map::HashMap::from([
        (Key::new(1_usize), CharacterName::from("Darkness")),
        (Key::new(2_usize), CharacterName::from("Lalatina Ford Dustiness")),
    ]);

    assert_eq!(character.get::<CharacterName, _>(&0_usize), None);

    for (key, character_name) in expected_map.iter() {
        let expected = Some(character_name.clone());
        let result = character.get::<CharacterName, _>(key).cloned();

        assert_eq!(result, expected);
    }

    for key in (3_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<CharacterName, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_character_name5() {
    let character = get_character_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, CharacterName>, CharacterName> = hash_map::HashMap::from([
        (Key::new(1_usize), CharacterName::from("Yunyun")),
    ]);

    assert_eq!(character.get::<CharacterName, _>(&0_usize), None);

    for (key, character_name) in expected_map.iter() {
        let expected = Some(character_name.clone());
        let result = character.get::<CharacterName, _>(key).cloned();

        assert_eq!(result, expected);
    }

    for key in (2_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<CharacterName, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_character_name6() {
    let character = get_character_wiz();
    let expected_map: hash_map::HashMap<Key<usize, CharacterName>, CharacterName> = hash_map::HashMap::from([
        (Key::new(1_usize), CharacterName::from("Wiz")),
        (Key::new(2_usize), CharacterName::from("Ice Witch")),
        (Key::new(3_usize), CharacterName::from("Queen Of The Undead")),
    ]);

    assert_eq!(character.get::<CharacterName, _>(&0_usize), None);

    for (key, character_name) in expected_map.iter() {
        let expected = Some(character_name.clone());
        let result = character.get::<CharacterName, _>(key).cloned();

        assert_eq!(result, expected);
    }

    for key in (4_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<CharacterName, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_character_name7() {
    let character = get_character_chris();
    let expected_map: hash_map::HashMap<Key<usize, CharacterName>, CharacterName> = hash_map::HashMap::from([
        (Key::new(1_usize), CharacterName::from("Chris")),
        (Key::new(2_usize), CharacterName::from("Noble Thief")),
    ]);

    assert_eq!(character.get::<CharacterName, _>(&0_usize), None);

    for (key, character_name) in expected_map.iter() {
        let expected = Some(character_name.clone());
        let result = character.get::<CharacterName, _>(key).cloned();

        assert_eq!(result, expected);
    }

    for key in (3_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<CharacterName, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_character_name8() {
    let character = get_character_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, CharacterName>, CharacterName> = hash_map::HashMap::from([
        (Key::new(1_usize), CharacterName::from("Kyouya Mitsurugi")),
        (Key::new(2_usize), CharacterName::from("Cursed Sword Hero")),
        (Key::new(3_usize), CharacterName::from("Magic Sword Guy")),
    ]);

    assert_eq!(character.get::<CharacterName, _>(&0_usize), None);

    for (key, character_name) in expected_map.iter() {
        let expected = Some(character_name.clone());
        let result = character.get::<CharacterName, _>(key).cloned();

        assert_eq!(result, expected);
    }

    for key in (4_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<CharacterName, _>(&key), None);
    }
}

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

#[test]
fn test_heterogeneous_hash_map_player_name1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, PlayerName>, PlayerName> = hash_map::HashMap::from([
        (Key::new(1_usize), PlayerName::from("I'm Kazuma")),
        (Key::new(2_usize), PlayerName::from("That's My Name")),
        (Key::new(3_usize), PlayerName::from("Kazuma Satou")),
    ]);

    assert_eq!(character.get::<PlayerName, _>(&0_usize), None);

    for (key, player_name) in expected_map.iter() {
        let expected = Some(player_name.clone());
        let result = character.get::<PlayerName, _>(key).cloned();

        assert_eq!(result, expected);
    }

    for key in (4_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<PlayerName, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_player_name2() {
    let character = get_character_megumin();
    for key in (0_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<PlayerName, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_player_name3() {
    let character = get_character_aqua();
    let expected_map: hash_map::HashMap<Key<usize, PlayerName>, PlayerName> = hash_map::HashMap::from([
        (Key::new(1_usize), PlayerName::from("Aqua")),
    ]);

    assert_eq!(character.get::<PlayerName, _>(&0_usize), None);

    for (key, player_name) in expected_map.iter() {
        let expected = Some(player_name.clone());
        let result = character.get::<PlayerName, _>(key).cloned();

        assert_eq!(result, expected);
    }

    for key in (2_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<PlayerName, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_player_name4() {
    let character = get_character_darkness();
    for key in (0_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<PlayerName, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_player_name5() {
    let character = get_character_yunyun();
    for key in (0_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<PlayerName, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_player_name6() {
    let character = get_character_wiz();
    for key in (0_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<PlayerName, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_player_name7() {
    let character = get_character_chris();
    let expected_map: hash_map::HashMap<Key<usize, PlayerName>, PlayerName> = hash_map::HashMap::from([
        (Key::new(1_usize), PlayerName::from("Eris")),
    ]);

    assert_eq!(character.get::<PlayerName, _>(&0_usize), None);

    for (key, player_name) in expected_map.iter() {
        let expected = Some(player_name.clone());
        let result = character.get::<PlayerName, _>(key).cloned();

        assert_eq!(result, expected);
    }

    for key in (2_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<PlayerName, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_player_name8() {
    let character = get_character_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, PlayerName>, PlayerName> = hash_map::HashMap::from([
        (Key::new(1_usize), PlayerName::from("Kyouya Mitsurugi")),
    ]);

    assert_eq!(character.get::<PlayerName, _>(&0_usize), None);

    for (key, player_name) in expected_map.iter() {
        let expected = Some(player_name.clone());
        let result = character.get::<PlayerName, _>(key).cloned();

        assert_eq!(result, expected);
    }

    for key in (2_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<PlayerName, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_age() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Age> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Age::from(17)),
        (String::from("Megumin"),   Age::from(14)),
        (String::from("Aqua"),      Age::from(16)),
        (String::from("Darkness"),  Age::from(18)),
        (String::from("Yunyun"),    Age::from(14)),
        (String::from("Wiz"),       Age::from(20)),
        (String::from("Chris"),     Age::from(15)),
        (String::from("Mitsurugi"), Age::from(17)),
    ]);
    let key = Key::new(1_usize);

    for (name, age) in expected_map.iter() {
        let expected = Some(age.clone());
        let result = characters
            .get_unchecked(name)
            .get::<Age, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_age1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(17)),
    ]);

    for (key, age) in expected_map.iter() {
        let expected = Some(age.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_age2() {
    let character = get_character_megumin();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(14)),
    ]);

    for (key, age) in expected_map.iter() {
        let expected = Some(age.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_age3() {
    let character = get_character_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(16)),
        (Key::new(2_usize), Age::from(u32::MAX)),
    ]);

    for (key, age) in expected_map.iter() {
        let expected = Some(age.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_age4() {
    let character = get_character_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(18)),
    ]);

    for (key, age) in expected_map.iter() {
        let expected = Some(age.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_age5() {
    let character = get_character_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(14)),
    ]);

    for (key, age) in expected_map.iter() {
        let expected = Some(age.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_age6() {
    let character = get_character_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(20)),
    ]);

    for (key, age) in expected_map.iter() {
        let expected = Some(age.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_age7() {
    let character = get_character_chris();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(15)),
        (Key::new(2_usize), Age::from(u32::MAX))
    ]);

    for (key, age) in expected_map.iter() {
        let expected = Some(age.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_age8() {
    let character = get_character_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Age>, Age> = hash_map::HashMap::from([
        (Key::new(1_usize), Age::from(17)),
    ]);

    for (key, age) in expected_map.iter() {
        let expected = Some(age.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_race() {
    let mut characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Race> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Race::from("Human")),
        (String::from("Megumin"),   Race::from("Human")),
        (String::from("Aqua"),      Race::from("God")),
        (String::from("Darkness"),  Race::from("Human")),
        (String::from("Yunyun"),    Race::from("Human")),
        (String::from("Wiz"),       Race::from("Lich")),
        (String::from("Chris"),     Race::from("Human")),
        (String::from("Mitsurugi"), Race::from("Human")),
    ]);
    let key = Key::new(1_usize);

    for (name, race) in expected_map.iter() {
        let expected = Some(race.clone());
        let result = characters
            .get_unchecked(name)
            .get::<Race, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_race1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("Human")),
    ]);

    for (key, race) in expected_map.iter() {
        let expected = Some(race.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_race2() {
    let character = get_character_megumin();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("Human")),
        (Key::new(2_usize), Race::from("Crimson Magic Clan")),
    ]);

    for (key, race) in expected_map.iter() {
        let expected = Some(race.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_race3() {
    let character = get_character_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("God")),
    ]);

    for (key, race) in expected_map.iter() {
        let expected = Some(race.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_race4() {
    let character = get_character_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("Human")),
    ]);

    for (key, race) in expected_map.iter() {
        let expected = Some(race.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_race5() {
    let character = get_character_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("Human")),
        (Key::new(2_usize), Race::from("Crimson Magic Clan")),
    ]);

    for (key, race) in expected_map.iter() {
        let expected = Some(race.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_race6() {
    let character = get_character_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("Lich")),
        (Key::new(2_usize), Race::from("Human")),
    ]);

    for (key, race) in expected_map.iter() {
        let expected = Some(race.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_race7() {
    let character = get_character_chris();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("Human")),
        (Key::new(2_usize), Race::from("God"))
    ]);

    for (key, race) in expected_map.iter() {
        let expected = Some(race.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_race8() {
    let character = get_character_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Race>, Race> = hash_map::HashMap::from([
        (Key::new(1_usize), Race::from("Human")),
    ]);

    for (key, race) in expected_map.iter() {
        let expected = Some(race.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_class() {
    let characters = get_character_map();
    let expected_map: hash_map::HashMap<String, Class> = hash_map::HashMap::from([
        (String::from("Kazuma"),    Class::from("Adventurer")),
        (String::from("Megumin"),   Class::from("Arch Wizard")),
        (String::from("Aqua"),      Class::from("Arch Priest")),
        (String::from("Darkness"),  Class::from("Crusader")),
        (String::from("Yunyun"),    Class::from("Arch Wizard")),
        (String::from("Wiz"),       Class::from("Arch Wizard")),
        (String::from("Chris"),     Class::from("Thief")),
        (String::from("Mitsurugi"), Class::from("Swordmaster")),
    ]);
    let key = Key::new(1_usize);

    for (name, class) in expected_map.iter() {
        let expected = Some(class.clone());
        let result = characters
            .get_unchecked(name)
            .get::<Class, _>(&key)
            .map(|s| s.clone());

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_class1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Adventurer")),
    ]);

    for (key, class) in expected_map.iter() {
        let expected = Some(class.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_class2() {
    let character = get_character_megumin();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Arch Wizard")),
    ]);

    for (key, class) in expected_map.iter() {
        let expected = Some(class.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_class3() {
    let character = get_character_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Arch Priest")),
    ]);

    for (key, class) in expected_map.iter() {
        let expected = Some(class.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_class4() {
    let character = get_character_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Crusader")),
    ]);

    for (key, class) in expected_map.iter() {
        let expected = Some(class.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_class5() {
    let character = get_character_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Arch Wizard")),
    ]);

    for (key, class) in expected_map.iter() {
        let expected = Some(class.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_class6() {
    let character = get_character_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Arch Wizard")),
    ]);

    for (key, class) in expected_map.iter() {
        let expected = Some(class.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_class7() {
    let character = get_character_chris();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Thief")),
    ]);

    for (key, class) in expected_map.iter() {
        let expected = Some(class.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_class8() {
    let character = get_character_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Class>, Class> = hash_map::HashMap::from([
        (Key::new(1_usize), Class::from("Swordmaster")),
    ]);

    for (key, class) in expected_map.iter() {
        let expected = Some(class.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

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

#[test]
fn test_heterogeneous_hash_map_job1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Job>, Job> = hash_map::HashMap::from([
        (Key::new(1_usize), Job::from("Jack of All Trades, Master Of Dumb Luck")),
        (Key::new(2_usize), Job::from("Definitely Not A Harem Protagonist")),
        (Key::new(3_usize), Job::from("Strategic Coward")),
    ]);

    for (name, job) in expected_map.iter() {
        let expected = Some(job.clone());
        let result = character.get(name).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_job2() {
    let character = get_character_megumin();
    for key in (0_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<Job, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_job3() {
    let character = get_character_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Job>, Job> = hash_map::HashMap::from([
        (Key::new(1_usize), Job::from("Self-Proclaimed Goddess")),
    ]);

    for (key, job) in expected_map.iter() {
        let expected = Some(job.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_job4() {
    let character = get_character_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Job>, Job> = hash_map::HashMap::from([
        (Key::new(1_usize), Job::from("Noble")),
    ]);

    for (key, job) in expected_map.iter() {
        let expected = Some(job.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_job5() {
    let character = get_character_yunyun();
    for key in (0_usize..=1024_usize).map(Key::new) {
        assert_eq!(character.get::<Job, _>(&key), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_job6() {
    let character = get_character_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Job>, Job> = hash_map::HashMap::from([
        (Key::new(1_usize), Job::from("Devil King's General")),
        (Key::new(2_usize), Job::from("Shopkeeper")),
    ]);

    for (key, job) in expected_map.iter() {
        let expected = Some(job.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_job7() {
    let character = get_character_chris();
    let expected_map: hash_map::HashMap<Key<usize, Job>, Job> = hash_map::HashMap::from([
        (Key::new(1_usize), Job::from("Goddess")),
        (Key::new(2_usize), Job::from("Aqua's Janitor")),
        (Key::new(3_usize), Job::from("Currently Cleaning Up Aqua's Messes. Send Help.")),
    ]);

    for (key, job) in expected_map.iter() {
        let expected = Some(job.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_job8() {
    let character = get_character_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Job>, Job> = hash_map::HashMap::from([
        (Key::new(1_usize), Job::from("Isekai Protagonist")),
    ]);

    for (key, job) in expected_map.iter() {
        let expected = Some(job.clone());
        let result = character.get(key).cloned();

        assert_eq!(result, expected);
    }
}

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

#[test]
fn test_heterogeneous_hash_map_status1() {
    let character = get_character_map_kazuma();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Alive")),
    ]);

    for (name, status) in expected_map.iter() {
        let expected = Some(status.clone());
        let result = character.get(name).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_status2() {
    let character = get_character_megumin();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Alive")),
    ]);

    for (name, status) in expected_map.iter() {
        let expected = Some(status.clone());
        let result = character.get(name).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_status3() {
    let character = get_character_aqua();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Alive")),
    ]);

    for (name, status) in expected_map.iter() {
        let expected = Some(status.clone());
        let result = character.get(name).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_status4() {
    let character = get_character_darkness();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Alive")),
    ]);

    for (name, status) in expected_map.iter() {
        let expected = Some(status.clone());
        let result = character.get(name).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_status5() {
    let character = get_character_yunyun();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Alive")),
    ]);

    for (name, status) in expected_map.iter() {
        let expected = Some(status.clone());
        let result = character.get(name).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_status6() {
    let character = get_character_wiz();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Undead")),
    ]);

    for (name, status) in expected_map.iter() {
        let expected = Some(status.clone());
        let result = character.get(name).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_status7() {
    let character = get_character_chris();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Alive")),
    ]);

    for (name, status) in expected_map.iter() {
        let expected = Some(status.clone());
        let result = character.get(name).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_heterogeneous_hash_map_status8() {
    let character = get_character_mitsurugi();
    let expected_map: hash_map::HashMap<Key<usize, Status>, Status> = hash_map::HashMap::from([
        (Key::new(1_usize), Status::from("Alive")),
    ]);

    for (name, status) in expected_map.iter() {
        let expected = Some(status.clone());
        let result = character.get(name).cloned();

        assert_eq!(result, expected);
    }
}
