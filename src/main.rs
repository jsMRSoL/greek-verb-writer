use clap::{App, Arg};
use csv::Writer;
use std::error::Error;
use std::fmt;

// gkverb
// Usage:
//
// Conjugate a given tense, voice and mood of a verb and print it.
// gkverb -- stem p --tva pai
// Conjugate a given tense, voice and mood of a verb and write it to csv.
// gkverb --stem a --tva api --outfile FILE.csv
// Conjugate all the parts of all the verbs from a csv file and write the forms to csv.
// gkverb --infile FILE.csv --outfile FILE.csv

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("greek-verb-writer")
        .arg(
            Arg::with_name("stem")
                .help("Tense and stem, e.g. pres:παυ")
                .short("s")
                .long("stem")
                .multiple(false)
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("tva")
                .help("Tense, voice and mood, e.g. pai,ppi")
                .short("t")
                .long("tva")
                .multiple(true)
                .use_delimiter(true)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("all")
                .help("Print all combinations of tense, voice and mood for the given stem")
                .short("a")
                .long("all")
                .multiple(false)
                .required_unless("tva")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("to-csv")
                .help("Print to csv")
                .short("c")
                .long("to-csv"),
        )
        .get_matches();

    if let Some(stm) = matches.value_of("stem") {
        let stem = stm;
        let mut vb = Verb::new(stem);
        // vb.conj_pai();
        // vb.pai.print();

        let reqs: Vec<&str> = if let Some(tvas) = matches.values_of("tva") {
            tvas.collect()
        } else {
            match vb.stem {
                Stem::Pres(_) => vec!["pai", "ppi", "iai", "ipi"],
                Stem::Fut(_) => vec!["fai", "fmi", "fpi"],
                Stem::Aor(_) => vec!["aai", "ami", "api"],
                Stem::Perf(_) => vec!["pfai", "pfpi", "plai", "plpi"],
            }
        };
        conj_reqs(&mut vb, &reqs);
        print_reqs(&vb, &reqs);
        if matches.is_present("to-csv") {
            to_csv(&vb, &reqs)?;
        }
    }
    Ok(())
}

#[derive(Debug)]
enum Stem {
    Pres(String),
    Fut(String),
    Aor(String),
    Perf(String),
}

impl fmt::Display for Stem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stem::Pres(val) => write!(f, "{}", val),
            Stem::Fut(val) => write!(f, "{}", val),
            Stem::Aor(val) => write!(f, "{}", val),
            Stem::Perf(val) => write!(f, "{}", val),
        }
    }
}

#[derive(Debug)]
enum Conjugated {
    Some(Vec<String>),
    None,
}

impl Conjugated {
    fn print(&self) {
        match self {
            Conjugated::Some(v) => {
                let mut s = String::new();
                for part in v {
                    s.push_str(format!(", {}", part).as_ref());
                }
                println!("{}", &s[2..]);
            }
            Conjugated::None => {}
        }
    }
}

#[derive(Debug)]
struct Verb {
    stem: Stem,
    pai: Conjugated,
    ppi: Conjugated,
    iai: Conjugated,
    ipi: Conjugated,
    fai: Conjugated,
    fmi: Conjugated,
    fpi: Conjugated,
    aai: Conjugated,
    ami: Conjugated,
    api: Conjugated,
}

impl Verb {
    fn new(s: &str) -> Self {
        let stm = Verb::get_stem_type(s);
        Self {
            stem: stm,
            pai: Conjugated::None,
            ppi: Conjugated::None,
            iai: Conjugated::None,
            ipi: Conjugated::None,
            fai: Conjugated::None,
            fmi: Conjugated::None,
            fpi: Conjugated::None,
            aai: Conjugated::None,
            ami: Conjugated::None,
            api: Conjugated::None,
        }
    }

