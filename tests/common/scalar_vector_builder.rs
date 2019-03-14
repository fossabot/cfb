pub mod example {
    #![allow(unused_imports)]

    use cfb::builder::{
        Builder, Component, DesignatedComponent, NestedBufferComponent, ReferenceVectorComponent,
        ScalarVectorComponent, StringComponent,
    };
    use cfb::scalar::Scalar;
    use cfb::types::{SOffset, SIZE_OF_SOFFSET};
    #[cfg(not(target_endian = "little"))]
    use std::mem::transmute;

    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct Sensor {
        pub readings: Vec<u32>,
    }

    impl Sensor {
        const VT_READINGS: usize = 4;
        const SIZE_READINGS: usize = 4;
        const ALIGNMENT_READINGS: usize = 4;
        const ALIGNMENT: usize = 4;
    }

    impl<'c> Component<'c> for Sensor {
        fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if !self.readings.is_empty() {
                    vtable.add_field(Self::VT_READINGS, Self::SIZE_READINGS, Self::ALIGNMENT_READINGS);
                }
                vtable.finish()
            };

            builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

            let table_start = builder.tell();
            builder.push_scalar((table_start - vtable_start) as SOffset);
            if !self.readings.is_empty() {
                builder.align(Self::ALIGNMENT_READINGS);
                let offset_position = builder.tell();
                builder.pad(Self::SIZE_READINGS);
                builder.push_component(DesignatedComponent::new(
                    offset_position,
                    Box::new(ScalarVectorComponent::new(self.readings, 4)),
                ));
            }

            table_start
        }
    }
}