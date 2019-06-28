# Rust I²C/SPI driver for AS3935 Franklin Lightning Sensor IC

[![Build Status](https://travis-ci.com/trashware/as3935-rs.svg?branch=master)](https://travis-ci.com/trashware/as3935-rs)
[![crates.io](https://meritbadge.herokuapp.com/as3935)](https://crates.io/crates/as3935)

This crate provides a Rust driver for the AS3935 Franklin Lightning Sensor IC.
It provides an easy to use high-level asynchronous API to interact with the sensor which provides
you with a non-blocking [channel](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html) of events.

It's currently built on top of [rppal library](https://crates.io/crates/rppal) and as such is limited to
Raspberry Pi.

--------------------------------------------------

The datasheet for AS3935 can be found [here](https://www.embeddedadventures.com/datasheets/AS3935_Datasheet_EN_v2.pdf)
or [here](https://cz.mouser.com/pdfdocs/AMS_AS3935_Datasheet_v4.pdf).
