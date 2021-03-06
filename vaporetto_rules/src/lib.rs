//! # vaporetto_rules
//!
//! Rule base filters for Vaporetto.
//!
//! ## Examples
//!
//! ```no_run
//! use std::fs::File;
//! use std::io::BufReader;
//!
//! use vaporetto::{CharacterType, Model, Predictor, Sentence};
//! use vaporetto_rules::{
//!     SentenceFilter, StringFilter,
//!     sentence_filters::{ConcatGraphemeClustersFilter, KyteaWsConstFilter},
//!     string_filters::KyteaFullwidthFilter,
//! };
//!
//! let mut f = BufReader::new(File::open("model.bin").unwrap());
//! let model = Model::read(&mut f).unwrap();
//! let mut predictor = Predictor::new(model);
//!
//! let pre_filters: Vec<Box<dyn StringFilter<String>>> = vec![
//!     Box::new(KyteaFullwidthFilter::new()),
//! ];
//! let post_filters: Vec<Box<dyn SentenceFilter>> = vec![
//!     Box::new(ConcatGraphemeClustersFilter::new()),
//!     Box::new(KyteaWsConstFilter::new(CharacterType::Digit)),
//! ];
//!
//! let input = "Vaporettoは仲良し家族👨‍👨‍👧‍👦を離れ離れにさせません。"
//!     .to_string();
//!
//! let preproc_input = pre_filters.iter().fold(input, |s, filter| filter.filter(s));
//!
//! let sentence = Sentence::from_raw(preproc_input).unwrap();
//! let sentence = predictor.predict(sentence);
//!
//! let postproc_result = post_filters.iter().fold(sentence, |s, filter| filter.filter(s));
//!
//! assert_eq!(
//!     "Ｖａｐｏｒｅｔｔｏ は 仲良 し 家族 👨‍👨‍👧‍👦 を 離れ離れ に さ せ ま せ ん 。",
//!     postproc_result.to_tokenized_string().unwrap(),
//! );
//! ```
//!

pub mod sentence_filters;
pub mod string_filters;

use vaporetto::Sentence;

pub trait SentenceFilter {
    /// Filter a specified sentence using rules.
    ///
    /// # Arguments:
    ///
    /// * `sentence` - Input sentence.
    ///
    /// # Returns
    ///
    /// A processed sentence.
    fn filter(&self, sentence: Sentence) -> Sentence;
}

pub trait StringFilter<S>
where
    S: AsRef<str>,
{
    /// Filter a specified string using rules.
    ///
    /// # Arguments:
    ///
    /// * `string` - Input string.
    ///
    /// # Returns
    ///
    /// A processed string.
    fn filter(&self, string: S) -> String;
}
