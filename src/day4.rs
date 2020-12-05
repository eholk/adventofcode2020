use regex::Regex;
use std::collections::HashMap;

pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    println!("Part 1: {}", count_valid_passports(input)?);
    Ok(())
}

fn count_valid_passports<IO: std::io::BufRead>(input: IO) -> std::io::Result<usize> {
    let mut passport = String::new();
    let mut count = 0;
    for line in input.lines() {
        let line = line?;
        if line.trim() == "" {
            if parse_and_validate_passport(passport.as_str()) {
                count += 1
            }
            passport = String::new();
        } else {
            passport += line.as_str();
            passport += "\n";
        }
    }

    if passport != "" {
        if parse_and_validate_passport(passport.as_str()) {
            count += 1
        }
    }

    Ok(count)
}

fn parse_and_validate_passport(passport: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<key>\w+):(?P<value>[\S]+)").unwrap();
    }

    let mut required_fields: HashMap<String, bool> =
        ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"]
            .into_iter()
            .map(|k| (k.to_string(), false))
            .collect();

    for m in RE.captures_iter(passport) {
        let key = &m["key"];

        if key == "cid" {
            continue;
        }

        match required_fields.insert(key.to_string(), true) {
            None => panic!("no unrecognized fields allowed"),
            Some(true) => panic!("no duplicate fields allowed"),
            Some(false) => (), // okay
        }
    }

    required_fields.into_iter().all(|(_, v)| v)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_example_passports() {
        assert!(parse_and_validate_passport(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm"
        ));

        assert!(!parse_and_validate_passport(
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929"
        ));

        assert!(parse_and_validate_passport(
            "hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm"
        ));

        assert!(!parse_and_validate_passport(
            "hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in
        "
        ));
    }

    #[test]
    fn count_example_passports() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm
        
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929
        
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm
        
        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

        assert_eq!(count_valid_passports(std::io::BufReader::new(input.as_bytes())).unwrap(), 2);
    }
}
