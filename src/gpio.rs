//! General Purpose Input / Output

#[derive(PartialEq)]
pub enum PWMPins {
    // These are all Peripheral B on the A/B Select
    PA0, // PWML3
    // PA3, // PWMFI1
    // PA5, // PWMFI0
    PA8, // PWMH0
    PA9, // PWMH3
    PA12, // PWML1
    PA13, // PWMH2
    PA19, // PWMH1
    PA20, // PWML2
    PA21, // PWML0
    PB6, // PWML4
    PB7, //PWML5
    PB8, // PWML6
    PB9, // PWML7
    PB12, // PWMH0
    PB13, // PWMH1
    PB14, // PWMH2,
    PB15, // PWMH3
    PB16, // PWML0
    PB17, // PWML1
    PB18, // PWML2
    PB19, // PWML3
    PC2, // PWML0
    PC3, // PWMH0
    PC4, // PWML1
    PC5, // PWMH1
    PC6, // PWML2
    PC7, // PWMH2
    PC8, // PWML3
    PC9, // PWMH3
    PC18, // PWMH6
    PC19, // PWMH5
    PC20, // PWMH4
    PC21, // PWML4
    PC22, // PWML5
    PC23, // PWML6
    PC24, // PWML7
    // PD6, // PWMFI2
}
pub enum Pin {
    PA0,
    PA1,
    PA2,
    PA3,
    PA4,
    PA5,
    PA6,
    PA7,
    PA8,
    PA9,
    PA10,
    PA11,
    PA12,
    PA13,
    PA14,
    PA15,
    PA16,
    PA17,
    PA18,
    PA19,
    PA20,
    PA21,
    PA22,
    PA23,
    PA24,
    PA25,
    PA26,
    PA27,
    PA28,
    PA29,
    PB0,
    PB1,
    PB2,
    PB3,
    PB4,
    PB5,
    PB6,
    PB7,
    PB8,
    PB9,
    PB10,
    PB11,
    PB12,
    PB13,
    PB14,
    PB15,
    PB16,
    PB17,
    PB18,
    PB19,
    PB20,
    PB21,
    PB22,
    PB23,
    PB24,
    PB25,
    PB26,
    PB27,
    PB28,
    PB29,
    PB30,
    PB31,
    PC0,
    PC1,
    PC2,
    PC3,
    PC4,
    PC5,
    PC6,
    PC7,
    PC8,
    PC9,
    PC10,
    PC11,
    PC12,
    PC13,
    PC14,
    PC15,
    PC16,
    PC17,
    PC18,
    PC19,
    PC20,
    PC21,
    PC22,
    PC23,
    PC24,
    PC25,
    PC26,
    PC27,
    PC28,
    PC29,
    PC30,
    PD0,
    PD1,
    PD2,
    PD3,
    PD4,
    PD5,
    PD6,
    PD7,
    PD8,
    PD9,
}
