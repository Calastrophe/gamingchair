use category::Category;
use egui::{Image, Rgba};

pub mod category;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Equipment {
    #[default]
    Unknown,
    Knife,
    Deagle,
    Dualies,
    Fiveseven,
    Glock,
    AK47,
    AUG,
    AWP,
    FAMAS,
    G3GS1,
    Galil,
    M249,
    M4A4,
    MAC10,
    P90,
    MP5SD,
    UMP45,
    XM1014,
    Bizon,
    MAG7,
    Negev,
    SawedOff,
    Tec9,
    Zeus,
    P2000,
    MP7,
    MP9,
    Nova,
    P250,
    SCAR20,
    SG556,
    SSG08,
    Flashbang,
    HE,
    Smoke,
    Molotov,
    Decoy,
    Incendiary,
    C4,
    M4A1S,
    USPS,
    CZ75,
    Revolver,
}

impl From<i16> for Equipment {
    fn from(value: i16) -> Self {
        match value {
            1 => Equipment::Deagle,
            2 => Equipment::Dualies,
            3 => Equipment::Fiveseven,
            4 => Equipment::Glock,
            7 => Equipment::AK47,
            8 => Equipment::AUG,
            9 => Equipment::AWP,
            10 => Equipment::FAMAS,
            11 => Equipment::G3GS1,
            13 => Equipment::Galil,
            14 => Equipment::M249,
            16 => Equipment::M4A4,
            17 => Equipment::MAC10,
            19 => Equipment::P90,
            23 => Equipment::MP5SD,
            24 => Equipment::UMP45,
            25 => Equipment::XM1014,
            26 => Equipment::Bizon,
            27 => Equipment::MAG7,
            28 => Equipment::Negev,
            29 => Equipment::SawedOff,
            30 => Equipment::Tec9,
            31 => Equipment::Zeus,
            32 => Equipment::P2000,
            33 => Equipment::MP7,
            34 => Equipment::MP9,
            35 => Equipment::Nova,
            36 => Equipment::P250,
            38 => Equipment::SCAR20,
            39 => Equipment::SG556,
            40 => Equipment::SSG08,
            43 => Equipment::Flashbang,
            44 => Equipment::HE,
            45 => Equipment::Smoke,
            46 => Equipment::Molotov,
            47 => Equipment::Decoy,
            48 => Equipment::Incendiary,
            49 => Equipment::C4,
            60 => Equipment::M4A1S,
            61 => Equipment::USPS,
            63 => Equipment::CZ75,
            64 => Equipment::Revolver,
            42 | 59 | 500 | 503 | 505..=509 | 512 | 514..=523 | 525 => Equipment::Knife,
            _ => Equipment::Unknown,
        }
    }
}

