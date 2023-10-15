#[derive(Debug, Clone)]
pub enum Currency {
    USD,
    EUR,
    JPY,
    GBP,
    CHF,
    AUD,
    CAD,
    CNY,
    HKD,
    SGD,
    MOP,
    MYR,
    RUB,
    BRL,
    INR,
    SEK,
    KRW,
    IDR,
    PHP,
    MXN,
    UAH,
    BYN,
    ZAR,
    KES,
    NGN,
}

#[derive(Debug, Clone)]
pub enum Country {
    USA,
    GBR,
    CAN,
    BRA,
    DEU,
    RUS,
    FRA,
    ESP,
    ITA,
    SWE,
    AUS,
    IND,
    JPN,
    KOR,
    CHN,
    HKG,
    MAC,
    SGP,
    MYS,
    CHE,
    IDN,
    PHL,
    MEX,
    UKR,
    BLR,
    AUT,
    NLD,
    ZAF,
    KEN,
    NGA,
}

impl Currency {
    pub fn countries(&self) -> Vec<Country> {
        match self {
            Currency::USD => vec![Country::USA],
            Currency::EUR => vec![Country::NLD, Country::DEU, Country::FRA, Country::ESP, Country::ITA, Country::AUT],
            Currency::JPY => vec![Country::JPN],
            Currency::GBP => vec![Country::GBR],
            Currency::CHF => vec![Country::CHE],
            Currency::AUD => vec![Country::AUS],
            Currency::CAD => vec![Country::CAN],
            Currency::CNY => vec![Country::CHN],
            Currency::HKD => vec![Country::HKG],
            Currency::SGD => vec![Country::SGP],
            Currency::MOP => vec![Country::MAC],
            Currency::MYR => vec![Country::MYS],
            Currency::RUB => vec![Country::RUS],
            Currency::BRL => vec![Country::BRA],
            Currency::INR => vec![Country::IND],
            Currency::SEK => vec![Country::SWE],
            Currency::KRW => vec![Country::KOR],
            Currency::IDR => vec![Country::IDN],
            Currency::PHP => vec![Country::PHL],
            Currency::MXN => vec![Country::MEX],
            Currency::UAH => vec![Country::UKR],
            Currency::BYN => vec![Country::BLR],
            Currency::ZAR => vec![Country::ZAF],
            Currency::KES => vec![Country::KEN],
            Currency::NGN => vec![Country::NGA],
            _ => todo!(),
        }
    }
}