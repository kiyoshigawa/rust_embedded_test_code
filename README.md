# NRF52840 Testing Library

This is a repo for testing basic embedded rust projects using the NRF52840-dk board from Nordik and to get the [nrf_hal](https://github.com/nrf-rs/nrf-hal)'s `nrf52840-hal` crate and [Knurling](https://knurling.ferrous-systems.com/)'s `probe-run` and `defmt` functionality working. The goal of this repo is to work as a good starting point for further projects which will use the Knurling toolchain and nrf-hal peripheral libraries.

Almost all of the setup for this code was taken directly from [this blog post](https://nitschinger.at/Getting-Started-with-the-nRF52840-in-Rust/) by Michael Nitschinger. It covers the steps required to get the code to build and run on the hardware.

Since I was doing this on Windows, I also had to use [Zadig](https://zadig.akeo.ie/) to replace the J-Link's `BULK Interface (Interface 2)` SEGGER driver with the WinUSB version in order to get it to work with Knurling's `probe-run` command. Once that was done, the code worked just as it was shown in the blog post.