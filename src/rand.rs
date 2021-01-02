extern crate getrandom;

use getrandom::getrandom;

/*
Returns a u8
optionally from 0 inclusive to max exclusive
*/
pub fn rand_u8(max: Option<u8>) -> u8 {
    let mut max_usable_value: u16 = 0;
    let mut buffer: [u8; 1] = [0];

    if max.is_some() {
        // not all numbers can multiply into 256 evenly
        max_usable_value = 256 / max.unwrap() as u16 * max.unwrap() as u16;
    }

    loop {
        getrandom(&mut buffer).unwrap();

        if max.is_some() {
            // get a new number if the current one is too large
            if buffer[0] as u16 <= max_usable_value {
                return buffer[0] % max.unwrap();
            }
        } else {
            return buffer[0];
        }
    }
}