impl Equipment {
    pub fn image(&self) -> Option<Image<'_>> {
        let img_src = match self {
            Equipment::Unknown => return None,
            Equipment::Knife => egui::include_image!("../../../assets/equipment/knife.svg"),
            Equipment::AK47 => egui::include_image!("../../../assets/equipment/ak47.svg"),
            Equipment::AUG => egui::include_image!("../../../assets/equipment/aug.svg"),
            Equipment::AWP => egui::include_image!("../../../assets/equipment/awp.svg"),
            Equipment::Bizon => egui::include_image!("../../../assets/equipment/bizon.svg"),
            Equipment::C4 => egui::include_image!("../../../assets/equipment/c4.svg"),
            Equipment::CZ75 => egui::include_image!("../../../assets/equipment/cz75a.svg"),
            Equipment::Deagle => egui::include_image!("../../../assets/equipment/deagle.svg"),
            Equipment::Decoy => egui::include_image!("../../../assets/equipment/decoy.svg"),
            Equipment::FAMAS => egui::include_image!("../../../assets/equipment/famas.svg"),
            Equipment::Fiveseven => egui::include_image!("../../../assets/equipment/fiveseven.svg"),
            Equipment::Flashbang => egui::include_image!("../../../assets/equipment/flashbang.svg"),
            Equipment::G3GS1 => egui::include_image!("../../../assets/equipment/g3sg1.svg"),
            Equipment::Galil => egui::include_image!("../../../assets/equipment/galilar.svg"),
            Equipment::Glock => egui::include_image!("../../../assets/equipment/glock.svg"),
            Equipment::HE => egui::include_image!("../../../assets/equipment/hegrenade.svg"),
            Equipment::Incendiary => {
                egui::include_image!("../../../assets/equipment/incgrenade.svg")
            }
            Equipment::M249 => egui::include_image!("../../../assets/equipment/m249.svg"),
            Equipment::M4A1S => egui::include_image!("../../../assets/equipment/m4a1_silencer.svg"),
            Equipment::M4A4 => egui::include_image!("../../../assets/equipment/m4a1.svg"),
            Equipment::MAC10 => egui::include_image!("../../../assets/equipment/mac10.svg"),
            Equipment::MAG7 => egui::include_image!("../../../assets/equipment/mag7.svg"),
            Equipment::Molotov => egui::include_image!("../../../assets/equipment/molotov.svg"),
            Equipment::MP5SD => egui::include_image!("../../../assets/equipment/mp5sd.svg"),
            Equipment::MP7 => egui::include_image!("../../../assets/equipment/mp7.svg"),
            Equipment::MP9 => egui::include_image!("../../../assets/equipment/mp9.svg"),
            Equipment::Negev => egui::include_image!("../../../assets/equipment/negev.svg"),
            Equipment::Nova => egui::include_image!("../../../assets/equipment/nova.svg"),
            Equipment::P2000 => egui::include_image!("../../../assets/equipment/p2000.svg"),
            Equipment::P250 => egui::include_image!("../../../assets/equipment/p250.svg"),
            Equipment::P90 => egui::include_image!("../../../assets/equipment/p90.svg"),
            Equipment::Revolver => egui::include_image!("../../../assets/equipment/revolver.svg"),
            Equipment::SawedOff => egui::include_image!("../../../assets/equipment/sawedoff.svg"),
            Equipment::SCAR20 => egui::include_image!("../../../assets/equipment/scar20.svg"),
            Equipment::SG556 => egui::include_image!("../../../assets/equipment/sg556.svg"),
            Equipment::Smoke => {
                egui::include_image!("../../../assets/equipment/smokegrenade.svg")
            }
            Equipment::SSG08 => egui::include_image!("../../../assets/equipment/ssg08.svg"),
            Equipment::Zeus => egui::include_image!("../../../assets/equipment/taser.svg"),
            Equipment::Tec9 => egui::include_image!("../../../assets/equipment/tec9.svg"),
            Equipment::UMP45 => egui::include_image!("../../../assets/equipment/ump45.svg"),
            Equipment::USPS => {
                egui::include_image!("../../../assets/equipment/usp_silencer.svg")
            }
            Equipment::XM1014 => egui::include_image!("../../../assets/equipment/xm1014.svg"),
            Equipment::Dualies => egui::include_image!("../../../assets/equipment/elite.svg"),
        };

        Some(Image::new(img_src).bg_fill(Rgba::TRANSPARENT))
    }

    pub fn category(&self) -> Category {
        match self {
            Equipment::AK47
            | Equipment::M4A4
            | Equipment::M4A1S
            | Equipment::AUG
            | Equipment::FAMAS
            | Equipment::SG556
            | Equipment::SSG08
            | Equipment::AWP
            | Equipment::Galil
            | Equipment::M249
            | Equipment::MAC10
            | Equipment::P90
            | Equipment::MP5SD
            | Equipment::UMP45
            | Equipment::XM1014
            | Equipment::Bizon
            | Equipment::MAG7
            | Equipment::Negev
            | Equipment::SawedOff
            | Equipment::MP7
            | Equipment::MP9
            | Equipment::Nova
            | Equipment::SCAR20
            | Equipment::G3GS1 => Category::Primary,
            Equipment::Deagle
            | Equipment::Glock
            | Equipment::P2000
            | Equipment::P250
            | Equipment::CZ75
            | Equipment::Revolver
            | Equipment::Tec9
            | Equipment::USPS
            | Equipment::Dualies
            | Equipment::Fiveseven => Category::Secondary,
            Equipment::Flashbang
            | Equipment::HE
            | Equipment::Smoke
            | Equipment::Molotov
            | Equipment::Decoy
            | Equipment::Incendiary => Category::Utility,
            Equipment::C4 | Equipment::Zeus | Equipment::Knife | Equipment::Unknown => {
                Category::Special
            }
        }
    }

    pub fn is_sniper(&self) -> bool {
        matches!(self, Equipment::SSG08 | Equipment::AWP | Equipment::SCAR20)
    }

    pub fn is_primary(&self) -> bool {
        matches!(self.category(), Category::Primary)
    }

    pub fn is_secondary(&self) -> bool {
        matches!(self.category(), Category::Secondary)
    }

    pub fn is_utility(&self) -> bool {
        matches!(self.category(), Category::Utility)
    }

    pub fn is_special(&self) -> bool {
        matches!(self.category(), Category::Special)
    }
}
