#![allow(dead_code)]

// Area Level enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AreaLevel {
    None = 0,
    RogueEncampment = 1,
    BloodMoor = 2,
    ColdPlains = 3,
    StonyField = 4,
    DarkWood = 5,
    BlackMarsh = 6,
    TamoeHighland = 7,
    DenOfEvil = 8,
    CaveLevel1 = 9,
    UndergroundPassageLevel1 = 10,
    HoleLevel1 = 11,
    PitLevel1 = 12,
    CaveLevel2 = 13,
    UndergroundPassageLevel2 = 14,
    HoleLevel2 = 15,
    PitLevel2 = 16,
    BurialGrounds = 17,
    Crypt = 18,
    Mausoleum = 19,
    ForgottenTower = 20,
    TowerCellarLevel1 = 21,
    TowerCellarLevel2 = 22,
    TowerCellarLevel3 = 23,
    TowerCellarLevel4 = 24,
    TowerCellarLevel5 = 25,
    MonasteryGate = 26,
    OuterCloister = 27,
    Barracks = 28,
    JailLevel1 = 29,
    JailLevel2 = 30,
    JailLevel3 = 31,
    InnerCloister = 32,
    Cathedral = 33,
    CatacombsLevel1 = 34,
    CatacombsLevel2 = 35,
    CatacombsLevel3 = 36,
    CatacombsLevel4 = 37,
    Tristram = 38,
    MooMooFarm = 39,
    LutGholein = 40,
    RockyWaste = 41,
    DryHills = 42,
    FarOasis = 43,
    LostCity = 44,
    ValleyOfSnakes = 45,
    CanyonOfTheMagi = 46,
    SewersLevel1Act2 = 47,
    SewersLevel2Act2 = 48,
    SewersLevel3Act2 = 49,
    HaremLevel1 = 50,
    HaremLevel2 = 51,
    PalaceCellarLevel1 = 52,
    PalaceCellarLevel2 = 53,
    PalaceCellarLevel3 = 54,
    StonyTombLevel1 = 55,
    HallsOfTheDeadLevel1 = 56,
    HallsOfTheDeadLevel2 = 57,
    ClawViperTempleLevel1 = 58,
    StonyTombLevel2 = 59,
    HallsOfTheDeadLevel3 = 60,
    ClawViperTempleLevel2 = 61,
    MaggotLairLevel1 = 62,
    MaggotLairLevel2 = 63,
    MaggotLairLevel3 = 64,
    AncientTunnels = 65,
    TalRashasTomb1 = 66,
    TalRashasTomb2 = 67,
    TalRashasTomb3 = 68,
    TalRashasTomb4 = 69,
    TalRashasTomb5 = 70,
    TalRashasTomb6 = 71,
    TalRashasTomb7 = 72,
    DurielsLair = 73,
    ArcaneSanctuary = 74,
    KurastDocks = 75,
    SpiderForest = 76,
    GreatMarsh = 77,
    FlayerJungle = 78,
    LowerKurast = 79,
    KurastBazaar = 80,
    UpperKurast = 81,
    KurastCauseway = 82,
    Travincal = 83,
    SpiderCave = 84,
    SpiderCavern = 85,
    SwampyPitLevel1 = 86,
    SwampyPitLevel2 = 87,
    FlayerDungeonLevel1 = 88,
    FlayerDungeonLevel2 = 89,
    SwampyPitLevel3 = 90,
    FlayerDungeonLevel3 = 91,
    SewersLevel1Act3 = 92,
    SewersLevel2Act3 = 93,
    RuinedTemple = 94,
    DisusedFane = 95,
    ForgottenReliquary = 96,
    ForgottenTemple = 97,
    RuinedFane = 98,
    DisusedReliquary = 99,
    DuranceOfHateLevel1 = 100,
    DuranceOfHateLevel2 = 101,
    DuranceOfHateLevel3 = 102,
    ThePandemoniumFortress = 103,
    OuterSteppes = 104,
    PlainsOfDespair = 105,
    CityOfTheDamned = 106,
    RiverOfFlame = 107,
    ChaosSanctuary = 108,
    Harrogath = 109,
    BloodyFoothills = 110,
    FrigidHighlands = 111,
    ArreatPlateau = 112,
    CrystallinePassage = 113,
    FrozenRiver = 114,
    GlacialTrail = 115,
    DrifterCavern = 116,
    FrozenTundra = 117,
    TheAncientsWay = 118,
    IcyCellar = 119,
    ArreatSummit = 120,
    NihlathaksTemple = 121,
    HallsOfAnguish = 122,
    HallsOfPain = 123,
    HallsOfVaught = 124,
    Abaddon = 125,
    PitOfAcheron = 126,
    InfernalPit = 127,
    TheWorldStoneKeepLevel1 = 128,
    TheWorldStoneKeepLevel2 = 129,
    TheWorldStoneKeepLevel3 = 130,
    ThroneOfDestruction = 131,
    TheWorldstoneChamber = 132,
    MatronsDen = 133,
    ForgottenSands = 134,
    FurnaceOfPain = 135,
    UberTristram = 136,
    MapsAncientTemple = 137,
    MapsDesecratedTemple = 138,
    MapsFrigidPlateau = 139,
    MapsInfernalTrial = 140,
    MapsRuinedCitadel = 141,
}

