//! Abilities and battle hooks for their effects.

use enums::*;

#[EnumRepr(type = "u8")]
pub enum Ability {
    Stench = 1,
    Drizzle,
    SpeedBoost,
    BattleArmor,
    Sturdy,
    Damp,
    Limber,
    SandVeil,
    Static,
    VoltAbsorb,
    WaterAbsorb,
    Oblivious,
    CloudNine,
    Compoundeyes,
    Insomnia,
    ColorChange,
    Immunity,
    FlashFire,
    ShieldDust,
    OwnTempo,
    SuctionCups,
    Intimidate,
    ShadowTag,
    RoughSkin,
    WonderGuard,
    Levitate,
    EffectSpore,
    Synchronize,
    ClearBody,
    NaturalCure,
    Lightningrod,
    SereneGrace,
    SwiftSwim,
    Chlorophyll,
    Illuminate,
    Trace,
    HugePower,
    PoisonPoint,
    InnerFocus,
    MagmaArmor,
    WaterVeil,
    MagnetPull,
    Soundproof,
    RainDish,
    SandStream,
    Pressure,
    ThickFat,
    EarlyBird,
    FlameBody,
    RunAway,
    KeenEye,
    HyperCutter,
    Pickup,
    Truant,
    Hustle,
    CuteCharm,
    Plus,
    Minus,
    Forecast,
    StickyHold,
    ShedSkin,
    Guts,
    MarvelScale,
    LiquidOoze,
    Overgrow,
    Blaze,
    Torrent,
    Swarm,
    RockHead,
    Drought,
    ArenaTrap,
    VitalSpirit,
    WhiteSmoke,
    PurePower,
    ShellArmor,
    AirLock,
    TangledFeet,
    MotorDrive,
    Rivalry,
    Steadfast,
    SnowCloak,
    Gluttony,
    AngerPoint,
    Unburden,
    Heatproof,
    Simple,
    DrySkin,
    Download,
    IronFist,
    PoisonHeal,
    Adaptability,
    SkillLink,
    Hydration,
    SolarPower,
    QuickFeet,
    Normalize,
    Sniper,
    MagicGuard,
    NoGuard,
    Stall,
    Technitian,
    LeafGuard,
    Klutz,
    MoldBreaker,
    SuperLuck,
    Aftermath,
    Anticipation,
    Forewarn,
    Unaware,
    TintedLens,
    Filter,
    SlowStart,
    Scrappy,
    StormDrain,
    IceBody,
    SolidRock,
    SnowWarning,
    HoneyGather,
    Frisk,
    Reckless,
    Multitype,
    FlowerGift,
    BadDreams,
    Pickpocket,
    SheerForce,
    Contrary,
    Unnerve,
    Defiant,
    Defeatist,
    CursedBody,
    Healer,
    FriendGuard,
    WeakArmor,
    HeavyMetal,
    LightMetal,
    Multiscale,
    ToxicBoost,
    FlareBoost,
    Harvest,
    Telepathy,
    Moody,
    Overcoat,
    PoisonTouch,
    Regenerator,
    BigPecks,
    SandRush,
    WonderSkin,
    Analytic,
    Illusion,
    Imposter,
    Infiltrator,
    Mummy,
    Moxie,
    Justified,
    Rattled,
    MagicBounce,
    SapSipper,
    Prankster,
    SandForce,
    IronBarbs,
    ZenMode,
    VictoryStar,
    Turboblaze,
    Teravolt,
}