    fn get_stem_type(s: &str) -> Stem {
        let v: Vec<&str> = s.split(":").collect();
        match v[0] {
            t if t == "pres" => Stem::Pres(v[1].to_string()),
            t if t == "fut" => Stem::Fut(v[1].to_string()),
            t if t == "aor" => Stem::Aor(v[1].to_string()),
            t if t == "perf" => Stem::Perf(v[1].to_string()),
            _ => Stem::Pres(v[0].to_string()),
        }
    }

    fn conj_pai(&mut self) {
        let mut v: Vec<String> = Vec::new();
        for ending in ["ω", "εις", "ει", "ομεν", "ετε", "ουσι"].iter() {
            let part = format!("{}{}", &self.stem, ending);
            v.push(part);
        }
        self.pai = Conjugated::Some(v);
    }

    fn conj_ppi(&mut self) {
        let mut v: Vec<String> = Vec::new();
        for ending in ["ομαι", "ῃ", "εται", "ομεθα", "εσθε", "ονται"].iter()
        {
            let part = format!("{}{}", &self.stem, ending);
            v.push(part);
        }
        self.ppi = Conjugated::Some(v);
    }

    fn conj_iai(&mut self) {
        let s = &self.stem.to_string();
        let (aug, stem) = Verb::aug_and_stem(s);
        let mut v: Vec<String> = Vec::new();
        for ending in ["ον", "ες", "ε", "ομεν", "ετε", "ον"].iter() {
            let part = format!("{}{}{}", aug, stem, ending);
            v.push(part);
        }
        self.iai = Conjugated::Some(v);
    }

    fn conj_ipi(&mut self) {
        let s = &self.stem.to_string();
        let (aug, stem) = Verb::aug_and_stem(s);
        let mut v: Vec<String> = Vec::new();
        for ending in ["ομην", "ου", "ετο", "ομεθα", "εσθε", "οντο"].iter() {
            let part = format!("{}{}{}", aug, stem, ending);
            v.push(part);
        }
        self.ipi = Conjugated::Some(v);
    }

    fn conj_fai(&mut self) {
        let mut v: Vec<String> = Vec::new();
        for ending in ["ω", "εις", "ει", "ομεν", "ετε", "ουσι"].iter() {
            let part = format!("{}{}", &self.stem, ending);
            v.push(part);
        }
        self.fai = Conjugated::Some(v);
    }

    fn conj_fmi(&mut self) {
        let mut v: Vec<String> = Vec::new();
        for ending in ["ομαι", "ῃ", "εται", "ομεθα", "εσθε", "ονται"].iter()
        {
            let part = format!("{}{}", &self.stem, ending);
            v.push(part);
        }
        self.fmi = Conjugated::Some(v);
    }

    fn conj_fpi(&mut self) {
        let mut v: Vec<String> = Vec::new();
        for ending in [
            "θησομαι",
            "θησῃ",
            "θησεται",
            "θησομεθα",
            "θησεσθε",
            "θησονται",
        ]
        .iter()
        {
            let part = format!("{}{}", &self.stem, ending);
            v.push(part);
        }
        self.fpi = Conjugated::Some(v);
    }

    fn conj_aai(&mut self) {
        let mut v: Vec<String> = Vec::new();
        for ending in ["α", "ας", "ε", "αμεν", "ατε", "αν"].iter() {
            let part = format!("{}{}", &self.stem, ending);
            v.push(part);
        }
        self.aai = Conjugated::Some(v);
    }

    fn conj_ami(&mut self) {
        let mut v: Vec<String> = Vec::new();
        for ending in ["αμην", "ω", "ατο", "αμεθα", "ασθε", "αντο"].iter() {
            let part = format!("{}{}", &self.stem, ending);
            v.push(part);
        }
        self.ami = Conjugated::Some(v);
    }

    fn conj_api(&mut self) {
        let mut v: Vec<String> = Vec::new();
        for ending in ["θην", "θης", "θη", "θημεν", "θητε", "θησαν"].iter() {
            let part = format!("{}{}", &self.stem, ending);
            v.push(part);
        }
        self.api = Conjugated::Some(v);
    }

