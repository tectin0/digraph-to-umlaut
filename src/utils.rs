use crate::constants::{LOWER_AE, LOWER_OE, LOWER_UE, UPPER_AE, UPPER_OE, UPPER_UE};

pub(crate) trait ToUmlautLowercase {
    fn to_umlaut_lowercase(&self) -> Vec<u8>;
    fn make_umlaut_lowercase(&mut self) -> ();
}

impl ToUmlautLowercase for Vec<u8> {
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
