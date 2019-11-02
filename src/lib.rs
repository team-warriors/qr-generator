extern crate qrcode;
extern crate image;

use qrcode::QrCode;
use image::Luma;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::env;
use std::io::Result as IoResult;

pub struct ImageName {
    pub image_name: String,
}

impl Hash for ImageName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.image_name.hash(state);
    }
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn get_current_path() -> IoResult<String> {
    let path = env::current_dir()?;
    Ok(path.display().to_string())
}

pub fn generate(content: String, name: String) -> IoResult<()> {
    let code = QrCode::new(content).unwrap();
    let image = code.render::<Luma<u8>>().build();

    let current_path: String = get_current_path().unwrap();
    let image_name = ImageName {
        image_name: name,
    };
    let image_name_to_hash = calculate_hash(&image_name).to_string();
    let image_path: String = format!("{}/{}.png", current_path, image_name_to_hash);

    image.save(image_path).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::generate;
    use self::calculate_hash;

    #[test]
    fn test_generate() {
        let content = "https://example.com/path".to_string();
        let name = "name_of_image".to_string();

        assert!(generate(content, name).unwrap() == ())
    }

    #[test]
    fn test_calculate_hash() {
        let image1 = ImageName {
            image_name: "image1".to_string(),
        };

        let image2 = ImageName {
            image_name: "image1".to_string(),
        };

        assert!(calculate_hash(&image1) == calculate_hash(&image2))
    }
}
