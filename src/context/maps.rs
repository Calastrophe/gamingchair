use egui::{Image, Rgba};

#[derive(Debug, Default, Clone, Copy)]
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
            "de_ancient" => Map::Ancient,
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
            Self::Ancient => egui::include_image!("../../assets/maps/ancient.png"),
            Self::Anubis => egui::include_image!("../../assets/maps/anubis.png"),
            Self::Dust2 => egui::include_image!("../../assets/maps/dust2.png"),
            Self::Inferno => egui::include_image!("../../assets/maps/inferno.png"),
            Self::Italy => egui::include_image!("../../assets/maps/italy.png"),
            Self::Mirage => egui::include_image!("../../assets/maps/mirage.png"),
            Self::Nuke => egui::include_image!("../../assets/maps/nuke.png"),
            Self::Office => egui::include_image!("../../assets/maps/office.png"),
            Self::Overpass => egui::include_image!("../../assets/maps/overpass.png"),
            Self::Train => egui::include_image!("../../assets/maps/train.png"),
            Self::Vertigo => egui::include_image!("../../assets/maps/vertigo.png"),
        };

        Some(Image::new(img_src).bg_fill(Rgba::TRANSPARENT))
    }
}
