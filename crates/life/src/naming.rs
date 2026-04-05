//! Order-2 Markov-chain name generator.
//!
//! Trains on bundled phonology corpora and produces novel names. Output is
//! deterministic from a seeded RNG, so a given `(seed, style, index)` tuple
//! always yields the same name. Useful for settlements, dynasties, and
//! named geographic features.

use seeded_dice_roller::SeededDiceRoller;
use std::collections::HashMap;

const START: char = '^';
const END: char = '$';

/// Pre-baked phonology style — picks which corpus to train on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NameStyle {
    /// Fantasy human names: Latin/Gothic medieval-ish phonology.
    FantasyHuman,
    /// Dwarvish: consonant-heavy, guttural.
    Dwarvish,
    /// Elvish: vowel-rich, flowing, multi-syllable.
    Elvish,
    /// Norse / Scandinavian saga names.
    Norse,
    /// Generic alien: unusual consonant clusters.
    Alien,
}

/// An order-2 Markov chain keyed by consecutive character pairs.
#[derive(Clone, Debug)]
pub struct MarkovNameGen {
    /// Maps 2-char context → possible next chars (weighted by frequency).
    transitions: HashMap<(char, char), Vec<char>>,
}

impl MarkovNameGen {
    /// Train on a slice of lowercase words.
    pub fn train(corpus: &[&str]) -> Self {
        let mut transitions: HashMap<(char, char), Vec<char>> = HashMap::new();
        for word in corpus {
            let padded: Vec<char> = std::iter::once(START)
                .chain(std::iter::once(START))
                .chain(word.chars().filter(|c| c.is_ascii_alphabetic()))
                .chain(std::iter::once(END))
                .collect();
            for window in padded.windows(3) {
                let key = (window[0], window[1]);
                transitions.entry(key).or_default().push(window[2]);
            }
        }
        Self { transitions }
    }

    /// Build a generator for the given style using the bundled corpus.
    pub fn for_style(style: NameStyle) -> Self {
        Self::train(bundled_corpus(style))
    }

    /// Generate one name, deterministic on the RNG state. Retries up to 8
    /// times to find a name within the length bounds.
    pub fn generate(&self, rng: &mut SeededDiceRoller, min_len: usize, max_len: usize) -> String {
        for _ in 0..8 {
            let mut result = String::new();
            let mut prev = (START, START);
            for _ in 0..max_len {
                let options = match self.transitions.get(&prev) {
                    Some(o) if !o.is_empty() => o,
                    _ => break,
                };
                let idx = (rng.gen_usize()) % options.len();
                let next = options[idx];
                if next == END {
                    break;
                }
                result.push(next);
                prev = (prev.1, next);
            }
            if result.len() >= min_len {
                return capitalize(&result);
            }
        }
        // Fallback: return whatever we last got even if it's short.
        capitalize("unnamed")
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => {
            let mut out = String::with_capacity(s.len());
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
            out
        }
        None => String::new(),
    }
}

fn bundled_corpus(style: NameStyle) -> &'static [&'static str] {
    match style {
        NameStyle::FantasyHuman => FANTASY_HUMAN,
        NameStyle::Dwarvish => DWARVISH,
        NameStyle::Elvish => ELVISH,
        NameStyle::Norse => NORSE,
        NameStyle::Alien => ALIEN,
    }
}

const FANTASY_HUMAN: &[&str] = &[
    "alden",
    "baldric",
    "cedric",
    "darian",
    "edric",
    "fergus",
    "gareth",
    "harold",
    "ignatius",
    "jareth",
    "korin",
    "lionel",
    "maren",
    "nolan",
    "orrin",
    "percival",
    "quinn",
    "rowan",
    "sebastian",
    "theron",
    "ulric",
    "vance",
    "wynn",
    "ysolde",
    "zephyrus",
    "aldric",
    "benedict",
    "cassian",
    "dorian",
    "evander",
    "florian",
    "gilbert",
    "horatio",
    "isidore",
    "jerome",
    "lothair",
    "magnus",
    "nikolas",
    "othmar",
    "perrin",
    "radric",
    "silvio",
    "tristan",
    "ulysses",
    "valerius",
    "wilhelm",
    "xander",
    "yorick",
    "zaden",
    "aedric",
    "bastian",
    "cyrus",
];

const DWARVISH: &[&str] = &[
    "thorin",
    "balin",
    "dwalin",
    "gimli",
    "durin",
    "borin",
    "nori",
    "dori",
    "ori",
    "oin",
    "gloin",
    "bifur",
    "bofur",
    "bombur",
    "kili",
    "fili",
    "thrain",
    "thror",
    "grimli",
    "dain",
    "grumdar",
    "kazthor",
    "bruenor",
    "hargrim",
    "tordek",
    "khaldrim",
    "morgrim",
    "baerdin",
    "drazkul",
    "karrik",
    "urist",
    "khadzim",
    "throkk",
    "groznor",
    "tharak",
    "burim",
    "gondin",
    "moradin",
    "stonefist",
    "orgrim",
    "krunzor",
    "halvar",
    "durkhar",
    "skarnil",
    "tharnok",
];

const ELVISH: &[&str] = &[
    "aelar",
    "aerin",
    "anaris",
    "caelynn",
    "eladrin",
    "galanodel",
    "haerandir",
    "ilanis",
    "koehlan",
    "lorendir",
    "mindartis",
    "nailo",
    "paelias",
    "quilathe",
    "riardon",
    "silvyr",
    "thamior",
    "uthemar",
    "virdan",
    "xantholen",
    "yelenor",
    "aramil",
    "arannis",
    "berrian",
    "dayereth",
    "enna",
    "galinndan",
    "hadarai",
    "immeral",
    "ivellios",
    "laucian",
    "lucan",
    "nutae",
    "peren",
    "quarion",
    "soveliss",
    "suhnaal",
    "theriatis",
    "therivol",
    "uthemar",
    "vanuath",
    "aeolyn",
    "caelith",
    "elendil",
    "firiel",
    "gilinwen",
    "lindir",
    "nessa",
];

