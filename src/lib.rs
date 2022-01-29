#![no_std]
#![doc = include_str!("../README.md")]

/// Extension trait offering EquisizedPrimitiveSignedInt type.
/// 
/// * `<u8 as EquisizedPrimitiveSignedIntExt>::EquisizedPrimitiveSignedInt == i8`;
/// * `<i16 as EquisizedPrimitiveSignedIntExt>::EquisizedPrimitiveSignedInt == i16`;
/// * ...
pub trait EquisizedPrimitiveSignedIntExt {
    type EquisizedPrimitiveSignedInt;
}

impl EquisizedPrimitiveSignedIntExt for u8 {
    type EquisizedPrimitiveSignedInt = i8;
}

impl EquisizedPrimitiveSignedIntExt for u16 {
    type EquisizedPrimitiveSignedInt = i16;
}

impl EquisizedPrimitiveSignedIntExt for u32 {
    type EquisizedPrimitiveSignedInt = i32;
}

impl EquisizedPrimitiveSignedIntExt for u64 {
    type EquisizedPrimitiveSignedInt = i64;
}

impl EquisizedPrimitiveSignedIntExt for u128 {
    type EquisizedPrimitiveSignedInt = i128;
}

impl EquisizedPrimitiveSignedIntExt for usize {
    type EquisizedPrimitiveSignedInt = isize;
}

impl EquisizedPrimitiveSignedIntExt for i8 {
    type EquisizedPrimitiveSignedInt = i8;
}

impl EquisizedPrimitiveSignedIntExt for i16 {
    type EquisizedPrimitiveSignedInt = i16;
}

impl EquisizedPrimitiveSignedIntExt for i32 {
    type EquisizedPrimitiveSignedInt = i32;
}

impl EquisizedPrimitiveSignedIntExt for i64 {
    type EquisizedPrimitiveSignedInt = i64;
}

impl EquisizedPrimitiveSignedIntExt for i128 {
    type EquisizedPrimitiveSignedInt = i128;
}

impl EquisizedPrimitiveSignedIntExt for isize {
    type EquisizedPrimitiveSignedInt = isize;
}