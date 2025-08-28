use egui::{Image, Rgba};

#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub enum Map {
    #[default]
    Empty,
    Ancient,
    Anubis,
    Dust2,
    Inferno,
    Italy,
    Mirage,
    Nuke,
    Office,
    Overpass,
    Train,
    Vertigo,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        match value {
            "de_ancient" | "de_ancient_night" => Map::Ancient,
            "de_anubis" => Map::Anubis,
            "de_dust2" => Map::Dust2,
            "de_inferno" => Map::Inferno,
            "cs_italy" => Map::Italy,
            "de_mirage" => Map::Mirage,
            "de_nuke" => Map::Nuke,
            "cs_office" => Map::Office,
            "de_overpass" => Map::Overpass,
            "de_train" => Map::Train,
            "de_vertigo" => Map::Vertigo,
            _ => Map::Empty,
        }
    }
}

impl Map {
    pub fn image(&self) -> Option<Image<'_>> {
        let img_src = match self {
            Self::Empty => return None,
            Self::Ancient => egui::include_image!("../../../assets/maps/ancient.png"),
            Self::Anubis => egui::include_image!("../../../assets/maps/anubis.png"),
            Self::Dust2 => egui::include_image!("../../../assets/maps/dust2.png"),
            Self::Inferno => egui::include_image!("../../../assets/maps/inferno.png"),
            Self::Italy => egui::include_image!("../../../assets/maps/italy.png"),
            Self::Mirage => egui::include_image!("../../../assets/maps/mirage.png"),
            Self::Nuke => egui::include_image!("../../../assets/maps/nuke.png"),
            Self::Office => egui::include_image!("../../../assets/maps/office.png"),
            Self::Overpass => egui::include_image!("../../../assets/maps/overpass.png"),
            Self::Train => egui::include_image!("../../../assets/maps/train.png"),
            Self::Vertigo => egui::include_image!("../../../assets/maps/vertigo.png"),
        };

        Some(Image::new(img_src).bg_fill(Rgba::TRANSPARENT))
    }

    pub fn zeroing(&self) -> (f32, f32) {
        match self {
            Self::Ancient => (-2953.0, 2164.0),
            Self::Anubis => (-2796.0, 3328.0),
            Self::Dust2 => (-2476.0, 3239.0),
            Self::Inferno => (-2087.0, 3870.0),
            Self::Italy => (-2647.0, 2592.0),
            Self::Mirage => (-3230.0, 1713.0),
            Self::Nuke => (-3453.0, 2887.0),
            Self::Office => (-1838.0, 1858.0),
            Self::Overpass => (-4831.0, 1781.0),
            Self::Train => (-2308.0, 2078.0),
            Self::Vertigo => (-3168.0, 1762.0),
            Self::Empty => unreachable!(),
        }
    }

    pub fn scale(&self) -> f32 {
        match self {
            Self::Ancient => 5.0,
            Self::Anubis => 5.22,
            Self::Dust2 => 4.400000095367432,
            Self::Inferno => 4.9,
            Self::Italy => 4.6,
            Self::Mirage => 5.0,
            Self::Nuke => 7.0,
            Self::Office => 4.1,
            Self::Overpass => 5.2,
            Self::Train => 4.082077,
            Self::Vertigo => 4.0,
            Self::Empty => unreachable!(),
        }
    }
}