pub fn object_is_useless(id: u32) -> bool {
    matches!(
        id,
        1 | 4 | 7 | 9 | 10 | 11 | 12 | 26 | 28 | 29 | 31..=39 | 40..=45 | 48 | 49 | 52 | 54..=58
            | 61 | 65 | 66 | 67..=72 | 73 | 76 | 77 | 78 | 80 | 82 | 94 | 95 | 101 | 102 | 103
            | 117 | 121 | 122 | 142 | 143 | 154 | 155 | 158 | 159 | 160..=162 | 169 | 171 | 174
            | 175 | 178 | 179 | 180 | 207..=217 | 218..=221 | 225 | 227 | 228 | 233..=235 | 239
            | 244..=246 | 253 | 254 | 259 | 268 | 269 | 270..=272 | 273 | 274 | 283 | 285..=287
            | 296 | 297 | 307..=327 | 328 | 345..=347 | 348..=353 | 358 | 359 | 363 | 364 | 366
            | 367 | 370 | 374 | 375 | 392..=396 | 400 | 401 | 403 | 409 | 410 | 434..=439 | 442
            | 446 | 447 | 459..=462 | 478 | 480..=482 | 489 | 490 | 500 | 506 | 507 | 510 | 514
            | 515 | 523 | 527 | 528 | 536..=538 | 552..=556 | 560 | 570
    )
}

pub fn object_is_door(id: u32) -> bool {
    matches!(
        id,
        13 | 14 | 15 | 16 | 23 | 24 | 25 | 27 | 47 | 62 | 63 | 64 | 74 | 75 | 91 | 92 | 98 | 99
            | 129 | 153 | 229 | 230 | 290..=295 | 304..=313 | 366 | 367 | 432 | 433 | 434 | 449
            | 451 | 508 | 547 | 564 | 571 | 572
    )
}

pub fn get_object_class(code: u32, _name: &str, operate_fn: i32) -> Option<&'static str> {
    match operate_fn {
        1 => Some("casket"),
        2 => Some("shrine"),
        3 => Some("urn"),
        5 => Some("barrel"),
        7 => Some("barrel-exploding"),
        14 => Some("bolder"),
        19 => Some("rack-armor"),
        20 => Some("rack-weapon"),
        22 => Some("well"),
        23 => Some("waypoint"),
        68 => Some("urn-evil"),
        30 => Some("chest-exploding"),
        40 | 41 | 59 | 58 | 4 => Some("chest"),
        8 | 18 | 29 => Some("door"),
        54 | 52 | 55 | 56 | 9 | 53 | 25 | 45 | 49 | 28 | 24 => Some("quest"),
        _ => {
            if code == 580 || code == 581 {
                Some("chest-super")
            } else {
                None
            }
        }
    }
}

pub fn npc_is_useless(code: u32) -> bool {
    matches!(
        code,
        34..=37
            | 106..=109
            | 146..=159
            | 175..=186
            | 195..=209
            | 217..=226
            | 227
            | 231..=234
            | 244..=246
            | 251
            | 264..=277
            | 289..=293
            | 294
            | 296
            | 297
            | 318..=332
            | 333
            | 339
            | 340..=344
            | 348..=359
            | 363
            | 364
            | 366
            | 367
            | 368
            | 369
            | 370
            | 377
            | 378
            | 392
            | 393
            | 401
            | 403
            | 404
            | 405
            | 406
            | 409
            | 410
            | 411..=418
            | 419..=431
            | 432
            | 433
            | 434
            | 435
            | 478
            | 497..=500
            | 516..=523
            | 528
            | 534..=539
            | 542..=545
            | 556
            | 561
            | 562
            | 567..=569
            | 574
            | 711
    )
}

pub fn get_act(level_code: u32) -> i32 {
    if level_code < 40 {
        0
    } else if level_code < 75 {
        1
    } else if level_code < 103 {
        2
    } else if level_code < 109 {
        3
    } else if level_code < 200 {
        4
    } else {
        -1
    }
}
