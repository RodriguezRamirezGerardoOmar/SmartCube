#![no_std]
use codec::{Decode, Encode};
use contract::*;
use gstd::prelude::*;
use scale_info::TypeInfo;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum AIAction {
    UpdateAI { contract: Contract },
    AnalyzeAI { text: String },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum AIOutput {
    AIUpdated,
    AIAnalyzed,
}