    fn aug_and_stem(mut stem: &str) -> (&str, &str) {
        let aug: &str = match stem {
            stm if stm.starts_with("ἀ") => {
                stem = stem.split("ἀ").collect::<Vec<&str>>()[1];
                "ἠ"
            }
            stm if stm.starts_with("ἂ") => {
                stem = stem.split("ἂ").collect::<Vec<&str>>()[1];
                "ἠ"
            }
            stm if stm.starts_with("ἆ") => {
                stem = stem.split("ἆ").collect::<Vec<&str>>()[1];
                "ἠ"
            }
            stm if stm.starts_with("ἁ") => {
                stem = stem.split("ἁ").collect::<Vec<&str>>()[1];
                "ἡ"
            }
            stm if stm.starts_with("ἅ") => {
                stem = stem.split("ἅ").collect::<Vec<&str>>()[1];
                "ἡ"
            }
            stm if stm.starts_with("ἇ") => {
                stem = stem.split("ἇ").collect::<Vec<&str>>()[1];
                "ἡ"
            }
            stm if stm.starts_with("αἰ") => {
                stem = stem.split("αἰ").collect::<Vec<&str>>()[1];
                "ᾐ"
            }
            stm if stm.starts_with("αἴ") => {
                stem = stem.split("αἴ").collect::<Vec<&str>>()[1];
                "ᾐ"
            }
            stm if stm.starts_with("αἶ") => {
                stem = stem.split("αἶ").collect::<Vec<&str>>()[1];
                "ᾐ"
            }
            stm if stm.starts_with("αἱ") => {
                stem = stem.split("αἱ").collect::<Vec<&str>>()[1];
                "ᾑ"
            }
            stm if stm.starts_with("αἵ") => {
                stem = stem.split("αἵ").collect::<Vec<&str>>()[1];
                "ᾑ"
            }
            stm if stm.starts_with("αἷ") => {
                stem = stem.split("αἷ").collect::<Vec<&str>>()[1];
                "ᾑ"
            }
            _ => "ἐ",
        };
        (aug, stem)
    }
}

fn conj_reqs(vb: &mut Verb, reqs: &[&str]) {
    for req in reqs {
        match req {
            &"pai" => vb.conj_pai(),
            &"ppi" => vb.conj_ppi(),
            &"iai" => vb.conj_iai(),
            &"ipi" => vb.conj_ipi(),
            &"fai" => vb.conj_fai(),
            &"fmi" => vb.conj_fmi(),
            &"fpi" => vb.conj_fpi(),
            &"aai" => vb.conj_aai(),
            &"ami" => vb.conj_ami(),
            &"api" => vb.conj_api(),
            _ => {}
        }
    }
}

fn print_reqs(vb: &Verb, reqs: &[&str]) {
    for req in reqs {
        match req {
            &"pai" => vb.pai.print(),
            &"ppi" => vb.ppi.print(),
            &"iai" => vb.iai.print(),
            &"ipi" => vb.ipi.print(),
            &"fai" => vb.fai.print(),
            &"fmi" => vb.fmi.print(),
            &"fpi" => vb.fpi.print(),
            &"aai" => vb.aai.print(),
            &"ami" => vb.ami.print(),
            &"api" => vb.api.print(),
            _ => {
                eprintln!("print_reqs part not recognised.");
            }
        }
    }
}

fn to_csv(vb: &Verb, reqs: &[&str]) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path("./test-output.csv")?;
    for req in reqs {
        let conjugated = match req {
            &"pai" => &vb.pai,
            &"ppi" => &vb.ppi,
            &"iai" => &vb.iai,
            &"ipi" => &vb.ipi,
            &"fai" => &vb.fai,
            &"fmi" => &vb.fmi,
            &"fpi" => &vb.fpi,
            &"aai" => &vb.aai,
            &"ami" => &vb.ami,
            &"api" => &vb.api,
            _ => &vb.pai,
        };
        if let Conjugated::Some(conj) = conjugated {
            wtr.write_record(conj)?;
        }
    }
    wtr.flush()?;
    Ok(())
}
