#![allow(unused_imports)]

use cfb::builder::{
    Builder, Component, DesignatedComponent, NestedBufferComponent, ReferenceVectorComponent,
    ScalarVectorComponent, StringComponent,
};
use cfb::scalar::Scalar;
use cfb::types::{SOffset, SIZE_OF_SOFFSET};
#[cfg(not(target_endian = "little"))]
use std::mem::transmute;

#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl Default for Color {
    fn default() -> Self {
        Color::Red
    }
}

impl Scalar for Color {
    fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(not(target_endian = "little"))]
        {
            unsafe { transmute((self as i8).swap_bytes()) }
        }
    }

    fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(not(target_endian = "little"))]
        {
            unsafe { transmute((x as i8).swap_bytes()) }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Result {
    Ok(Ok),
    Err(Err),
}

impl Result {
    pub fn union_type(&self) -> u8 {
        match self {
            Result::Ok(_) => 1,
            Result::Err(_) => 2,
        }
    }
}

#[repr(C, align(8))]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Complex {
    pub a: u64,
    pub b: u64,
}

impl Complex {
    pub fn is_present(&self) -> bool {
        self.a != 0u64 || self.b != 0u64
    }
}

impl Scalar for Complex {
    #[cfg(target_endian = "little")]
    fn to_le(self) -> Self {
        self
    }

    #[cfg(target_endian = "little")]
    fn from_le(x: Self) -> Self {
        x
    }

    #[cfg(not(target_endian = "little"))]
    fn to_le(mut self) -> Self {
        self.a = self.a.to_le();
        self.b = self.b.to_le();
        self
    }

