use egui::{Image, Rgba};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Map {
    Italy,
    Office,
    Ancient,
    Anubis,
    Dust2,
    Inferno,
    Mirage,
    Nuke,
    Overpass,
    Train,
    Vertigo,
}

impl Map {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Italy => "Italy",
            Self::Office => "Office",
            Self::Ancient => "Ancient",
            Self::Anubis => "Anubis",
            Self::Dust2 => "Dust2",
            Self::Inferno => "Inferno",
            Self::Mirage => "Mirage",
            Self::Nuke => "Nuke",
            Self::Overpass => "Overpass",
            Self::Train => "Train",
            Self::Vertigo => "Vertigo",
        }
    }

    pub fn image(&self) -> Image<'_> {
        let img_src = match self {
            Self::Italy => egui::include_image!("../../assets/Italy.png"),
            Self::Office => egui::include_image!("../../assets/Office.png"),
            Self::Ancient => egui::include_image!("../../assets/Ancient.png"),
            Self::Anubis => egui::include_image!("../../assets/Anubis.png"),
            Self::Dust2 => egui::include_image!("../../assets/Dust2.png"),
            Self::Inferno => egui::include_image!("../../assets/Inferno.png"),
            Self::Mirage => egui::include_image!("../../assets/Mirage.png"),
            Self::Nuke => egui::include_image!("../../assets/Nuke.png"),
            Self::Overpass => egui::include_image!("../../assets/Overpass.png"),
            Self::Train => egui::include_image!("../../assets/Train.png"),
            Self::Vertigo => egui::include_image!("../../assets/Vertigo.png"),
        };

        Image::new(img_src).bg_fill(Rgba::TRANSPARENT)
    }
}
