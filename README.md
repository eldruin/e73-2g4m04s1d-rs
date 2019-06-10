# Rust example programs running on Ebyte's E73-2G4M04S1D BLE module board

[![Build Status](https://travis-ci.org/eldruin/e73-2g4m04s1d-rs.svg?branch=master)](https://travis-ci.org/eldruin/e73-2g4m04s1d-rs)

This repository contains example programs running on Ebyte's [E73-2G4M04S1D] BLE module board. This board is based on the nRF51822 chip and supports Bluetooth Low Energy (BLE) / Bluetooth Smart 4.2 and 5.0.

As it is based on the same chip as the [BBC micro:bit][] board, you can find many examples in the [microbit crate][] which you can adapt.

## Hardware

To run these programs you will need:

* An [E73-2G4M04S1D] board. Tipically you can buy one of these from [AliExpress][alie-e73] for very little money.
* An ST-Link V2 programmer. You can also buy one of these on [AliExpress][alie-stlink].
* A computer with Rust and OpenOCD (ideally Linux).

Look at the individual examples for the complete setup.

### Setup

Once you have the hardware you will need to solder some cables. You need to solder at least the pins: `GND`, `VCC`, `SWDCLK`, `SWDIO`, `P0.20` (Also some more I/O pins for other examples than blinky).

On the nRF51822 most I/O pins can be assigned to many different peripherals, which is very convenient.

**Beware** that this board only accepts power up to **3.6V**. Applying 5V will probably burn it. Remember to use a [logic level converter][level-conv-howto] if you connect 5V peripherals. Yes, you can also get one of those on [AliExpress][alie-level-conv].

## Support

For questions, issues, and other requests, please file an [issue](https://github.com/eldruin/e73-2g4m04s1d-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[AliExpress]: https://www.aliexpress.com
[alie-e73]: https://www.aliexpress.com/wholesale?SearchText=E73-2G4M04S1D
[alie-level-conv]: https://www.aliexpress.com/wholesale?SearchText=logic+level+converter
[alie-stlink]: https://www.aliexpress.com/wholesale?SearchText=st-link+v2
[BBC micro:bit]: https://microbit.org
[E73-2G4M04S1D]: http://www.cdebyte.net/e77%ef%bc%882g4m04s%ef%bc%89.html
[level-conv-howto]: https://learn.sparkfun.com/tutorials/bi-directional-logic-level-converter-hookup-guide/all
[microbit crate]: https://github.com/therealprof/microbit