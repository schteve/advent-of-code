/*
    --- Day 4: Passport Processing ---
    You arrive at the airport only to realize that you grabbed your North Pole Credentials instead of your passport. While these documents are extremely similar, North Pole Credentials aren't issued by a country and therefore aren't actually valid documentation for travel in most of the world.

    It seems like you're not the only one having problems, though; a very long line has formed for the automatic passport scanners, and the delay could upset your travel itinerary.

    Due to some questionable network security, you realize you might be able to solve both of these problems at the same time.

    The automatic passport scanners are slow because they're having trouble detecting which passports have all required fields. The expected fields are as follows:

    byr (Birth Year)
    iyr (Issue Year)
    eyr (Expiration Year)
    hgt (Height)
    hcl (Hair Color)
    ecl (Eye Color)
    pid (Passport ID)
    cid (Country ID)
    Passport data is validated in batch files (your puzzle input). Each passport is represented as a sequence of key:value pairs separated by spaces or newlines. Passports are separated by blank lines.

    Here is an example batch file containing four passports:

    ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm

    iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929

    hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm

    hcl:#cfa07d eyr:2025 pid:166559648
    iyr:2011 ecl:brn hgt:59in
    The first passport is valid - all eight fields are present. The second passport is invalid - it is missing hgt (the Height field).

    The third passport is interesting; the only missing field is cid, so it looks like data from North Pole Credentials, not a passport at all! Surely, nobody would mind if you made the system temporarily ignore missing cid fields. Treat this "passport" as valid.

    The fourth passport is missing two fields, cid and byr. Missing cid is fine, but missing any other field is not, so this passport is invalid.

    According to the above rules, your improved system would report 2 valid passports.

    Count the number of valid passports - those that have all required fields. Treat cid as optional. In your batch file, how many passports are valid?

    --- Part Two ---
    The line is moving more quickly now, but you overhear airport security talking about how passports with invalid data are getting through. Better add some data validation, quick!

    You can continue to ignore the cid field, but each other field has strict rules about what values are valid for automatic validation:

    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.
    Your job is to count the passports where all required fields are both present and valid according to the above rules. Here are some example values:

    byr valid:   2002
    byr invalid: 2003

    hgt valid:   60in
    hgt valid:   190cm
    hgt invalid: 190in
    hgt invalid: 190

    hcl valid:   #123abc
    hcl invalid: #123abz
    hcl invalid: 123abc

    ecl valid:   brn
    ecl invalid: wat

    pid valid:   000000001
    pid invalid: 0123456789
    Here are some invalid passports:

    eyr:1972 cid:100
    hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

    iyr:2019
    hcl:#602927 eyr:1967 hgt:170cm
    ecl:grn pid:012533040 byr:1946

    hcl:dab227 iyr:2012
    ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

    hgt:59cm ecl:zzz
    eyr:2038 hcl:74454a iyr:2023
    pid:3556412378 byr:2007
    Here are some valid passports:

    pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
    hcl:#623a2f

    eyr:2029 ecl:blu cid:129 byr:1989
    iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

    hcl:#888785
    hgt:164cm byr:2001 iyr:2015 cid:88
    pid:545766238 ecl:hzl
    eyr:2022

    iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
    Count the number of valid passports - those that have all required fields and valid values. Continue to treat cid as optional. In your batch file, how many passports are valid?
*/

use nom::{
    branch::alt,
    bytes::complete::{is_not, take_while_m_n},
    character::{
        complete::{alpha1, char, digit1, line_ending, multispace0, space1},
        is_digit, is_hex_digit,
    },
    combinator::{all_consuming, map_res},
    multi::{many1, separated_list1},
    sequence::{pair, preceded, separated_pair},
    IResult,
};

struct Field {
    key: String,
    value: String,
}

