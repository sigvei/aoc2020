use regex::Regex;
use std::collections::HashMap;

enum PassportFieldType {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

struct PassportField {
    ftype: PassportFieldType,
    value: String,
}

impl PassportField {
    fn from_strings(ftype: &str, value: &str) -> PassportField {
        PassportField {
            ftype: PassportField::ftype_from_string(ftype),
            value: value.to_owned(),
        }
    }

    fn ftype_from_string(ftype: &str) -> PassportFieldType {
        match ftype {
            "byr" => PassportFieldType::Byr,
            "iyr" => PassportFieldType::Iyr,
            "eyr" => PassportFieldType::Eyr,
            "hgt" => PassportFieldType::Hgt,
            "hcl" => PassportFieldType::Hcl,
            "ecl" => PassportFieldType::Ecl,
            "pid" => PassportFieldType::Pid,
            "cid" => PassportFieldType::Cid,
            &_ => panic!("No such passport field type!"),
        }
    }

    fn validate(&self) -> bool {
        match self.ftype {
            PassportFieldType::Byr => self.validate_int(1920, 2002),
            PassportFieldType::Iyr => self.validate_int(2010, 2020),
            PassportFieldType::Eyr => self.validate_int(2020, 2030),
            PassportFieldType::Hgt => self.validate_height(),
            PassportFieldType::Hcl => self.validate_regex("^#[0-9a-f]{6}$"),
            PassportFieldType::Ecl => self.validate_regex("^(amb|blu|brn|gry|grn|hzl|oth)$"),
            PassportFieldType::Pid => self.validate_regex(r"^\d{9}$"),
            PassportFieldType::Cid => true,
        }
    }

    fn validate_int(&self, min: usize, max: usize) -> bool {
        let val: usize = self.value.parse().unwrap();
        val >= min && val <= max
    }

    fn validate_regex(&self, re: &str) -> bool {
        let re = Regex::new(re).unwrap();
        re.is_match(&self.value)
    }

    fn validate_height(&self) -> bool {
        // 158-193 cm or 59-76 inches
        self.validate_regex(r"^(1([5-8]\d|9[0-3])cm|(59|[67][0-9]|7[0-6])in)$")
    }
}

struct Passport {
    metadata: HashMap<String, PassportField>,
}

impl Passport {
    fn from_record(record: &str) -> Passport {
        let mut metadata = HashMap::new();
        record.split_ascii_whitespace().for_each(|r: &str| {
            let val: Vec<&str> = r.split(":").collect();
            let fld = PassportField::from_strings(val[0], val[1]);
            metadata.insert(val[0].to_owned(), fld);
        });
        Passport { metadata }
    }

    fn missing_keys(&self) -> Vec<&str> {
        let all_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

        all_keys
            .into_iter()
            .filter(|k| !self.metadata.contains_key(k.to_owned()))
            .collect()
    }

    fn all_present(&self) -> bool {
        let missing = self.missing_keys();
        missing.is_empty() || (missing.len() == 1 && missing[0] == "cid")
    }

    fn valid(&self) -> bool {
        self.all_present() && self.metadata.values().all(|f| f.validate())
    }
}

pub fn calculate(input: &str) {
    let records = input.split("\n\n");
    let passports: Vec<Passport> = records.map(|rec| Passport::from_record(rec)).collect();

    let mut present = 0;
    let mut valid = 0;

    for passport in passports {
        if passport.all_present() {
            present += 1
        }
        if passport.valid() {
            valid += 1
        }
    }

    println!("Passwords with fields present: {}", present);
    println!("Valid passwords: {}", valid);
}