const NORSE: &[&str] = &[
    "ragnar", "bjorn", "erik", "leif", "gunnar", "haakon", "ivar", "sigurd", "thorvald", "ulf",
    "vidar", "hrothgar", "olaf", "torsten", "arvid", "sven", "knut", "frode", "harald", "jorund",
    "kettil", "magnus", "njord", "odin", "thor", "tyr", "freyr", "balder", "loki", "heimdall",
    "baldr", "vali", "vidar", "forseti", "bragi", "hoder", "magni", "modi", "ull", "freja",
    "aslaug", "astrid", "sigrid", "ragnhild", "thyra", "helga", "ingrid",
];

const ALIEN: &[&str] = &[
    "zxyth", "kaerix", "xorvath", "qilax", "vrenth", "pzhor", "grexis", "thrax", "zylox", "oomrag",
    "kzzal", "vryxx", "qtharr", "nhyl", "phssk", "gzeth", "kroznak", "xxolith", "vrazzak", "qyrn",
    "zzhaal", "grymn", "drexil", "slyxn", "vgharr", "tzhek", "pnxoth", "lyxxa", "kthun", "rrazeth",
    "mpyrr", "tzol", "vxorn", "qqeln", "khssh", "rnyxal", "zzulth", "baragok", "shrykk", "gwxen",
    "ythra", "tlar", "nzoth",
];

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_many(style: NameStyle, seed: &str, count: usize) -> Vec<String> {
        let gen = MarkovNameGen::for_style(style);
        let mut rng = SeededDiceRoller::new(seed, "naming");
        (0..count).map(|_| gen.generate(&mut rng, 4, 12)).collect()
    }

    #[test]
    fn all_styles_produce_names() {
        for style in [
            NameStyle::FantasyHuman,
            NameStyle::Dwarvish,
            NameStyle::Elvish,
            NameStyle::Norse,
            NameStyle::Alien,
        ] {
            let names = gen_many(style, "seed", 10);
            for name in &names {
                assert!(!name.is_empty(), "empty name from {:?}", style);
                assert!(name.chars().next().unwrap().is_uppercase());
            }
        }
    }

    #[test]
    fn names_respect_length_bounds() {
        let gen = MarkovNameGen::for_style(NameStyle::FantasyHuman);
        let mut rng = SeededDiceRoller::new("seed", "len");
        for _ in 0..50 {
            let name = gen.generate(&mut rng, 4, 12);
            assert!(name.len() >= 4 || name == "Unnamed");
            assert!(name.len() <= 12);
        }
    }

    #[test]
    fn same_seed_produces_same_sequence() {
        let a = gen_many(NameStyle::Elvish, "repeat", 20);
        let b = gen_many(NameStyle::Elvish, "repeat", 20);
        assert_eq!(a, b);
    }

    #[test]
    fn different_seeds_diverge() {
        let a = gen_many(NameStyle::FantasyHuman, "alpha", 20);
        let b = gen_many(NameStyle::FantasyHuman, "beta", 20);
        let identical = a.iter().zip(b.iter()).filter(|(x, y)| x == y).count();
        // Expect most names to differ.
        assert!(identical < 5, "too many identical names: {}", identical);
    }

    #[test]
    fn diversity_in_100_samples() {
        let gen = MarkovNameGen::for_style(NameStyle::FantasyHuman);
        let mut rng = SeededDiceRoller::new("diversity", "naming");
        let mut seen = std::collections::HashSet::new();
        for _ in 0..100 {
            seen.insert(gen.generate(&mut rng, 4, 12));
        }
        // At least 60 unique names out of 100 attempts.
        assert!(
            seen.len() >= 60,
            "too many duplicates: {}",
            100 - seen.len()
        );
    }

    #[test]
    fn corpus_words_are_alphabetic() {
        for style in [
            NameStyle::FantasyHuman,
            NameStyle::Dwarvish,
            NameStyle::Elvish,
            NameStyle::Norse,
            NameStyle::Alien,
        ] {
            for word in bundled_corpus(style) {
                assert!(
                    word.chars().all(|c| c.is_ascii_lowercase()),
                    "corpus {:?} has non-alphabetic: {:?}",
                    style,
                    word
                );
            }
        }
    }

    #[test]
    fn training_on_empty_corpus_handles_gracefully() {
        let gen = MarkovNameGen::train(&[]);
        let mut rng = SeededDiceRoller::new("empty", "naming");
        let name = gen.generate(&mut rng, 4, 12);
        // Falls back to "Unnamed".
        assert_eq!(name, "Unnamed");
    }

    #[test]
    fn fantasy_human_looks_different_from_alien() {
        // Count vowels — alien should be dramatically lower.
        let human = gen_many(NameStyle::FantasyHuman, "style", 50).join("");
        let alien = gen_many(NameStyle::Alien, "style", 50).join("");
        let vowel_ratio = |s: &str| {
            let vs = s.chars().filter(|c| "aeiouAEIOU".contains(*c)).count() as f32;
            vs / s.len().max(1) as f32
        };
        assert!(
            vowel_ratio(&human) > vowel_ratio(&alien),
            "human vowel ratio {} should exceed alien {}",
            vowel_ratio(&human),
            vowel_ratio(&alien)
        );
    }
}
