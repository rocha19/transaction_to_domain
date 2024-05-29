pub(crate) struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

trait DisplayColor {
    fn display(&self) -> String;
}

impl Rgb {
    fn new(red: i16, green: i16, blue: i16) -> Result<Self, String> {
        let range = 0..=255;
        if !range.contains(&red) || !range.contains(&green) || !range.contains(&blue) {
            Err(String::from("RGB values must be between 0 and 255"))
        } else {
            Ok(Rgb {
                red: red as u8,
                green: green as u8,
                blue: blue as u8,
            })
        }
    }
}

impl DisplayColor for Rgb {
    fn display(&self) -> String {
        format!("({}, {}, {})", self.red, self.green, self.blue)
    }
}

pub fn rgb() {
    match Rgb::new(255, 100, 50) {
        Ok(color) => println!("Created RGB color: {}", color.display()),
        Err(e) => println!("Error: {}", e),
    }
}