    #[cfg(not(target_endian = "little"))]
    fn from_le(mut x: Self) -> Self {
        x.a = Scalar::from_le(x.a);
        x.b = Scalar::from_le(x.b);
        x
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Err {
    pub reason: String,
}

impl Err {
    const VT_REASON: usize = 4;
    const SIZE_REASON: usize = 4;
    const ALIGNMENT_REASON: usize = 4;
    const ALIGNMENT: usize = 4;
}

impl<'c> Component<'c> for Err {
    fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
        let vtable_start = {
            let mut vtable = builder.start_vtable();
            if !self.reason.is_empty() {
                vtable.add_field(Self::VT_REASON, Self::SIZE_REASON, Self::ALIGNMENT_REASON);
            }
            vtable.finish()
        };

        builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

        let table_start = builder.tell();
        builder.push_scalar((table_start - vtable_start) as SOffset);
        if !self.reason.is_empty() {
            builder.align(Self::ALIGNMENT_REASON);
            let offset_position = builder.tell();
            builder.pad(Self::SIZE_REASON);
            builder.push_component(DesignatedComponent::new(
                offset_position,
                Box::new(StringComponent::new(self.reason))
            ));
        }

        table_start
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Ok {
    pub value: u32,
}

impl Ok {
    const VT_VALUE: usize = 4;
    const SIZE_VALUE: usize = 4;
    const ALIGNMENT_VALUE: usize = 4;
    const ALIGNMENT: usize = 4;
}

impl<'c> Component<'c> for Ok {
    fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
        let vtable_start = {
            let mut vtable = builder.start_vtable();
            if self.value != 0u32 {
                vtable.add_field(Self::VT_VALUE, Self::SIZE_VALUE, Self::ALIGNMENT_VALUE);
            }
            vtable.finish()
        };

        builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

        let table_start = builder.tell();
        builder.push_scalar((table_start - vtable_start) as SOffset);
        if self.value != 0u32 {
            builder.align(Self::ALIGNMENT_VALUE);
            builder.push_scalar(self.value);
        }

        table_start
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct T {
    pub a_ubyte: u8,
    pub complex: Complex,
    pub a_uint32: u32,
    pub result: Option<Result>,
    pub a_uint64: u64,
    pub uint16_array: Vec<u16>,
    pub color: Color,
}

impl T {
    const VT_A_UBYTE: usize = 4;
    const SIZE_A_UBYTE: usize = 1;
    const ALIGNMENT_A_UBYTE: usize = 1;
    const VT_COMPLEX: usize = 6;
    const SIZE_COMPLEX: usize = 16;
    const ALIGNMENT_COMPLEX: usize = 8;
    const VT_A_UINT32: usize = 8;
    const SIZE_A_UINT32: usize = 4;
    const ALIGNMENT_A_UINT32: usize = 4;
    const VT_RESULT_TYPE: usize = 10;
    const SIZE_RESULT_TYPE: usize = 1;
    const ALIGNMENT_RESULT_TYPE: usize = 1;
    const VT_RESULT: usize = 12;
    const SIZE_RESULT: usize = 4;
    const ALIGNMENT_RESULT: usize = 4;
    const VT_A_UINT64: usize = 14;
    const SIZE_A_UINT64: usize = 8;
    const ALIGNMENT_A_UINT64: usize = 8;
    const VT_UINT16_ARRAY: usize = 16;
    const SIZE_UINT16_ARRAY: usize = 4;
    const ALIGNMENT_UINT16_ARRAY: usize = 4;
    const VT_COLOR: usize = 18;
    const SIZE_COLOR: usize = 1;
    const ALIGNMENT_COLOR: usize = 1;
    const ALIGNMENT: usize = 8;
}

impl<'c> Component<'c> for T {
    fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
        let vtable_start = {
            let mut vtable = builder.start_vtable();
            if self.complex.is_present() {
                vtable.add_field(Self::VT_COMPLEX, Self::SIZE_COMPLEX, Self::ALIGNMENT_COMPLEX);
            }
            if self.a_uint64 != 0u64 {
                vtable.add_field(Self::VT_A_UINT64, Self::SIZE_A_UINT64, Self::ALIGNMENT_A_UINT64);
            }
            if self.a_uint32 != 0u32 {
                vtable.add_field(Self::VT_A_UINT32, Self::SIZE_A_UINT32, Self::ALIGNMENT_A_UINT32);
            }
            if self.result.is_some() {
                vtable.add_field(Self::VT_RESULT, Self::SIZE_RESULT, Self::ALIGNMENT_RESULT);
            }
            if !self.uint16_array.is_empty() {
                vtable.add_field(Self::VT_UINT16_ARRAY, Self::SIZE_UINT16_ARRAY, Self::ALIGNMENT_UINT16_ARRAY);
            }
            if self.a_ubyte != 0u8 {
                vtable.add_field(Self::VT_A_UBYTE, Self::SIZE_A_UBYTE, Self::ALIGNMENT_A_UBYTE);
            }
            if self.result.is_some() {
                vtable.add_field(Self::VT_RESULT_TYPE, Self::SIZE_RESULT_TYPE, Self::ALIGNMENT_RESULT_TYPE);
            }
            if self.color != Color::Red {
                vtable.add_field(Self::VT_COLOR, Self::SIZE_COLOR, Self::ALIGNMENT_COLOR);
            }
            vtable.finish()
        };

        builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

        let table_start = builder.tell();
        builder.push_scalar((table_start - vtable_start) as SOffset);
        if self.complex.is_present() {
            builder.align(Self::ALIGNMENT_COMPLEX);
            builder.push_scalar(self.complex);
        }
        if self.a_uint64 != 0u64 {
            builder.align(Self::ALIGNMENT_A_UINT64);
            builder.push_scalar(self.a_uint64);
        }
        if self.a_uint32 != 0u32 {
            builder.align(Self::ALIGNMENT_A_UINT32);
            builder.push_scalar(self.a_uint32);
        }
        let result_type = self.result.as_ref().map(|v| v.union_type());
        if let Some(f) = self.result {
            builder.align(Self::ALIGNMENT_RESULT);
            let offset_position = builder.tell();
            builder.pad(Self::SIZE_RESULT);
            let component: Box<dyn Component<'c> + 'c> = match f {
                Result::Ok(v) => Box::new(v),
                Result::Err(v) => Box::new(v),
            };
            builder.push_component(DesignatedComponent::new(offset_position, component));
        }
        if !self.uint16_array.is_empty() {
            builder.align(Self::ALIGNMENT_UINT16_ARRAY);
            let offset_position = builder.tell();
            builder.pad(Self::SIZE_UINT16_ARRAY);
            builder.push_component(DesignatedComponent::new(
                offset_position,
                Box::new(ScalarVectorComponent::new(self.uint16_array, 2)),
            ));
        }
        if self.a_ubyte != 0u8 {
            builder.align(Self::ALIGNMENT_A_UBYTE);
            builder.push_scalar(self.a_ubyte);
        }
        if let Some(f) = result_type {
            builder.align(Self::ALIGNMENT_RESULT_TYPE);
            builder.push_scalar(f);
        }
        if self.color != Color::Red {
            builder.align(Self::ALIGNMENT_COLOR);
            builder.push_scalar(self.color);
        }

        table_start
    }
}