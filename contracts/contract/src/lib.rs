#![no_std]
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Clone, Debug, Encode, Decode, TypeInfo, Default, PartialEq, Eq)]
pub struct Contract {
    pub id: ActorId,
    pub autor: ActorId,
    pub program: String,
    pub description: Description,
    pub rate: i32,
    pub audited: bool,
    pub label: String,
}

impl Contract {
    pub fn new(
        id: ActorId,
        autor: ActorId,
        program: String,
        description: Description,
        rate: i32,
        audited: bool,
        label: String,
    ) -> Self {
        Self {
            id,
            autor,
            program,
            description,
            rate,
            audited,
            label,
        }
    }
    pub fn set_id(&mut self, id: ActorId) {
        self.id = id;
    }
    pub fn set_rate(&mut self, rate: i32) {
        self.rate = rate;
    }
    pub fn set_audited(&mut self, audited: bool) {
        self.audited = audited;
    }
    pub fn set_auditor_description(&mut self, auditor_description: String) {
        self.description
            .set_auditor_description(auditor_description);
    }
    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo, Default, PartialEq, Eq)]
pub struct Description {
    pub owner_description: String,
    pub auditor_description: String,
}

impl Description {
    pub fn set_auditor_description(&mut self, auditor_description: String) {
        self.auditor_description = auditor_description;
    }
}
