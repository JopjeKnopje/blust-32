# Rust embedded experimentation

## probe-rs

Instead of using `cargo embed` (which is being phased out) we use [probe-rs](https://probe.rs/docs/tools/which-tool/). Which is a really faken cool tool.
You can write tiny rust [scripts](https://probe.rs/docs/library/quickstart/) for it to automate debugging. Its also used for [RTT](https://kb.segger.com/RTT) - which appeartanly is what rust people use for debugging instead of serial prints.

Probe-rs supports the jlinkv2 and a bunch of other hardware programmers / debuggers.

## Which libary or HAL to use?

After some searching I've come across `Embassy` and the official `embedded-hal`.
Embassy looks very much like a high level (RTOS?) supporting async and other wild stuff I won't need.
The embedded-hal on the other hand just implements traits making it system agnostic, if you want to run it on and STM32 you'd have to install a package (in my case `cortex-m`) which implements the traits for that specific platform. This sounds pretty cool and handy. It also looks like the embedded-hal is a bit more low level than Embassy.

## Hardware

STM32 Bluepill
![bluepill-pinout](https://github.com/stm32-rs/stm32f1xx-hal/blob/master/BluePillPinout.jpg?raw=true)

### Boot headers

The 2 boot headers on the stm32 can be left at their `0` position.

> [!NOTE]
> I haven't tested putting them in a different position because flashing with the ST-LINK V2 just worked.

Read more about the boot process here.

- [STM32 - boot process](https://community.st.com/stm32-mcus-60/faq-stm32-boot-process-37)

https://github.com/stm32-rs/stm32-rs

## Todo

- [ ] Read about different kinds of `delay` function, I'm pretty sure they're all using hardware timers. But I'm curious to see how they're abstracted away. Also have a look at how arduino implements them (I think its just a bunch of `NOOP`s scaled to the core's clock)

## Resources

- [i2c-bme280 example](https://github.com/stm32-rs/stm32f1xx-hal/blob/master/examples/i2c-bme280/src/main.rs#L33-L33)
- [Demystifying Rust Embedded HAL | split() constrain()](https://dev.to/theembeddedrustacean/demystifying-rust-embedded-hal-split-and-constrain-methods-591e)
- [Embedded rust basics](https://blog.implrust.com/posts/2026/08/blinky-with-stm32f103c8t6-embedded-rust/)
