extern crate qrcode;
extern crate image;

use qrcode::QrCode;
use image::Luma;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::env;

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

pub fn get_current_path() -> std::io::Result<String> {
    let path = env::current_dir()?;
    Ok(path.display().to_string())
}

pub fn generate(content: String, name: String) -> std::io::Result<()> {
    let code = QrCode::new(content).unwrap();
    let image = code.render::<Luma<u8>>().build();

    let get_path: String = get_current_path().unwrap().to_owned();
    let mut image_path: String = get_path;
    let add_suffix_path = String::from("/");
    image_path.push_str(&add_suffix_path);

    let generate_image_hash = ImageName {
        image_name: name,
    };
    let generate_image_to_hash = calculate_hash(&generate_image_hash).to_string();
    let convert_image_to_string: String = generate_image_to_hash.to_owned();
    let image_extension_to_string: String = String::from(".png");

    image_path.push_str(&convert_image_to_string);
    image_path.push_str(&image_extension_to_string);
    image.save(image_path).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::generate;
    use self::calculate_hash;

    #[test]
    fn test_render() {
        let content = "https://example.com/path".to_string();
        let name = "name_of_image".to_string();

        generate(content, name).unwrap();
    }

    #[test]
    fn test_calculate_hash() {
        let image1 = ImageName {
            image_name: "image1".to_string(),
        };

        let image2 = ImageName {
            image_name: "image1".to_string(),
        };

        assert_eq!(calculate_hash(&image1), calculate_hash(&image2))
    }
}
