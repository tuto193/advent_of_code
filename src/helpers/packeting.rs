use std::{cell::RefCell, rc::Rc};

type OtherPacketEntries = Rc<RefCell<PackEntry>>;

enum EntryType {
    Int(isize),
    List,
}

struct PackEntry {
    entry_type: EntryType,
    list: Vec<Option<OtherPacketEntries>>,
}

impl PackEntry {

}
