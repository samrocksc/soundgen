mod text;

use crate::text::Display;
use light_morse::*;

fn main() {
    "Morse".to_string().to_morse(MorseType::Gerke).display();
}
