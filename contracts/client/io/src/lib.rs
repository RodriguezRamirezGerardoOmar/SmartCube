#![no_std]
use codec::{Decode, Encode};
use contract::*;
use gstd::{prelude::String, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum ClientAction {
    IDRecived { id: ActorId },
}
