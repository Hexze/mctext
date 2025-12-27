use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const REPO: &str = "hexze/mctext";

fn get_fonts_version() -> String {
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "1.0.0".to_string());
    format!("v{}", version)
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed=MCTEXT_FONTS_DIR");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let fonts_dir = out_dir.join("fonts");

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let local_assets = Path::new(&manifest_dir).join("assets");
        if local_assets.exists() {
            emit_local_paths(&local_assets);
            return;
        }
    }

    if let Ok(user_fonts) = env::var("MCTEXT_FONTS_DIR") {
        let user_path = Path::new(&user_fonts);
        if user_path.exists() {
            emit_local_paths(user_path);
            return;
        }
    }

    fs::create_dir_all(&fonts_dir).expect("Failed to create fonts directory");

    let modern = env::var("CARGO_FEATURE_MODERN_FONTS").is_ok();
    let legacy = env::var("CARGO_FEATURE_LEGACY_FONTS").is_ok();
    let special = env::var("CARGO_FEATURE_SPECIAL_FONTS").is_ok();

    if modern {
        download_font_pack("minecraft-fonts-modern.zip", &fonts_dir, "modern");
    }
    if legacy {
        download_font_pack("minecraft-fonts-legacy.zip", &fonts_dir, "legacy");
    }
    if special {
        download_font_pack("minecraft-fonts-special.zip", &fonts_dir, "special");
    }

    emit_downloaded_paths(&fonts_dir, modern, legacy, special);
}

fn emit_local_paths(base: &Path) {
    let modern = env::var("CARGO_FEATURE_MODERN_FONTS").is_ok();
    let legacy = env::var("CARGO_FEATURE_LEGACY_FONTS").is_ok();
    let special = env::var("CARGO_FEATURE_SPECIAL_FONTS").is_ok();

    if modern {
        emit_path("MCTEXT_MODERN_REGULAR", base.join("modern/minecraft.ttf"));
        emit_path("MCTEXT_MODERN_BOLD", base.join("modern/minecraft-bold.ttf"));
        emit_path(
            "MCTEXT_MODERN_ITALIC",
            base.join("modern/minecraft-italic.ttf"),
        );
        emit_path(
            "MCTEXT_MODERN_BOLD_ITALIC",
            base.join("modern/minecraft-bold-italic.ttf"),
        );
    }

    if legacy {
        emit_path("MCTEXT_LEGACY_REGULAR", base.join("legacy/minecraft.ttf"));
        emit_path("MCTEXT_LEGACY_BOLD", base.join("legacy/minecraft-bold.ttf"));
        emit_path(
            "MCTEXT_LEGACY_ITALIC",
            base.join("legacy/minecraft-italic.ttf"),
        );
        emit_path(
            "MCTEXT_LEGACY_BOLD_ITALIC",
            base.join("legacy/minecraft-bold-italic.ttf"),
        );
    }

    if special {
        emit_path("MCTEXT_ENCHANTING", base.join("modern/enchanting.ttf"));
        emit_path("MCTEXT_ILLAGER", base.join("modern/illager.ttf"));
    }
}

fn emit_downloaded_paths(base: &Path, modern: bool, legacy: bool, special: bool) {
    if modern {
        emit_path("MCTEXT_MODERN_REGULAR", base.join("modern/minecraft.ttf"));
        emit_path("MCTEXT_MODERN_BOLD", base.join("modern/minecraft-bold.ttf"));
        emit_path(
            "MCTEXT_MODERN_ITALIC",
            base.join("modern/minecraft-italic.ttf"),
        );
        emit_path(
            "MCTEXT_MODERN_BOLD_ITALIC",
            base.join("modern/minecraft-bold-italic.ttf"),
        );
    }

    if legacy {
        emit_path("MCTEXT_LEGACY_REGULAR", base.join("legacy/minecraft.ttf"));
        emit_path("MCTEXT_LEGACY_BOLD", base.join("legacy/minecraft-bold.ttf"));
        emit_path(
            "MCTEXT_LEGACY_ITALIC",
            base.join("legacy/minecraft-italic.ttf"),
        );
        emit_path(
            "MCTEXT_LEGACY_BOLD_ITALIC",
            base.join("legacy/minecraft-bold-italic.ttf"),
        );
    }

    if special {
        emit_path("MCTEXT_ENCHANTING", base.join("special/enchanting.ttf"));
        emit_path("MCTEXT_ILLAGER", base.join("special/illager.ttf"));
    }
}

fn emit_path(name: &str, path: PathBuf) {
    println!("cargo::rustc-env={}={}", name, path.display());
}

fn download_font_pack(filename: &str, dest: &Path, subdir: &str) {
    let subdir_path = dest.join(subdir);

    if subdir_path.exists()
        && fs::read_dir(&subdir_path)
            .map(|d| d.count() > 0)
            .unwrap_or(false)
    {
        return;
    }

    let version = get_fonts_version();
    let url = format!(
        "https://github.com/{}/releases/download/{}/{}",
        REPO, version, filename
    );

    println!(
        "cargo::warning=Downloading {} fonts from GitHub releases...",
        subdir
    );

    let response = ureq::get(&url)
        .call()
        .unwrap_or_else(|e| panic!("Failed to download {}: {}", filename, e));

    let mut data = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut data)
        .expect("Failed to read response");

    fs::create_dir_all(&subdir_path).expect("Failed to create subdir");

    let cursor = std::io::Cursor::new(data);
    let mut archive = zip::ZipArchive::new(cursor).expect("Failed to open zip");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failed to read zip entry");

        if file.is_dir() {
            continue;
        }

        let name = file.name().to_string();
        if !name.ends_with(".ttf") {
            continue;
        }

        let file_name = Path::new(&name)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or(name);

        let out_path = subdir_path.join(&file_name);
        let mut out_file = File::create(&out_path).expect("Failed to create font file");

        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .expect("Failed to read font");
        out_file.write_all(&contents).expect("Failed to write font");
    }
}
