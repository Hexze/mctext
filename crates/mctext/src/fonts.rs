pub static MINECRAFT_REGULAR: &[u8] = include_bytes!("../assets/modern/minecraft.ttf");
pub static MINECRAFT_BOLD: &[u8] = include_bytes!("../assets/modern/minecraft-bold.ttf");
pub static MINECRAFT_ITALIC: &[u8] = include_bytes!("../assets/modern/minecraft-italic.ttf");
pub static MINECRAFT_BOLD_ITALIC: &[u8] = include_bytes!("../assets/modern/minecraft-bold-italic.ttf");

pub static LEGACY_REGULAR: &[u8] = include_bytes!("../assets/legacy/minecraft.ttf");
pub static LEGACY_BOLD: &[u8] = include_bytes!("../assets/legacy/minecraft-bold.ttf");
pub static LEGACY_ITALIC: &[u8] = include_bytes!("../assets/legacy/minecraft-italic.ttf");
pub static LEGACY_BOLD_ITALIC: &[u8] = include_bytes!("../assets/legacy/minecraft-bold-italic.ttf");

pub static ENCHANTING_REGULAR: &[u8] = include_bytes!("../assets/modern/enchanting.ttf");

pub static ILLAGER_REGULAR: &[u8] = include_bytes!("../assets/modern/illager.ttf");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontFamily {
    #[default]
    Minecraft,
    Enchanting,
    Illager,
}

impl FontFamily {
    pub fn data(&self) -> &'static [u8] {
        match self {
            FontFamily::Minecraft => MINECRAFT_REGULAR,
            FontFamily::Enchanting => ENCHANTING_REGULAR,
            FontFamily::Illager => ILLAGER_REGULAR,
        }
    }

    pub fn supports_styles(&self) -> bool {
        matches!(self, FontFamily::Minecraft)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontVersion {
    #[default]
    Modern,
    Legacy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontVariant {
    #[default]
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

impl FontVariant {
    pub fn from_style(bold: bool, italic: bool) -> Self {
        match (bold, italic) {
            (true, true) => FontVariant::BoldItalic,
            (true, false) => FontVariant::Bold,
            (false, true) => FontVariant::Italic,
            (false, false) => FontVariant::Regular,
        }
    }

    pub fn data(&self) -> &'static [u8] {
        self.data_for_version(FontVersion::Modern)
    }

    pub fn data_for_version(&self, version: FontVersion) -> &'static [u8] {
        match (version, self) {
            (FontVersion::Modern, FontVariant::Regular) => MINECRAFT_REGULAR,
            (FontVersion::Modern, FontVariant::Bold) => MINECRAFT_BOLD,
            (FontVersion::Modern, FontVariant::Italic) => MINECRAFT_ITALIC,
            (FontVersion::Modern, FontVariant::BoldItalic) => MINECRAFT_BOLD_ITALIC,
            (FontVersion::Legacy, FontVariant::Regular) => LEGACY_REGULAR,
            (FontVersion::Legacy, FontVariant::Bold) => LEGACY_BOLD,
            (FontVersion::Legacy, FontVariant::Italic) => LEGACY_ITALIC,
            (FontVersion::Legacy, FontVariant::BoldItalic) => LEGACY_BOLD_ITALIC,
        }
    }
}
