extern crate colortty;

#[cfg(test)]
mod color_tests {
    mod color {
        use colortty::color::{Color, ColorError};

        #[test]
        fn from_string_works() {
            assert_eq!(
                Color::from_string("12,3,255").unwrap(),
                Color { red: 12, green: 3, blue: 255 }
            );
        }

        #[test]
        fn from_string_invalid_format() {
            assert_eq!(
                Color::from_string("123"),
                Err(ColorError::InvalidFormat)
            );
        }

        #[test]
        fn from_string_parse_int_error() {
            assert!(Color::from_string("abc,3,fo").is_err());
        }

        #[test]
        fn to_hex() {
            assert_eq!(
                Color { red: 123, green: 4, blue: 255 }.to_hex(),
                "0x7b04ff"
            );
        }
    }

    mod color_scheme {
        use colortty::color::{ColorScheme};
        use std::io::{Read};
        use std::fs::File;

        fn read_fixture(filename: &str) -> String {
            let mut fixture= String::new();
            File::open(filename)
                .unwrap()
                .read_to_string(&mut fixture)
                .unwrap();
            return fixture;
        }

        #[test]
        fn convert_minttyrc() {
            let dracula_minttyrc = read_fixture("tests/fixtures/Dracula.minttyrc");
            let dracula_alacritty: String = "colors:
  # Default colors
  primary:
    background: '0x282a36'
    foreground: '0xf8f8f2'

  # Normal colors
  normal:
    black:   '0x000000'
    red:     '0xff5555'
    green:   '0x50fa7b'
    yellow:  '0xf1fa8c'
    blue:    '0xcaa9fa'
    magenta: '0xff79c6'
    cyan:    '0x8be9fd'
    white:   '0xbfbfbf'

  # Bright colors
  bright:
    black:   '0x282a35'
    red:     '0xff6e67'
    green:   '0x5af78e'
    yellow:  '0xf4f99d'
    blue:    '0xcaa9fa'
    magenta: '0xff92d0'
    cyan:    '0x9aedfe'
    white:   '0xe6e6e6'
".to_string();
            let scheme = ColorScheme::from_minttyrc(&dracula_minttyrc);
            assert_eq!(scheme.to_yaml(), dracula_alacritty);
        }

        #[test]
        fn convert_iterm() {
            let dracula_iterm = read_fixture("tests/fixtures/Dracula.itermcolors");
            let dracula_alacritty: String = "colors:
  # Default colors
  primary:
    background: '0x1e1f28'
    foreground: '0xf8f8f2'

  # Normal colors
  normal:
    black:   '0x000000'
    red:     '0xff5555'
    green:   '0x50fa7b'
    yellow:  '0xf1fa8c'
    blue:    '0xbd93f9'
    magenta: '0xff79c6'
    cyan:    '0x8be9fd'
    white:   '0xbbbbbb'

  # Bright colors
  bright:
    black:   '0x555555'
    red:     '0xff5555'
    green:   '0x50fa7b'
    yellow:  '0xf1fa8c'
    blue:    '0xbd93f9'
    magenta: '0xff79c6'
    cyan:    '0x8be9fd'
    white:   '0xffffff'
".to_string();
            let scheme = ColorScheme::from_iterm(&dracula_iterm);
            assert_eq!(scheme.to_yaml(), dracula_alacritty);
        }
    }
}
