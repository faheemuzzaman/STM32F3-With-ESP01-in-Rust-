#![deny(unsafe_code)]
#![no_main]
#![no_std]

use heapless::{consts, Vec};
#[allow(unused_imports)]
use stmlib::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = stmlib::init();

    for byte in b"AT+CWJAP=\"TP-LINK_F290\",\"36401802\"\r\n".iter() {
        // wait until it's safe to write to TDR
        while usart1.isr.read().txe().bit_is_clear() {} // <- NEW!

        usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
    }
    let mut buffer: Vec<u8, consts::U32> = Vec::new();
    for i in 0..99999 {}
    loop {
        let mut count: u8 = 0;
        for byte in b"AT+CIPSTART=\"TCP\",\"192.168.1.107\",5500\r\n".iter() {
            // wait until it's safe to write to TDR
            while usart1.isr.read().txe().bit_is_clear() {} // <- NEW!

            usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
        }
        for i in 0..999 {}
        for byte in b"AT+CIPSEND=40\r\n".iter() {
            // wait until it's safe to write to TDR
            while usart1.isr.read().txe().bit_is_clear() {} // <- NEW!

            usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
        }
        for i in 0..999 {}
        for byte in b"GET / HTTP/1.1\r\nHost: 192.168.1.107\r\n\r\n\r\n".iter() {
            // wait until it's safe to write to TDR
            while usart1.isr.read().txe().bit_is_clear() {} // <- NEW!

            usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
        }

        buffer.clear();

        loop {
            while usart1.isr.read().rxne().bit_is_clear() {}
            let byte = usart1.rdr.read().rdr().bits() as u8;
            if byte == 13 {
                count = count + 1;
                if count == 13 {
                    if buffer.push(byte).is_err() {
                        // buffer full
                        for byte in b"error: buffer full\n\r" {
                            while usart1.isr.read().txe().bit_is_clear() {}
                            usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
                        }
                        iprintln!(&mut itm.stim[0], "DATA : {:#?}", byte as char);
                    }
                } else if count == 14 {
                    break;
                }
            }
        }
        // for ascii in &buffer {
        //     iprintln!(&mut itm.stim[0], "DATA : {:#?}", *ascii as char);
        // }
    }
}
