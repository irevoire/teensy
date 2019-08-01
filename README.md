# Teensy

Currently this crate should **NOT** be used in anything serious.

You can find the documentation here until we publish the crate:
http://irevoire.irevoire.ovh/teensy/crate_doc/teensy/index.html

You can find some **examples** in the [examples](examples) directory.
To test the example you first need to follow the quickstart guide from the documentation.
Then you can choose an example and flash it in the teensy using the `make flash_%` rule.
For example if you want to test the blink example you can run `make flash_blink`.

## Documentation on the teensy
[Official documentation from pjrc](teensy_3.2.pdf)

Library implementations:
https://github.com/mensi/teensy_bare_metal

## USB
We should implement this:
https://docs.rs/usb-device/0.2.0/src/usb_device/bus.rs.html#17-134

Here is a great ressource on usb:
https://wiki.osdev.org/Universal_Serial_Bus
