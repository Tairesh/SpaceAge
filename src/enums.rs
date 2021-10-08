use std::convert::TryFrom;

pub fn next<S: Into<u8> + TryFrom<u8>>(val: S, count: usize) -> S {
    let mut i: u8 = val.into();
    if (i as usize) < count - 1 {
        i += 1;
    } else {
        i = 0;
    }
    S::try_from(i).ok().unwrap()
}

pub fn prev<S: Into<u8> + TryFrom<u8>>(val: S, count: usize) -> S {
    let mut i: u8 = val.into();
    if i > 0 {
        i -= 1;
    } else {
        i = count as u8 - 1;
    }
    S::try_from(i).ok().unwrap()
}
