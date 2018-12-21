pub mod effects;
pub mod meta;

use enums::*;
use FromVeekun;

#[EnumRepr(type = "u8")]
pub enum BattleStyle {
    Attack = 1,
    Defense,
    Support,
}

#[EnumRepr(type = "u8")]
pub enum DamageClass {
    NonDamaging = 1,
    Physical,
    Special,
}

#[EnumRepr(type = "u8")]
pub enum LearnMethod {
    LevelUp = 1,
    Egg,
    Tutor,
    Machine,
    StadiumSurfingPikachu,
    LightBallEgg,
    ColosseumPurification,
    XDShadow,
    XDPurification,
    FormChange,
}

#[EnumRepr(type = "u8")]
pub enum Target {
    SpecificMove = 1,
    SelectedPokemonReuseStolen,
    Ally,
    UsersField,
    UserOrAlly,
    OpponentsField,
    User,
    RandomOpponent,
    AllOtherPokemon,
    SelectedPokemon,
    AllOpponents,
    EntireField,
}

pub fn assert_sanity() {
    assert_eq!(BattleStyle::Support.repr(), 3);
    assert_eq!(DamageClass::Special.repr(), 3);
    assert_eq!(LearnMethod::FormChange.repr(), 10);
    assert_eq!(Target::EntireField.repr(), 12);
    effects::assert_sanity();
    meta::assert_sanity();
}

impl FromVeekun for BattleStyle {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl FromVeekun for DamageClass {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl FromVeekun for LearnMethod {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl FromVeekun for Target {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}
