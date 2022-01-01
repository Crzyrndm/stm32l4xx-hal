use stm32l4::stm32l4x6::{LPTIM1, TIM16, TIM17, TIM8};

use super::*;

// AF0 == SYS_AF
// AF1 == TIM1 / TIM2 / TIM5 / TIM8 / LPTIM1
// AF2 == TIM1 / TIM2 / TIM3 / TIM5
// AF3 == TIM8
// AF4 == I2C1 / I2C2 / I2C3
// AF5 == SPI1 / SPI2
// AF6 == SPI3 / DFSDM
// AF7 == USART1 / USART2 / USART3
// AF8 == UART4 / UART5 / LPUART1
// AF9 == CAN1 / TSC
// AF10 == OTG_FS / QUADSPI
// AF11 == LCD
// AF12 == SDMMC1 / COMP1 / COMP2 / FMC / SWPMI1
// AF13 == SAI1 / SAI2
// AF14 == TIM2 / TIM15 / TIM16 / TIM17 / LPTIM2
// AF15 == EVENTOUT

macro_rules! idx_from_AFx {
    (AF0) => {
        0
    };
    (AF1) => {
        1
    };
    (AF2) => {
        2
    };
    (AF3) => {
        3
    };
    (AF4) => {
        4
    };
    (AF5) => {
        5
    };
    (AF6) => {
        6
    };
    (AF7) => {
        7
    };
    (AF8) => {
        8
    };
    (AF9) => {
        9
    };
    (AF10) => {
        10
    };
    (AF11) => {
        11
    };
    (AF12) => {
        12
    };
    (AF13) => {
        13
    };
    (AF14) => {
        14
    };
    (AF15) => {{
        15
    }};
}

macro_rules! map_af {
    ($(($af:ident = [ $(($pin:ident, $trait:ty),)+]),)*) => {
        $(
            $(
                impl<OTYPE> private::Sealed for $pin<Alternate<OTYPE, { idx_from_AFx!($af) }>> {}
                impl<OTYPE> $trait for $pin<Alternate<OTYPE, { idx_from_AFx!($af) }>> {}
            )*
        )*
    };
    // peripheral focused version
    ($peri:ty = [ $(($af:ident = [ $(($pin:ident, $trait:ident),)+]),)*]) => {
        $(
            $(
                impl<OTYPE> private::Sealed for $pin<Alternate<OTYPE, { idx_from_AFx!($af) }>> {}
                impl<OTYPE> $trait< $peri > for $pin<Alternate<OTYPE, { idx_from_AFx!($af) }>> {}
            )*
        )*
    };
}

// all ofAF0
map_af![
    (AF0 = [
        (PA8, McoPin),
        (PA13, SwdioPin),
        (PA14, SwclkPin),
        (PA15, JtdiPin),
        (PB3, JtdoPin),
        (PB4, NjrstPin),
        // ...
    ]),
];

// all of TIM1
map_af![
    TIM1 = [
        (AF1 = [
            (PA6, BreakIn),
            (PA7, PwmCh1N),
            (PA8, PwmCh1),
            (PA9, PwmCh2),
            (PA10, PwmCh3),
            (PA11, PwmCh4),
            (PA12, ExtTriggerIn),
            (PB0, PwmCh2N),
            // ...
        ]),
        (AF2 = [
            (PA11, BreakIn),
            // ...
        ]),
    ]
];
