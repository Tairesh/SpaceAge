use std::convert::TryFrom;

pub fn next<S: Into<u8> + TryFrom<u8>>(val: S, count: u8) -> S {
    let mut i: u8 = val.into();
    if i < count - 1 {
        i += 1;
    } else {
        i = 0;
    }
    S::try_from(i).ok().unwrap()
}

pub fn prev<S: Into<u8> + TryFrom<u8>>(val: S, count: u8) -> S {
    let mut i: u8 = val.into();
    if i > 0 {
        i -= 1;
    } else {
        i = count - 1;
    }
    S::try_from(i).ok().unwrap()
}
