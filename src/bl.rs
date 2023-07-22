use binary_layout::prelude::*;

define_layout!(string_ref, BigEndian, {
    start: u64,
    length: u64
});

define_layout!(token, BigEndian, {
    id: u128,
    document_id: u128,
    author_id: u128,
    line: u64,
    position: u64,
    text: crate::string_ref::NestedView,
    attributes: [u8; 64]
});

define_layout!(document, BigEndian, {
    id: u128,
    author_id: u128,
    collection_id: u128,
    date: u64,
    title: crate::string_ref::NestedView,
});

define_layout!(collection, BigEndian, {
    id: u128,
    date: u64,
    title: crate::string_ref::NestedView,
    notes: crate::string_ref::NestedView,
});
