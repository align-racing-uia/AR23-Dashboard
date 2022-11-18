// Generated code!
#![allow(unused_comparisons, unreachable_patterns)]
#![allow(clippy::let_and_return, clippy::eq_op)]
#![allow(clippy::excessive_precision, clippy::manual_range_contains, clippy::absurd_extreme_comparisons)]
#![deny(clippy::integer_arithmetic)]

//! Message definitions from file `"AlignDBC.dbc"`
//!
//! - Version: `Version("1")`

use core::ops::BitOr;
use bitvec::prelude::*;
#[cfg(feature = "arb")]
use arbitrary::{Arbitrary, Unstructured};

/// All messages
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum MessagesAlign {
    /// APPS
    Apps(Apps),
}

impl MessagesAlign {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: u32, payload: &[u8]) -> Result<Self, CanError> {
        use core::convert::TryFrom;
        
        let res = match id {
            377 => MessagesAlign::Apps(Apps::try_from(payload)?),
            n => return Err(CanError::UnknownMessageId(n)),
        };
        Ok(res)
    }
}

/// APPS
///
/// - ID: 377 (0x179)
/// - Size: 8 bytes
/// - Transmitter: AlignBus
///
/// APPS message
#[derive(Clone, Copy)]
pub struct Apps {
    raw: [u8; 8],
}

impl Apps {
    pub const MESSAGE_ID: u32 = 377;
    
    pub const BRAKE_PRESSURE_MIN: f32 = 0_f32;
    pub const BRAKE_PRESSURE_MAX: f32 = 2500_f32;
    pub const SENSOR1_MIN: u8 = 0_u8;
    pub const SENSOR1_MAX: u16 = 2500_u16;
    pub const SENSOR2_MIN: u8 = 0_u8;
    pub const SENSOR2_MAX: u16 = 2500_u16;
    
    /// Construct new APPS from values
    pub fn new(ready_to_drive: bool, shutdown_circuit: bool, brake_implosibility: bool, deviation: bool, brake_pressure: f32, sensor1: u8, sensor2: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_ready_to_drive(ready_to_drive)?;
        res.set_shutdown_circuit(shutdown_circuit)?;
        res.set_brake_implosibility(brake_implosibility)?;
        res.set_deviation(deviation)?;
        res.set_brake_pressure(brake_pressure)?;
        res.set_sensor1(sensor1)?;
        res.set_sensor2(sensor2)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// ReadyToDrive
    ///
    /// Ready_to_drive_state
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn ready_to_drive(&self) -> bool {
        self.ready_to_drive_raw()
    }
    
    /// Get raw value of ReadyToDrive
    ///
    /// - Start bit: 7
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn ready_to_drive_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[7..8].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of ReadyToDrive
    #[inline(always)]
    pub fn set_ready_to_drive(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[7..8].store_le(value);
        Ok(())
    }
    
    /// ShutdownCircuit
    ///
    /// Shutdown_circuit_state
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn shutdown_circuit(&self) -> bool {
        self.shutdown_circuit_raw()
    }
    
    /// Get raw value of ShutdownCircuit
    ///
    /// - Start bit: 6
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn shutdown_circuit_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[6..7].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of ShutdownCircuit
    #[inline(always)]
    pub fn set_shutdown_circuit(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[6..7].store_le(value);
        Ok(())
    }
    
    /// BrakeImplosibility
    ///
    /// BSPD_function_triggered,_let_off_the_throtle_to_restart
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_implosibility(&self) -> bool {
        self.brake_implosibility_raw()
    }
    
    /// Get raw value of BrakeImplosibility
    ///
    /// - Start bit: 5
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_implosibility_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[5..6].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of BrakeImplosibility
    #[inline(always)]
    pub fn set_brake_implosibility(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[5..6].store_le(value);
        Ok(())
    }
    
    /// Deviation
    ///
    /// Deviation_error,_cycle_R2D_to_reset
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn deviation(&self) -> bool {
        self.deviation_raw()
    }
    
    /// Get raw value of Deviation
    ///
    /// - Start bit: 4
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn deviation_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[4..5].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of Deviation
    #[inline(always)]
    pub fn set_deviation(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[4..5].store_le(value);
        Ok(())
    }
    
    /// BrakePressure
    ///
    /// - Min: 0
    /// - Max: 2500
    /// - Unit: "bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_pressure(&self) -> f32 {
        self.brake_pressure_raw()
    }
    
