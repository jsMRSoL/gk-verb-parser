use crate::verbentry::{Verb, VerbAlternates, VerbEntry, VerbStemSet};
use csv::Reader;
use regex::Regex;
use std::error::Error;
use std::fs;

pub fn check_file(path: &str) -> bool {
    let contents = fs::read_to_string(path).expect(&format!("Unable to read {}.", path));
    // unshifted = accents: ; = acute e.g. ά ' = grave e.g. ὰ
    // shifted = breathings: : = smooth e.g. ἀ @ = rough e.g. ἁ
    // you must press the unshifted key first!
    // ] = iota subscript [ = circumflex. If both, ][
    let re = Regex::new(r"[άἄἅὰἂἃᾴᾲᾷᾶέὲἔἒἕἓήἤἥὴἢἣῆῄῂῇίἴἵὶἲἳόὄὅὸὂὃώὤὥὼὣὢῶῴῲῷύὔὕὺὒὓ]").unwrap();
    if re.is_match(&contents) {
        false
    } else {
        true
    }
}

pub fn parse_file(path: &str) -> Result<Vec<Verb>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let mut entries: Vec<Verb> = Vec::new();
    for result in rdr.deserialize() {
        let verb_entry: VerbEntry = result?;
        // println!("Read: {:?}", verb_entry);
        let verb_alternates: VerbAlternates = verb_entry.check_alternates();
        // println!("With alts: {:?}", verb_alternates);
        let verb_stem_set: VerbStemSet = verb_alternates.parse();
        // println!("Test: {:?}", verb_stem_set);
        let verb: Verb = verb_stem_set.conjugate();
        // println!("Conjugated: {:?}", verb);
        entries.push(verb);
    }
    Ok(entries)
}
