#![no_std]
use ai_io::*;
use gstd::{exec, msg, prelude::*};
use lazy_static::*;
use serde_json::Value;

gstd::metadata! {
    title: "SmartCube AI",
    handle:
        input: AIAction,
        output: AIOutput,
    state:
      output: u32,
}

#[derive(Default)]
pub struct AI {
    pub last_update: u32,
}

static mut ARTIFICIAL: Option<AI> = None;

impl AI {
    pub fn set_last_update(&mut self, last_update: u32) {
        self.last_update = last_update;
        msg::reply(AIOutput::AIUpdated, 0).expect("Error in reply to message IAOutput::UpdateAI");
    }
}

pub fn analyze_ai(text: &str) {
    msg::reply(analyze(text).score, 0).expect("Error in reply to message IAOutput::UpdateAI");
}

#[no_mangle]
extern "C" fn init() {
    let mut ai: AI = Default::default();
    ai.set_last_update(exec::block_height());
    unsafe { ARTIFICIAL = Some(ai) };
}

#[gstd::async_main]
async unsafe fn main() {
    let ai = unsafe { ARTIFICIAL.get_or_insert(Default::default()) };
    let action: AIAction =
        msg::load().unwrap_or_else(|_| panic!("The AI Unable to decode AI Action"));

    match action {
        AIAction::UpdateAI { contract: _ } => ai.set_last_update(exec::block_height()),
        AIAction::AnalyzeAI { text } => analyze_ai(&text),
    }
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let ai = ARTIFICIAL.get_or_insert(Default::default());
    let last_update: u32 = ai.last_update.clone();
    let encoded = last_update.encode();
    gstd::util::to_leak_ptr(encoded)
}

// include the json in the bin
const AFFIN: &[u8; 32811] = include_bytes!("./afinn.json");

lazy_static! {
    static ref AFFIN_VALUE: Value = {
        let json = String::from_utf8(AFFIN.to_vec()).unwrap();
        serde_json::from_str(&json).unwrap()
    };
}

/// Struct for return the outcome of individual sentiments
pub struct Sentiment {
    /// The sentiment score
    pub score: f32,
    /// The score compared with total tokens analysed
    pub comparative: f32,
    /// The matching set of words
    pub words: Vec<String>,
}

/// Struct for return the outcome of analysis
pub struct Analysis {
    /// The sentiment score
    pub score: f32,
    /// The score compared with total tokens analysed
    pub comparative: f32,
    /// Positivity score
    pub positive: Sentiment,
    /// Negativity score
    pub negative: Sentiment,
}

fn tokenize_with_no_punctuation(phrase: &str) -> Vec<String> {
    phrase
        .to_lowercase()
        .split(' ')
        .map(|s| s.to_string())
        .collect()
}

/// Calculates the negativity of a sentence
pub fn negativity(phrase: &str) -> Sentiment {
    let tokens = tokenize_with_no_punctuation(phrase);
    let tokens_len = tokens.len() as f32;
    let mut score = 0f32;
    let mut words = Vec::new();

    for t in tokens {
        let word = t.clone();
        if let Value::Number(ref val) = AFFIN_VALUE[t] {
            let diff = val.as_f64().unwrap() as f32;
            if diff < 0f32 {
                score -= diff;
                words.push(word);
            }
        }
    }

    Sentiment {
        score,
        comparative: score / tokens_len,
        words,
    }
}

/// Calculates the positivity of a sentence
pub fn positivity(phrase: &str) -> Sentiment {
    let tokens = tokenize_with_no_punctuation(phrase);
    let tokens_len = tokens.len() as f32;
    let mut score = 0f32;
    let mut words = Vec::new();

    for t in tokens {
        let word = t.clone();
        if let Value::Number(ref val) = AFFIN_VALUE[t] {
            let diff = val.as_f64().unwrap() as f32;
            if diff > 0f32 {
                score += diff;
                words.push(word);
            }
        }
    }

    Sentiment {
        score,
        comparative: score / tokens_len,
        words,
    }
}

/// Calculates the overall sentiment
pub fn analyze(phrase: &str) -> Analysis {
    let neg = negativity(phrase);
    let pos = positivity(phrase);

    Analysis {
        score: pos.score - neg.score,
        comparative: pos.comparative - neg.comparative,
        positive: pos,
        negative: neg,
    }
}
