#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use stmlib::{entry, iprint, iprintln};

use heapless::{consts, String, Vec};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = stmlib::init();
    // Send command to Wifi
    for esp in b"AT\r\n".iter() {
        while usart1.isr.read().txe().bit_is_clear() {}

        usart1.tdr.write(|w| w.tdr().bits(u16::from(*esp)));
    }
    loop {
        let mut buffer: Vec<u8, consts::U32> = Vec::new();
        let mut count: u8 = 0;
        buffer.clear();
        loop {
            while usart1.isr.read().rxne().bit_is_clear() {}
            let byte = usart1.rdr.read().rdr().bits() as u8;
            // count the carrige retrun and break the function
            if byte == 13 {
                count = count + 1;
                if count == 2 {
                    break;
                }
            } else if byte == 10 {
                //if the byte resive new line byte will do nothing
            } else {
                if buffer.push(byte).is_err() {
                    // buffer full
                    for byte in b"error: buffer full\n\r" {
                        while usart1.isr.read().txe().bit_is_clear() {}
                        usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
                    }

                    break;
                }
            }
            // iprintln!(&mut itm.stim[0], "DATA : {:?}", byte as char);
        }
        // This will show responce from ESP
        let mut string = String::from_utf8(buffer);
        iprintln!(&mut itm.stim[0], "DATA : {:?}", string);
        //
        match String {
            Ok(i) => break,       // if result is ok then function break
            Err(i) => somefunc(), // try is error then call function again
        }
    }

    //for Convert to Server mode
    loop {
        iprintln!(&mut itm.stim[0], "GOT IT!!!");
    }
}
