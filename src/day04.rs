use regex::Regex;

#[derive(Debug, Default, Clone)]
pub struct Passport<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
}

impl<'a> From<&'a str> for Passport<'a> {
    fn from(s: &'a str) -> Self {
        Passport::new(s)
    }
}

impl<'a> Passport<'a> {
    pub fn new(s: &'a str) -> Self {
        let mut passport = Passport::default();
        for field in s.split(|c| c == ' ' || c == '\n') {
            let kv: Vec<_> = field.split(':').collect();
            match kv[0] {
                "byr" => passport.byr = Some(kv[1]),
                "iyr" => passport.iyr = Some(kv[1]),
                "eyr" => passport.eyr = Some(kv[1]),
                "hgt" => passport.hgt = Some(kv[1]),
                "hcl" => passport.hcl = Some(kv[1]),
                "ecl" => passport.ecl = Some(kv[1]),
                "pid" => passport.pid = Some(kv[1]),
                _ => (),
            }
        }
        passport
    }
    pub fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    pub fn is_valid_strict(&self) -> bool {
        self.is_valid_byr()
            && self.is_valid_iyr()
            && self.is_valid_eyr()
            && self.is_valid_hgt()
            && self.is_valid_hcl()
            && self.is_valid_ecl()
            && self.is_valid_pid()
    }

    pub fn is_valid_byr(&self) -> bool {
        if let Some(byr) = self.byr {
            let y = byr.parse::<usize>().unwrap_or(0);
            !(1920..=2002).contains(&y)
        } else {
            false
        }
    }

    pub fn is_valid_iyr(&self) -> bool {
        if let Some(iyr) = self.iyr {
            let y = iyr.parse::<usize>().unwrap_or(0);
            !(2010..=2020).contains(&y)
        } else {
            false
        }
    }

    pub fn is_valid_eyr(&self) -> bool {
        if let Some(eyr) = self.eyr {
            let y = eyr.parse::<usize>().unwrap_or(0);
            !(2020..=2030).contains(&y)
        } else {
            false
        }
    }

    pub fn is_valid_hgt(&self) -> bool {
        if let Some(hgt) = self.hgt {
            lazy_static! {
                static ref HGT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
            }
            if let Some(c) = HGT_RE.captures(hgt) {
                match c.get(2).map_or("0", |m| m.as_str()) {
                    "cm" => {
                        if let Ok(h) = c.get(1).map_or("0", |m| m.as_str()).parse::<usize>() {
                            !(150..=193).contains(&h)
                        } else {
                            false
                        }
                    }
                    "in" => {
                        if let Ok(h) = c.get(1).map_or("0", |m| m.as_str()).parse::<usize>() {
                            (59..=76).contains(&h)
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn is_valid_hcl(&self) -> bool {
        if let Some(hcl) = self.hcl {
            lazy_static! {
                static ref HCL_RE: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
            }
            HCL_RE.is_match(hcl)
        } else {
            false
        }
    }
    pub fn is_valid_ecl(&self) -> bool {
        if let Some(ecl) = self.ecl {
            lazy_static! {
                static ref EYE_COLORS: Vec<&'static str> =
                    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            }
            EYE_COLORS.contains(&ecl)
        } else {
            false
        }
    }

    pub fn is_valid_pid(&self) -> bool {
        if let Some(pid) = self.pid {
            lazy_static! {
                static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
            }
            PID_RE.is_match(pid)
        } else {
            false
        }
    }
}

pub fn part1(inp: String) {
    println!(
        "{}",
        inp.split("\n\n")
            .map(|p| p.into())
            .filter(|p: &Passport| p.is_valid())
            .count()
    );
}

pub fn part2(inp: String) {
    println!(
        "{}",
        inp.split("\n\n")
            .map(|p| p.into())
            .filter(|p: &Passport| p.is_valid_strict())
            .count()
    );
}
