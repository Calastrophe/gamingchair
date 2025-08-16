use egui::{Image, Rgba};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Map {
    Ancient,
    Anubis,
    Dust2,
    Inferno,
    Mirage,
    Nuke,
    Train,
}

impl Map {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ancient => "Ancient",
            Self::Anubis => "Anubis",
            Self::Dust2 => "Dust2",
            Self::Inferno => "Inferno",
            Self::Mirage => "Mirage",
            Self::Nuke => "Nuke",
            Self::Train => "Train",
        }
    }

    pub fn image(&self) -> Image {
        let img_src = match self {
            Self::Ancient => egui::include_image!("../../assets/ancient.png"),
            Self::Anubis => egui::include_image!("../../assets/anubis.png"),
            Self::Dust2 => egui::include_image!("../../assets/dust2.png"),
            Self::Inferno => egui::include_image!("../../assets/inferno.png"),
            Self::Mirage => egui::include_image!("../../assets/mirage.png"),
            Self::Nuke => egui::include_image!("../../assets/nuke.png"),
            Self::Train => egui::include_image!("../../assets/train.png"),
        };

        Image::new(img_src).bg_fill(Rgba::TRANSPARENT)
    }
}