impl Field {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (k, v)) = separated_pair(alpha1, char(':'), is_not(" \t\r\n"))(input)?;

        Ok((
            input,
            Self {
                key: k.to_owned(),
                value: v.to_owned(),
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expire_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    pass_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn new() -> Self {
        Self {
            birth_year: None,
            issue_year: None,
            expire_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            pass_id: None,
            country_id: None,
        }
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, fields) = preceded(
            multispace0,
            separated_list1(alt((space1, line_ending)), Field::parser),
        )(input)?;

        let mut pass = Self::new();
        for field in fields {
            match field.key.as_str() {
                "byr" => pass.birth_year = Some(field.value),
                "iyr" => pass.issue_year = Some(field.value),
                "eyr" => pass.expire_year = Some(field.value),
                "hgt" => pass.height = Some(field.value),
                "hcl" => pass.hair_color = Some(field.value),
                "ecl" => pass.eye_color = Some(field.value),
                "pid" => pass.pass_id = Some(field.value),
                "cid" => pass.country_id = Some(field.value),
                _ => panic!("Unknown field found: {}", field.value),
            }
        }

        Ok((input, pass))
    }

    fn is_valid1(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expire_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.pass_id.is_some()
    }

    fn validate_birth_year(&self) -> bool {
        if let Some(v) = &self.birth_year {
            if let Ok(value) = v.parse::<u32>() {
                1920 <= value && value <= 2002
            } else {
                false
            }
        } else {
            false
        }
    }

    fn validate_issue_year(&self) -> bool {
        if let Some(v) = &self.issue_year {
            if let Ok(value) = v.parse::<u32>() {
                2010 <= value && value <= 2020
            } else {
                false
            }
        } else {
            false
        }
    }

    fn validate_expire_year(&self) -> bool {
        if let Some(v) = &self.expire_year {
            if let Ok(value) = v.parse::<u32>() {
                2020 <= value && value <= 2030
            } else {
                false
            }
        } else {
            false
        }
    }

    fn validate_height(&self) -> bool {
        if let Some(v) = &self.height {
            let result: IResult<&str, (u32, &str)> = all_consuming(pair(
                map_res(digit1, |d: &str| d.parse::<u32>()),
                alpha1,
            ))(v.as_str());

            if let Ok((_, (value, units))) = result {
                match units {
                    "cm" => 150 <= value && value <= 193,
                    "in" => 59 <= value && value <= 76,
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn validate_hair_color(&self) -> bool {
        if let Some(v) = &self.hair_color {
            let result: IResult<&str, &str> = all_consuming(preceded(
                char('#'),
                take_while_m_n(6, 6, |c| is_hex_digit(c as u8)),
            ))(v.as_str());

            result.is_ok()
        } else {
            false
        }
    }

    fn validate_eye_color(&self) -> bool {
        if let Some(v) = &self.eye_color {
            matches!(
                v.as_str(),
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
            )
        } else {
            false
        }
    }

    fn validate_pass_id(&self) -> bool {
        if let Some(v) = &self.pass_id {
            let result: IResult<&str, &str> =
                all_consuming(take_while_m_n(9, 9, |c| is_digit(c as u8)))(v.as_str());
            result.is_ok()
        } else {
            false
        }
    }

    fn validate_country_id(&self) -> bool {
        true
    }

    fn is_valid2(&self) -> bool {
        self.validate_birth_year()
            && self.validate_issue_year()
            && self.validate_expire_year()
            && self.validate_height()
            && self.validate_hair_color()
            && self.validate_eye_color()
            && self.validate_pass_id()
            && self.validate_country_id()
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    many1(Passport::parser)(input).unwrap().1
}

#[aoc(day4, part1)]
pub fn part1(input: &[Passport]) -> usize {
    let count = input.iter().filter(|pass| pass.is_valid1()).count();
    println!("Valid passports: {}", count);
    assert_eq!(count, 192);
    count
}

#[aoc(day4, part2)]
pub fn part2(input: &[Passport]) -> usize {
    let count = input.iter().filter(|pass| pass.is_valid2()).count();
    println!("Valid passports: {}", count);
    assert_eq!(count, 101);
    count
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    static EXAMPLE_INPUT_INVALID: &str = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    static EXAMPLE_INPUT_VALID: &str = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn test_passport_parser() {
        let passports = input_generator(EXAMPLE_INPUT);
        assert_eq!(
            passports,
            [
                Passport {
                    birth_year: Some("1937".into()),
                    issue_year: Some("2017".into()),
                    expire_year: Some("2020".into()),
                    height: Some("183cm".into()),
                    hair_color: Some("#fffffd".into()),
                    eye_color: Some("gry".into()),
                    pass_id: Some("860033327".into()),
                    country_id: Some("147".into()),
                },
                Passport {
                    birth_year: Some("1929".into()),
                    issue_year: Some("2013".into()),
                    expire_year: Some("2023".into()),
                    height: None,
                    hair_color: Some("#cfa07d".into()),
                    eye_color: Some("amb".into()),
                    pass_id: Some("028048884".into()),
                    country_id: Some("350".into()),
                },
                Passport {
                    birth_year: Some("1931".into()),
                    issue_year: Some("2013".into()),
                    expire_year: Some("2024".into()),
                    height: Some("179cm".into()),
                    hair_color: Some("#ae17e1".into()),
                    eye_color: Some("brn".into()),
                    pass_id: Some("760753108".into()),
                    country_id: None,
                },
                Passport {
                    birth_year: None,
                    issue_year: Some("2011".into()),
                    expire_year: Some("2025".into()),
                    height: Some("59in".into()),
                    hair_color: Some("#cfa07d".into()),
                    eye_color: Some("brn".into()),
                    pass_id: Some("166559648".into()),
                    country_id: None,
                },
            ]
        );
    }

    #[test]
    fn test_passport_is_valid1() {
        let passports = input_generator(EXAMPLE_INPUT);
        let valid: Vec<bool> = passports.iter().map(Passport::is_valid1).collect();
        assert_eq!(valid, [true, false, true, false]);
    }

    #[test]
    fn test_passport_is_valid2() {
        let passports = input_generator(EXAMPLE_INPUT_INVALID);
        let valid: Vec<bool> = passports.iter().map(Passport::is_valid2).collect();
        assert_eq!(valid, [false, false, false, false]);

        let passports = input_generator(EXAMPLE_INPUT_VALID);
        let valid: Vec<bool> = passports.iter().map(Passport::is_valid2).collect();
        assert_eq!(valid, [true, true, true, true]);
    }
}
