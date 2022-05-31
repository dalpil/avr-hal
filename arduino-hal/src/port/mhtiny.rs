pub use attiny_hal::port::mode;
pub use attiny_hal::port::Pin;

avr_hal_generic::renamed_pins! {
    type Pin = Pin;

    pub struct Pins from attiny_hal::Pins {
        pub a0: attiny_hal::port::PC0 = pc0,
        pub a1: attiny_hal::port::PC1 = pc1,
        pub a2: attiny_hal::port::PC2 = pc2,
        pub a3: attiny_hal::port::PC3 = pc3,
        pub a4: attiny_hal::port::PC4 = pc4,
        pub a5: attiny_hal::port::PC5 = pc5,
        pub a6: attiny_hal::port::PA0 = pa0,
        pub a7: attiny_hal::port::PA1 = pa1,

        pub d0: attiny_hal::port::PD0 = pd0,
        pub d1: attiny_hal::port::PD1 = pd1,
        pub d2: attiny_hal::port::PD2 = pd2,
        pub d3: attiny_hal::port::PD3 = pd3,
        pub d4: attiny_hal::port::PD4 = pd4,
        pub d5: attiny_hal::port::PD5 = pd5,
        pub d6: attiny_hal::port::PD6 = pd6,
        pub d7: attiny_hal::port::PD7 = pd7,

        pub d8: attiny_hal::port::PB0 = pb0,
        pub d9: attiny_hal::port::PB1 = pb1,
        pub d10: attiny_hal::port::PB2 = pb2,
        pub d11: attiny_hal::port::PB3 = pb3,
        pub d12: attiny_hal::port::PB4 = pb4,
        pub d13: attiny_hal::port::PB5 = pb5,
        pub d14: attiny_hal::port::PB7 = pb7,
        pub d15: attiny_hal::port::PA2 = pa2,

        pub d16: attiny_hal::port::PA3 = pa3,

        pub d25: attiny_hal::port::PC7 = pc7,
    }
}