    /// Get raw value of BrakePressure
    ///
    /// - Start bit: 8
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_pressure_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[8..24].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of BrakePressure
    #[inline(always)]
    pub fn set_brake_pressure(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 2500_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 377 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[8..24].store_le(value);
        Ok(())
    }
    
    /// Sensor1
    ///
    /// - Min: 0
    /// - Max: 2500
    /// - Unit: "%"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn sensor1(&self) -> u8 {
        self.sensor1_raw()
    }
    
    /// Get raw value of Sensor1
    ///
    /// - Start bit: 24
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn sensor1_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
        
        signal
    }
    
    /// Set value of Sensor1
    #[inline(always)]
    pub fn set_sensor1(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 2500_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 377 });
        }
        self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
        Ok(())
    }
    
    /// Sensor2
    ///
    /// - Min: 0
    /// - Max: 2500
    /// - Unit: "%"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn sensor2(&self) -> u8 {
        self.sensor2_raw()
    }
    
    /// Get raw value of Sensor2
    ///
    /// - Start bit: 32
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn sensor2_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[32..40].load_le::<u8>();
        
        signal
    }
    
    /// Set value of Sensor2
    #[inline(always)]
    pub fn set_sensor2(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 2500_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 377 });
        }
        self.raw.view_bits_mut::<Lsb0>()[32..40].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Apps {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Apps {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Apps")
                .field("ready_to_drive", &self.ready_to_drive())
                .field("shutdown_circuit", &self.shutdown_circuit())
                .field("brake_implosibility", &self.brake_implosibility())
                .field("deviation", &self.deviation())
                .field("brake_pressure", &self.brake_pressure())
                .field("sensor1", &self.sensor1())
                .field("sensor2", &self.sensor2())
            .finish()
        } else {
            f.debug_tuple("Apps").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Apps {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let ready_to_drive = u.int_in_range(0..=1)? == 1;
        let shutdown_circuit = u.int_in_range(0..=1)? == 1;
        let brake_implosibility = u.int_in_range(0..=1)? == 1;
        let deviation = u.int_in_range(0..=1)? == 1;
        let brake_pressure = u.float_in_range(0_f32..=2500_f32)?;
        let sensor1 = u.int_in_range(0..=2500)?;
        let sensor2 = u.int_in_range(0..=2500)?;
        Apps::new(ready_to_drive,shutdown_circuit,brake_implosibility,deviation,brake_pressure,sensor1,sensor2).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}


/// This is just to make testing easier
#[allow(dead_code)]
fn main() {}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(any(feature = "debug", feature = "std"), derive(Debug))]
pub enum CanError {
    UnknownMessageId(u32),
    /// Signal parameter is not within the range
    /// defined in the dbc
    ParameterOutOfRange {
        /// dbc message id
        message_id: u32,
    },
    InvalidPayloadSize,
    /// Multiplexor value not defined in the dbc
    InvalidMultiplexor {
        /// dbc message id
        message_id: u32,
        /// Multiplexor value not defined in the dbc
        multiplexor: u16,
    },
}

#[cfg(feature = "std")]
use std::error::Error;
#[cfg(feature = "std")]
use std::fmt;

#[cfg(feature = "std")]
impl fmt::Display for CanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl Error for CanError {}
#[cfg(feature = "arb")]
trait UnstructuredFloatExt {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32>;
}

#[cfg(feature = "arb")]
impl UnstructuredFloatExt for arbitrary::Unstructured<'_> {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32> {
        let min = range.start();
        let max = range.end();
        let steps = u32::MAX;
        let factor = (max - min) / (steps as f32);
        let random_int: u32 = self.int_in_range(0..=steps)?;
        let random = min + factor * (random_int as f32);
        Ok(random)
    }
}

