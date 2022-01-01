use stm32l4::stm32l4x6::TIM16;

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

af_table_af0af7! {
//      AF0      AF1            AF2             AF3          AF4           AF5        AF6          AF7
[ PA0  | _ |PwmCh1<TIM2>  |PwmCh1<TIM5>   |_/*TIM8_ETR*/|     _      |      _      |   _    |  CtsPin<USART2>]
[ PA1  | _ |PwmCh2<TIM2>  |PwmCh2<TIM5>   |      _      |     _      |      _      |   _    |RtsDePin<USART2>]
[ PA2  | _ |PwmCh3<TIM2>  |PwmCh3<TIM5>   |      _      |     _      |      _      |   _    |  TxPin<USART2> ]
[ PA3  | _ |PwmCh4<TIM2>  |PwmCh4<TIM5>   |      _      |     _      |      _      |   _    |  RxPin<USART2> ]
[ PA4  | _ |     _        |      _        |      _      |     _      |_/*SPI1_NSS*/|_/*SPI3_NSS*/|_/*USART2_CK*/]
[ PA5  | _ |PwmCh1<TIM2>  |_/*TIM2_ETR*/  |_/*TIM8_CH1N*/|     _      |SckPin<SPI1> |   _    |       _        ]
[ PA6  | _ |_/*TIM1_BKIN*/|PwmCh1<TIM3>   |_/*TIM8_BKIN*/|     _      |MisoPin<SPI1>|   _    |  CtsPin<USART3>]
[ PA7  | _ |_/*TIM1_CH1N*/|PwmCh2<TIM3>   |_/*TIM8_CH1N*/|     _      |MosiPin<SPI1>|   _    |       _        ]
[ PA8  |_/*MCO*/ |PwmCh1<TIM1>  |      _        |      _      |     _      |      _      |   _    |_/*USART2_CK*/]
[ PA9  | _ |PwmCh2<TIM1>  |      _        |      _      |     _      |      _      |   _    |  TxPin<USART1> ]
[ PA10 | _ |PwmCh3<TIM1>  |      _        |      _      |     _      |      _      |   _    |  RxPin<USART1> ]
[ PA11 | _ |PwmCh4<TIM1>  |_/*TIM1_BKIN2*/|      _      |     _      |_|   _    |CtsPin<USART1>]
[ PA12 | _ |_/*TIM1_ETR*/ |      _        |      _      |     _      |_|   _    |RtsDePin<USART1>]
[ PA13 |_/*JTMS-SWDIO*/| _ /*IR_OUT*/ |      _        |      _      |     _      |      _      |   _    |       _        ]
[ PA14 |_/*JTMS-SWCLK*/|     _        |      _        |      _      |     _      |      _      |   _    |       _        ]
[ PA15 |_/*JTDI*/|PwmCh1<TIM2>  |_/*TIM2_ETR*/  |RxPin<USART2>|     _      |_/*SPI1_NSS*/|_/*SPI3_NSS*/|RtsDePin<USART3>]
}

af_table_af8af15! {
//      AF8             AF9             AF10            AF11              AF12                   AF13                   AF14
[ PA0  |TxPin<UART4>   |       _       |       _       | _               | _                    | _/*SAI1_EXTCLK*/      | _/*TIM2_ETR */ | _ ]
[ PA1  |RxPin<UART4>   |       _       |       _       | _/* LCD_SEG0 */ | _                    | _                     | _/*TIM15_CH1N */ | _ ]
[ PA2  | _             |       _       |_              | _/* LCD_SEG1 */ | _                    | _/*SAI2_EXTCLK */     | PwmCh1<TIM15> | _ ]
[ PA3  | _             |       _       |_              | _/* LCD_SEG2 */ | _                    | _                     | PwmCh2<TIM15> | _ ]
[ PA4  | _             |       _       |       _       | _               | _                    | _/*SAI1_FS_B */       | _/*LPTIM2_OUT */ | _ ]
[ PA5  | _             |       _       |       _       | _               | _                    | _                     | _/*LPTIM2_ETR */ | _ ]
[ PA6  | _             |       _       |IO3Pin<QUADSPI>| _/* LCD_SEG3 */ | _/*TIM1_BKIN_COMP2 */| _/*TIM8_BKIN_COMP2 */ | _/*TIM16_ETR */ | _ ]
[ PA7  | _             |       _       |IO2Pin<QUADSPI>| _/* LCD_SEG4 */ | _                    | _                     | PwmCh1<TIM16>/*TIM16_CH1 */ | _ ]
[ PA8  | _             |       _       |_/*OTG_FS_SOF*/| _/* LCD_COM0 */ | _                    | _                     | PwmCh1<TIM17>/*TIM17_CH1 */ | _ ]
[ PA9  | _             |       _       |       _       | _/* LCD_COM1 */ | _                    | _                     | _/*TIM15_BKIN */ | _ ]
[ PA10 | _             |       _       |_/*OTG_FS_ID*/ | _/* LCD_COM2 */ | _                    | _                     | _/*TIM17_BKIN */ | _ ]
[ PA11 | _             |CanRxPin<CAN1> |_/*OTG_FS_DM*/ | _               | _/*TIM1_BKIN2_COMP1*/| _                     | _ | _ ]
[ PA12 | _             |CanTxPin<CAN1> |_/*OTG_FS_DP*/ | _               | _                    | _                     | _ | _ ]
[ PA13 | _             |       _       |_/*OTG_FS_NOE*/| _               | _                    | _                     | _ | _ ]
[ PA14 | _             |       _       |       _       | _               | _                    | _                     | _ | _ ]
[ PA15 |RtsDePin<UART4>|_/*TSC_G3_IO1*/|       _       | _/* LCD_SEG17*/ | _                    | _/*SAI2_FS_B */       | _ | _ ]
}
