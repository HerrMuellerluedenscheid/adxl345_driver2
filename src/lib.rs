// MIT License
//
// Copyright 2023-2024 Michael Büsch <m@bues.ch>
// Copyright © 2020-present, Michael Cummings <mgcummings@yahoo.com>.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Provides a complete ADXL345 compatible command set and supporting code.
//!
//! This is meant to be a hardware level driver interface for the device.

#![cfg_attr(no_std, no_std)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate c2rust_bitfields;

mod cmd;
mod error;
pub mod i2c;
pub mod spi;

pub(crate) use crate::cmd::Adxl345AccExtract;
pub(crate) use crate::cmd::Adxl345Init;
pub use crate::{
    cmd::{
        ATStatus, ActivityMode, Adxl345, Adxl345Reader, Adxl345Writer, BandwidthRateControl,
        DataFormat, FifoControl, FifoStatus, IntControlMode, IntMapMode, IntSource, PowerControl,
        Tap, TapMode,
    },
    error::{AdxlError, AdxlResult, Result},
};
