#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Result::Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Result::Err(Error::InvalidOutputBase);
    }
    let num_vec: Vec<u32> = number.to_vec();
    let invalid_digit = num_vec.iter().find(|&&n| n >= from_base);
    if invalid_digit.is_some() {
        return Result::Err(Error::InvalidDigit(*invalid_digit.unwrap()));
    }
    return Result::Ok(from_base_10(to_base_10(num_vec, from_base), to_base));
}

pub fn to_base_10(number: Vec<u32>, base: u32) -> u32 {
    number
        .into_iter()
        .rfold(
            (0, 0), // (initial sum, initial power)
            |(acc, pow), n| (acc + n * base.pow(pow), pow + 1),
        )
        .0
}

pub fn from_base_10(number: u32, base: u32) -> Vec<u32> {
    let powers: Vec<u32> = [
        vec![1],
        (1..) // infinite sequence
            .take_while(|&exp| base.pow(exp) <= number)
            .map(|exp| base.pow(exp))
            .collect(),
    ]
    .concat();

    powers
        .into_iter()
        .rfold(
            (vec![], number),
            |(mut acc, left): (Vec<u32>, u32), pow: u32| {
                acc.push(left / pow);
                (acc, left % pow)
            },
        )
        .0
}
