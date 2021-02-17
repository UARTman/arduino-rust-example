#![no_std]
#![no_main]
// Эти две директивы уберут стандартную библиотеку Rust

// Подключим библиотеку - обработчик паник
extern crate panic_halt;

// Подключим библиотеку для работы с Arduino Uno
use arduino_uno::prelude::*;

// Пометим функцию main
// Заметим, что она возвращает странный тип "!"
// Это гарантия компилятору, что при нормальной работе она никогда не завершится
#[arduino_uno::entry]
fn main() -> ! {
    // Подключим периферию
    let dp = arduino_uno::Peripherals::take().unwrap();

    // Объявим интерфейс для работы с пинами
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Светодиод на пине 13. Возьмем его в режиме вывода.
    let mut led = pins.d13.into_output(&mut pins.ddr);


    led.set_high().void_unwrap();

    loop {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(200);
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(200);
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(200);
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(200);
    }
}