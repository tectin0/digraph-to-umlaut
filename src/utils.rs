use crate::constants::{LOWER_AE, LOWER_OE, LOWER_UE, UPPER_AE, UPPER_OE, UPPER_UE};

pub(crate) trait ToUmlautLowercase<T> {
    fn to_umlaut_lowercase(&self) -> T;
    fn make_umlaut_lowercase(&mut self) -> ();
}

impl ToUmlautLowercase<Vec<u8>> for Vec<u8> {
    fn to_umlaut_lowercase(&self) -> Vec<u8> {
        self.iter()
            .map(|&x| match x {
                UPPER_AE => LOWER_AE,
                UPPER_OE => LOWER_OE,
                UPPER_UE => LOWER_UE,
                _ => x,
            })
            .collect()
    }

    fn make_umlaut_lowercase(&mut self) -> () {
        self.into_iter().for_each(|x| match x {
            &mut UPPER_AE => *x = LOWER_AE,
            &mut UPPER_OE => *x = LOWER_OE,
            &mut UPPER_UE => *x = LOWER_UE,
            _ => (),
        });
    }
}

impl ToUmlautLowercase<u8> for u8 {
    fn to_umlaut_lowercase(&self) -> u8 {
        match self {
            &UPPER_AE => LOWER_AE,
            &UPPER_OE => LOWER_OE,
            &UPPER_UE => LOWER_UE,
            _ => *self,
        }
    }

    fn make_umlaut_lowercase(&mut self) -> () {
        match self {
            &mut UPPER_AE => *self = LOWER_AE,
            &mut UPPER_OE => *self = LOWER_OE,
            &mut UPPER_UE => *self = LOWER_UE,
            _ => (),
        }
    }
}
