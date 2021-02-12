#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use stm32f1xx_hal::{
    pac, prelude::*, timer::Timer,
    serial::{Config, Serial},
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    //let mut gpiod = dp.GPIOD.split(&mut rcc.apb2);
    //let mut gpioe = dp.GPIOE.split(&mut rcc.apb2);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(0_2.hz());

    // USART communication init
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let channels = dp.DMA1.split(&mut rcc.ahb);

    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    let serial = Serial::usart1(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()),
        clocks,
        &mut rcc.apb2,
    );

    let mut tx = serial.split().0.with_dma(channels.4);

    macro_rules! run {
        ([
            $(($gpiox:ident, $pxi:ident, $crx:ident, $label:expr),)+
        ]) => {
            $(
                let mut $pxi = ($gpiox.$pxi.into_pull_up_input(&mut $gpiox.$crx), false);
            )+

            loop {
                block!(timer.wait()).unwrap();
                led.set_high().unwrap();

                $(
                    let is_high = $pxi.0.is_high().unwrap();
                    if $pxi.1 != is_high {  // has is_high state changed?
                        $pxi.1 = is_high;

                        tx = tx.write($label).wait().1;
                        tx = tx.write(b":").wait().1;
                        tx = tx.write(if is_high {b"1"} else {b"0"}).wait().1;
                        tx = tx.write(b";").wait().1;

                        led.set_low().unwrap();
                    }
                )+
            }
        }
    }

    run!([
        (gpioa, pa0, crl, b"PA0"),
        (gpioa, pa1, crl, b"PA1"),
        (gpioa, pa2, crl, b"PA2"),
        (gpioa, pa3, crl, b"PA3"),
        (gpioa, pa4, crl, b"PA4"),
        (gpioa, pa5, crl, b"PA5"),
        (gpioa, pa6, crl, b"PA6"),
        (gpioa, pa7, crl, b"PA7"),
        (gpioa, pa8, crh, b"PA8"),
        (gpioa, pa11, crh, b"PA11"),
        (gpioa, pa12, crh, b"PA12"),
        (gpiob, pb0, crl, b"PB0"),
        (gpiob, pb1, crl, b"PB1"),
        (gpiob, pb5, crl, b"PB5"),
        (gpiob, pb6, crl, b"PB6"),
        (gpiob, pb7, crl, b"PB7"),
        (gpiob, pb8, crh, b"PB8"),
        (gpiob, pb9, crh, b"PB9"),
        (gpiob, pb10, crh, b"PB10"),
        (gpiob, pb11, crh, b"PB11"),
        (gpiob, pb12, crh, b"PB12"),
        (gpiob, pb13, crh, b"PB13"),
        (gpiob, pb14, crh, b"PB14"),
        (gpiob, pb15, crh, b"PB15"),
    ]);
}
