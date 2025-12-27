#[cfg(feature = "modern-fonts")]
pub static MINECRAFT_REGULAR: &[u8] = include_bytes!(env!("MCTEXT_MODERN_REGULAR"));
#[cfg(feature = "modern-fonts")]
pub static MINECRAFT_BOLD: &[u8] = include_bytes!(env!("MCTEXT_MODERN_BOLD"));
#[cfg(feature = "modern-fonts")]
pub static MINECRAFT_ITALIC: &[u8] = include_bytes!(env!("MCTEXT_MODERN_ITALIC"));
#[cfg(feature = "modern-fonts")]
pub static MINECRAFT_BOLD_ITALIC: &[u8] = include_bytes!(env!("MCTEXT_MODERN_BOLD_ITALIC"));

#[cfg(feature = "legacy-fonts")]
pub static LEGACY_REGULAR: &[u8] = include_bytes!(env!("MCTEXT_LEGACY_REGULAR"));
#[cfg(feature = "legacy-fonts")]
pub static LEGACY_BOLD: &[u8] = include_bytes!(env!("MCTEXT_LEGACY_BOLD"));
#[cfg(feature = "legacy-fonts")]
pub static LEGACY_ITALIC: &[u8] = include_bytes!(env!("MCTEXT_LEGACY_ITALIC"));
#[cfg(feature = "legacy-fonts")]
pub static LEGACY_BOLD_ITALIC: &[u8] = include_bytes!(env!("MCTEXT_LEGACY_BOLD_ITALIC"));

#[cfg(feature = "special-fonts")]
pub static ENCHANTING_REGULAR: &[u8] = include_bytes!(env!("MCTEXT_ENCHANTING"));
#[cfg(feature = "special-fonts")]
pub static ILLAGER_REGULAR: &[u8] = include_bytes!(env!("MCTEXT_ILLAGER"));

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontFamily {
    #[default]
    Minecraft,
    #[cfg(feature = "special-fonts")]
    Enchanting,
    #[cfg(feature = "special-fonts")]
    Illager,
}

impl FontFamily {
    #[cfg(feature = "modern-fonts")]
    pub fn data(&self) -> &'static [u8] {
        match self {
            FontFamily::Minecraft => MINECRAFT_REGULAR,
            #[cfg(feature = "special-fonts")]
            FontFamily::Enchanting => ENCHANTING_REGULAR,
            #[cfg(feature = "special-fonts")]
            FontFamily::Illager => ILLAGER_REGULAR,
        }
    }

    pub fn supports_styles(&self) -> bool {
        matches!(self, FontFamily::Minecraft)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontVersion {
    #[cfg(feature = "modern-fonts")]
    Modern,
    #[cfg(feature = "legacy-fonts")]
    Legacy,
}

#[cfg(feature = "modern-fonts")]
#[allow(clippy::derivable_impls)]
impl Default for FontVersion {
    fn default() -> Self {
        FontVersion::Modern
    }
}

#[cfg(all(feature = "legacy-fonts", not(feature = "modern-fonts")))]
impl Default for FontVersion {
    fn default() -> Self {
        FontVersion::Legacy
    }
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

    #[cfg(feature = "modern-fonts")]
    pub fn data(&self) -> &'static [u8] {
        self.data_for_version(FontVersion::Modern)
    }

    pub fn data_for_version(&self, version: FontVersion) -> &'static [u8] {
        match (version, self) {
            #[cfg(feature = "modern-fonts")]
            (FontVersion::Modern, FontVariant::Regular) => MINECRAFT_REGULAR,
            #[cfg(feature = "modern-fonts")]
            (FontVersion::Modern, FontVariant::Bold) => MINECRAFT_BOLD,
            #[cfg(feature = "modern-fonts")]
            (FontVersion::Modern, FontVariant::Italic) => MINECRAFT_ITALIC,
            #[cfg(feature = "modern-fonts")]
            (FontVersion::Modern, FontVariant::BoldItalic) => MINECRAFT_BOLD_ITALIC,
            #[cfg(feature = "legacy-fonts")]
            (FontVersion::Legacy, FontVariant::Regular) => LEGACY_REGULAR,
            #[cfg(feature = "legacy-fonts")]
            (FontVersion::Legacy, FontVariant::Bold) => LEGACY_BOLD,
            #[cfg(feature = "legacy-fonts")]
            (FontVersion::Legacy, FontVariant::Italic) => LEGACY_ITALIC,
            #[cfg(feature = "legacy-fonts")]
            (FontVersion::Legacy, FontVariant::BoldItalic) => LEGACY_BOLD_ITALIC,
        }
    }
}
