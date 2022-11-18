// Generated code!
#![allow(unused_comparisons, unreachable_patterns)]
#![allow(clippy::let_and_return, clippy::eq_op)]
#![allow(clippy::excessive_precision, clippy::manual_range_contains, clippy::absurd_extreme_comparisons)]
#![deny(clippy::integer_arithmetic)]

//! Message definitions from file `"cascadia.dbc"`
//!
//! - Version: `Version("")`

use core::ops::BitOr;
use bitvec::prelude::*;
#[cfg(feature = "arb")]
use arbitrary::{Arbitrary, Unstructured};

/// All MessagesCascadia
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum MessagesCascadia {
    /// M173_Modulation_And_Flux_Info
    M173ModulationAndFluxInfo(M173ModulationAndFluxInfo),
    /// M172_Torque_And_Timer_Info
    M172TorqueAndTimerInfo(M172TorqueAndTimerInfo),
    /// M194_Read_Write_Param_Response
    M194ReadWriteParamResponse(M194ReadWriteParamResponse),
    /// M193_Read_Write_Param_Command
    M193ReadWriteParamCommand(M193ReadWriteParamCommand),
    /// M192_Command_Message
    M192CommandMessage(M192CommandMessage),
    /// M171_Fault_Codes
    M171FaultCodes(M171FaultCodes),
    /// M170_Internal_States
    M170InternalStates(M170InternalStates),
    /// M169_Internal_Voltages
    M169InternalVoltages(M169InternalVoltages),
    /// M168_Flux_ID_IQ_Info
    M168FluxIdIqInfo(M168FluxIdIqInfo),
    /// M167_Voltage_Info
    M167VoltageInfo(M167VoltageInfo),
    /// M166_Current_Info
    M166CurrentInfo(M166CurrentInfo),
    /// M165_Motor_Position_Info
    M165MotorPositionInfo(M165MotorPositionInfo),
    /// M164_Digital_Input_Status
    M164DigitalInputStatus(M164DigitalInputStatus),
    /// M163_Analog_Input_Voltages
    M163AnalogInputVoltages(M163AnalogInputVoltages),
    /// M162_Temperature_Set_3
    M162TemperatureSet3(M162TemperatureSet3),
    /// M161_Temperature_Set_2
    M161TemperatureSet2(M161TemperatureSet2),
    /// M160_Temperature_Set_1
    M160TemperatureSet1(M160TemperatureSet1),
    /// M174_Firmware_Info
    M174FirmwareInfo(M174FirmwareInfo),
    /// M175_Diag_Data
    M175DiagData(M175DiagData),
    /// M187_U2C_Command_Txd
    M187U2cCommandTxd(M187U2cCommandTxd),
    /// M188_U2C_Message_Rxd
    M188U2cMessageRxd(M188U2cMessageRxd),
    /// BMS_Current_Limit
    BmsCurrentLimit(BmsCurrentLimit),
    /// M176_Fast_Info
    M176FastInfo(M176FastInfo),
}

impl MessagesCascadia {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: u32, payload: &[u8]) -> Result<Self, CanError> {
        use core::convert::TryFrom;
        
        let res = match id {
            173 => MessagesCascadia::M173ModulationAndFluxInfo(M173ModulationAndFluxInfo::try_from(payload)?),
            172 => MessagesCascadia::M172TorqueAndTimerInfo(M172TorqueAndTimerInfo::try_from(payload)?),
            194 => MessagesCascadia::M194ReadWriteParamResponse(M194ReadWriteParamResponse::try_from(payload)?),
            193 => MessagesCascadia::M193ReadWriteParamCommand(M193ReadWriteParamCommand::try_from(payload)?),
            192 => MessagesCascadia::M192CommandMessage(M192CommandMessage::try_from(payload)?),
            171 => MessagesCascadia::M171FaultCodes(M171FaultCodes::try_from(payload)?),
            170 => MessagesCascadia::M170InternalStates(M170InternalStates::try_from(payload)?),
            169 => MessagesCascadia::M169InternalVoltages(M169InternalVoltages::try_from(payload)?),
            168 => MessagesCascadia::M168FluxIdIqInfo(M168FluxIdIqInfo::try_from(payload)?),
            167 => MessagesCascadia::M167VoltageInfo(M167VoltageInfo::try_from(payload)?),
            166 => MessagesCascadia::M166CurrentInfo(M166CurrentInfo::try_from(payload)?),
            165 => MessagesCascadia::M165MotorPositionInfo(M165MotorPositionInfo::try_from(payload)?),
            164 => MessagesCascadia::M164DigitalInputStatus(M164DigitalInputStatus::try_from(payload)?),
            163 => MessagesCascadia::M163AnalogInputVoltages(M163AnalogInputVoltages::try_from(payload)?),
            162 => MessagesCascadia::M162TemperatureSet3(M162TemperatureSet3::try_from(payload)?),
            161 => MessagesCascadia::M161TemperatureSet2(M161TemperatureSet2::try_from(payload)?),
            160 => MessagesCascadia::M160TemperatureSet1(M160TemperatureSet1::try_from(payload)?),
            174 => MessagesCascadia::M174FirmwareInfo(M174FirmwareInfo::try_from(payload)?),
            175 => MessagesCascadia::M175DiagData(M175DiagData::try_from(payload)?),
            471 => MessagesCascadia::M187U2cCommandTxd(M187U2cCommandTxd::try_from(payload)?),
            469 => MessagesCascadia::M188U2cMessageRxd(M188U2cMessageRxd::try_from(payload)?),
            514 => MessagesCascadia::BmsCurrentLimit(BmsCurrentLimit::try_from(payload)?),
            176 => MessagesCascadia::M176FastInfo(M176FastInfo::try_from(payload)?),
            n => return Err(CanError::UnknownMessageId(n)),
        };
        Ok(res)
    }
}

/// M173_Modulation_And_Flux_Info
///
/// - ID: 173 (0xad)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M173ModulationAndFluxInfo {
    raw: [u8; 8],
}

impl M173ModulationAndFluxInfo {
    pub const MESSAGE_ID: u32 = 173;
    
    pub const D4_IQ_COMMAND_MIN: f32 = -3276.8_f32;
    pub const D4_IQ_COMMAND_MAX: f32 = 3276.7_f32;
    pub const D3_ID_COMMAND_MIN: f32 = -3276.8_f32;
    pub const D3_ID_COMMAND_MAX: f32 = 3276.7_f32;
    pub const D2_FLUX_WEAKENING_OUTPUT_MIN: f32 = -3276.8_f32;
    pub const D2_FLUX_WEAKENING_OUTPUT_MAX: f32 = 3276.7_f32;
    pub const D1_MODULATION_INDEX_MIN: f32 = -3.2768_f32;
    pub const D1_MODULATION_INDEX_MAX: f32 = 3.2767_f32;
    
    /// Construct new M173_Modulation_And_Flux_Info from values
    pub fn new(d4_iq_command: f32, d3_id_command: f32, d2_flux_weakening_output: f32, d1_modulation_index: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d4_iq_command(d4_iq_command)?;
        res.set_d3_id_command(d3_id_command)?;
        res.set_d2_flux_weakening_output(d2_flux_weakening_output)?;
        res.set_d1_modulation_index(d1_modulation_index)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D4_Iq_Command
    ///
    /// The commanded Q-axis current
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_iq_command(&self) -> f32 {
        self.d4_iq_command_raw()
    }
    
    /// Get raw value of D4_Iq_Command
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d4_iq_command_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D4_Iq_Command
    #[inline(always)]
    pub fn set_d4_iq_command(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 173 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
    /// D3_Id_Command
    ///
    /// The commanded D-axis current
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_id_command(&self) -> f32 {
        self.d3_id_command_raw()
    }
    
    /// Get raw value of D3_Id_Command
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d3_id_command_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D3_Id_Command
    #[inline(always)]
    pub fn set_d3_id_command(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 173 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// D2_Flux_Weakening_Output
    ///
    /// This is the current output of the flux regulator.
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_flux_weakening_output(&self) -> f32 {
        self.d2_flux_weakening_output_raw()
    }
    
    /// Get raw value of D2_Flux_Weakening_Output
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d2_flux_weakening_output_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D2_Flux_Weakening_Output
    #[inline(always)]
    pub fn set_d2_flux_weakening_output(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 173 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// D1_Modulation_Index
    ///
    /// This is the modulation index. The scale factor is x100. To get the actual modulation index divide the value by 100.
    ///
    /// - Min: -3.2768
    /// - Max: 3.2767
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_modulation_index(&self) -> f32 {
        self.d1_modulation_index_raw()
    }
    
    /// Get raw value of D1_Modulation_Index
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.0001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d1_modulation_index_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.0001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D1_Modulation_Index
    #[inline(always)]
    pub fn set_d1_modulation_index(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3.2768_f32 || 3.2767_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 173 });
        }
        let factor = 0.0001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M173ModulationAndFluxInfo {
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
impl core::fmt::Debug for M173ModulationAndFluxInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M173ModulationAndFluxInfo")
                .field("d4_iq_command", &self.d4_iq_command())
                .field("d3_id_command", &self.d3_id_command())
                .field("d2_flux_weakening_output", &self.d2_flux_weakening_output())
                .field("d1_modulation_index", &self.d1_modulation_index())
            .finish()
        } else {
            f.debug_tuple("M173ModulationAndFluxInfo").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M173ModulationAndFluxInfo {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d4_iq_command = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d3_id_command = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d2_flux_weakening_output = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d1_modulation_index = u.float_in_range(-3.2768_f32..=3.2767_f32)?;
        M173ModulationAndFluxInfo::new(d4_iq_command,d3_id_command,d2_flux_weakening_output,d1_modulation_index).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M172_Torque_And_Timer_Info
///
/// - ID: 172 (0xac)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M172TorqueAndTimerInfo {
    raw: [u8; 8],
}

impl M172TorqueAndTimerInfo {
    pub const MESSAGE_ID: u32 = 172;
    
    pub const D3_POWER_ON_TIMER_MIN: f32 = 0_f32;
    pub const D3_POWER_ON_TIMER_MAX: f32 = 12884800_f32;
    pub const D2_TORQUE_FEEDBACK_MIN: f32 = -3276.8_f32;
    pub const D2_TORQUE_FEEDBACK_MAX: f32 = 3276.7_f32;
    pub const D1_COMMANDED_TORQUE_MIN: f32 = -3276.8_f32;
    pub const D1_COMMANDED_TORQUE_MAX: f32 = 3276.7_f32;
    
    /// Construct new M172_Torque_And_Timer_Info from values
    pub fn new(d3_power_on_timer: f32, d2_torque_feedback: f32, d1_commanded_torque: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d3_power_on_timer(d3_power_on_timer)?;
        res.set_d2_torque_feedback(d2_torque_feedback)?;
        res.set_d1_commanded_torque(d1_commanded_torque)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D3_Power_On_Timer
    ///
    /// Updated every 3 msec. This will roll over in approximately 150 days!
    ///
    /// - Min: 0
    /// - Max: 12884800
    /// - Unit: "Sec"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_power_on_timer(&self) -> f32 {
        self.d3_power_on_timer_raw()
    }
    
    /// Get raw value of D3_Power_On_Timer
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 0.003
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d3_power_on_timer_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
        
        let factor = 0.003_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D3_Power_On_Timer
    #[inline(always)]
    pub fn set_d3_power_on_timer(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 12884800_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 172 });
        }
        let factor = 0.003_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u32;
        
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
    /// D2_Torque_Feedback
    ///
    /// Estimated motor torque feedback
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "Nm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_torque_feedback(&self) -> f32 {
        self.d2_torque_feedback_raw()
    }
    
    /// Get raw value of D2_Torque_Feedback
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d2_torque_feedback_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D2_Torque_Feedback
    #[inline(always)]
    pub fn set_d2_torque_feedback(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 172 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// D1_Commanded_Torque
    ///
    /// The commanded Torque
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "Nm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_commanded_torque(&self) -> f32 {
        self.d1_commanded_torque_raw()
    }
    
    /// Get raw value of D1_Commanded_Torque
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d1_commanded_torque_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D1_Commanded_Torque
    #[inline(always)]
    pub fn set_d1_commanded_torque(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 172 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M172TorqueAndTimerInfo {
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
impl core::fmt::Debug for M172TorqueAndTimerInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M172TorqueAndTimerInfo")
                .field("d3_power_on_timer", &self.d3_power_on_timer())
                .field("d2_torque_feedback", &self.d2_torque_feedback())
                .field("d1_commanded_torque", &self.d1_commanded_torque())
            .finish()
        } else {
            f.debug_tuple("M172TorqueAndTimerInfo").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M172TorqueAndTimerInfo {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d3_power_on_timer = u.float_in_range(0_f32..=12884800_f32)?;
        let d2_torque_feedback = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d1_commanded_torque = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        M172TorqueAndTimerInfo::new(d3_power_on_timer,d2_torque_feedback,d1_commanded_torque).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M194_Read_Write_Param_Response
///
/// - ID: 194 (0xc2)
/// - Size: 8 bytes
/// - Transmitter: INV
///
/// The inverter response to each Parameter message that is received.
#[derive(Clone, Copy)]
pub struct M194ReadWriteParamResponse {
    raw: [u8; 8],
}

impl M194ReadWriteParamResponse {
    pub const MESSAGE_ID: u32 = 194;
    
    pub const D3_DATA_RESPONSE_MIN: i16 = -32768_i16;
    pub const D3_DATA_RESPONSE_MAX: i16 = 32767_i16;
    pub const D1_PARAMETER_ADDRESS_RESPONSE_MIN: u16 = 0_u16;
    pub const D1_PARAMETER_ADDRESS_RESPONSE_MAX: u16 = 65535_u16;
    
    /// Construct new M194_Read_Write_Param_Response from values
    pub fn new(d2_write_success: bool, d3_data_response: i16, d1_parameter_address_response: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d2_write_success(d2_write_success)?;
        res.set_d3_data_response(d3_data_response)?;
        res.set_d1_parameter_address_response(d1_parameter_address_response)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D2_Write_Success
    ///
    /// 0=Write failure, 1=Success
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_write_success(&self) -> bool {
        self.d2_write_success_raw()
    }
    
    /// Get raw value of D2_Write_Success
    ///
    /// - Start bit: 16
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d2_write_success_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[16..17].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D2_Write_Success
    #[inline(always)]
    pub fn set_d2_write_success(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[16..17].store_le(value);
        Ok(())
    }
    
    /// D3_Data_Response
    ///
    /// All EEPROM data is 16 bits and is contained in bytes 4 and 5. Bytes 6 and 7 should be ignored.
    ///
    /// - Min: -32768
    /// - Max: 32767
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_data_response(&self) -> i16 {
        self.d3_data_response_raw()
    }
    
    /// Get raw value of D3_Data_Response
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d3_data_response_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of D3_Data_Response
    #[inline(always)]
    pub fn set_d3_data_response(&mut self, value: i16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -32768_i16 || 32767_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 194 });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// D1_Parameter_Address_Response
    ///
    /// Valid EEPROM Parameter CAN addresses are between 100 and 499.
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_parameter_address_response(&self) -> u16 {
        self.d1_parameter_address_response_raw()
    }
    
    /// Get raw value of D1_Parameter_Address_Response
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d1_parameter_address_response_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        signal
    }
    
    /// Set value of D1_Parameter_Address_Response
    #[inline(always)]
    pub fn set_d1_parameter_address_response(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 194 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M194ReadWriteParamResponse {
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
impl core::fmt::Debug for M194ReadWriteParamResponse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M194ReadWriteParamResponse")
                .field("d2_write_success", &self.d2_write_success())
                .field("d3_data_response", &self.d3_data_response())
                .field("d1_parameter_address_response", &self.d1_parameter_address_response())
            .finish()
        } else {
            f.debug_tuple("M194ReadWriteParamResponse").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M194ReadWriteParamResponse {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d2_write_success = u.int_in_range(0..=1)? == 1;
        let d3_data_response = u.int_in_range(-32768..=32767)?;
        let d1_parameter_address_response = u.int_in_range(0..=65535)?;
        M194ReadWriteParamResponse::new(d2_write_success,d3_data_response,d1_parameter_address_response).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M193_Read_Write_Param_Command
///
/// - ID: 193 (0xc1)
/// - Size: 8 bytes
/// - Transmitter: VCU
///
/// Parameter Message sent as needed by VCU to request info, change EEPROM, or command a function.
#[derive(Clone, Copy)]
pub struct M193ReadWriteParamCommand {
    raw: [u8; 8],
}

impl M193ReadWriteParamCommand {
    pub const MESSAGE_ID: u32 = 193;
    
    pub const D3_DATA_COMMAND_MIN: i16 = -32768_i16;
    pub const D3_DATA_COMMAND_MAX: i16 = 32767_i16;
    pub const D1_PARAMETER_ADDRESS_COMMAND_MIN: u16 = 0_u16;
    pub const D1_PARAMETER_ADDRESS_COMMAND_MAX: u16 = 65535_u16;
    
    /// Construct new M193_Read_Write_Param_Command from values
    pub fn new(d3_data_command: i16, d2_read_write_command: bool, d1_parameter_address_command: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d3_data_command(d3_data_command)?;
        res.set_d2_read_write_command(d2_read_write_command)?;
        res.set_d1_parameter_address_command(d1_parameter_address_command)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D3_Data_Command
    ///
    /// All EEPROM data is 16 bits and is contained in bytes 4 and 5.
    ///
    /// - Min: -32768
    /// - Max: 32767
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_data_command(&self) -> i16 {
        self.d3_data_command_raw()
    }
    
    /// Get raw value of D3_Data_Command
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d3_data_command_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of D3_Data_Command
    #[inline(always)]
    pub fn set_d3_data_command(&mut self, value: i16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -32768_i16 || 32767_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 193 });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// D2_Read_Write_Command
    ///
    /// 0=Read, 1=Write
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_read_write_command(&self) -> bool {
        self.d2_read_write_command_raw()
    }
    
    /// Get raw value of D2_Read_Write_Command
    ///
    /// - Start bit: 16
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d2_read_write_command_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[16..17].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D2_Read_Write_Command
    #[inline(always)]
    pub fn set_d2_read_write_command(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[16..17].store_le(value);
        Ok(())
    }
    
    /// D1_Parameter_Address_Command
    ///
    /// Valid EEPROM Parameter CAN addresses are between 100 and 499.
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_parameter_address_command(&self) -> u16 {
        self.d1_parameter_address_command_raw()
    }
    
    /// Get raw value of D1_Parameter_Address_Command
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d1_parameter_address_command_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        signal
    }
    
    /// Set value of D1_Parameter_Address_Command
    #[inline(always)]
    pub fn set_d1_parameter_address_command(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 193 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M193ReadWriteParamCommand {
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
impl core::fmt::Debug for M193ReadWriteParamCommand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M193ReadWriteParamCommand")
                .field("d3_data_command", &self.d3_data_command())
                .field("d2_read_write_command", &self.d2_read_write_command())
                .field("d1_parameter_address_command", &self.d1_parameter_address_command())
            .finish()
        } else {
            f.debug_tuple("M193ReadWriteParamCommand").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M193ReadWriteParamCommand {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d3_data_command = u.int_in_range(-32768..=32767)?;
        let d2_read_write_command = u.int_in_range(0..=1)? == 1;
        let d1_parameter_address_command = u.int_in_range(0..=65535)?;
        M193ReadWriteParamCommand::new(d3_data_command,d2_read_write_command,d1_parameter_address_command).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M192_Command_Message
///
/// - ID: 192 (0xc0)
/// - Size: 8 bytes
/// - Transmitter: VCU
///
/// The command message is used to transmit data to the controller. This message is sent from a user supplied external controller to the PMxxx controller.
#[derive(Clone, Copy)]
pub struct M192CommandMessage {
    raw: [u8; 8],
}

impl M192CommandMessage {
    pub const MESSAGE_ID: u32 = 192;
    
    pub const SPEED_COMMAND_MIN: i16 = -32768_i16;
    pub const SPEED_COMMAND_MAX: i16 = 32767_i16;
    pub const TORQUE_COMMAND_MIN: f32 = -3276.8_f32;
    pub const TORQUE_COMMAND_MAX: f32 = 3276.7_f32;
    pub const TORQUE_LIMIT_COMMAND_MIN: f32 = -3276.8_f32;
    pub const TORQUE_LIMIT_COMMAND_MAX: f32 = 3276.7_f32;
    pub const ROLLING_COUNTER_MIN: u8 = 0_u8;
    pub const ROLLING_COUNTER_MAX: u8 = 15_u8;
    
    /// Construct new M192_Command_Message from values
    pub fn new(inverter_enable: bool, direction_command: bool, speed_command: i16, torque_command: f32, inverter_discharge: bool, torque_limit_command: f32, speed_mode_enable: bool, rolling_counter: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_inverter_enable(inverter_enable)?;
        res.set_direction_command(direction_command)?;
        res.set_speed_command(speed_command)?;
        res.set_torque_command(torque_command)?;
        res.set_inverter_discharge(inverter_discharge)?;
        res.set_torque_limit_command(torque_limit_command)?;
        res.set_speed_mode_enable(speed_mode_enable)?;
        res.set_rolling_counter(rolling_counter)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// Inverter_Enable
    ///
    /// 0=Inverter OFF, 1 = Inverter ON
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "Bit"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn inverter_enable(&self) -> M192CommandMessageInverterEnable {
        let signal = self.raw.view_bits::<Lsb0>()[40..41].load_le::<u8>();
        
        match signal {
            0 => M192CommandMessageInverterEnable::TurnTheInverterOff,
            1 => M192CommandMessageInverterEnable::TurnTheInverterOn,
            _ => M192CommandMessageInverterEnable::_Other(self.inverter_enable_raw()),
        }
    }
    
    /// Get raw value of Inverter_Enable
    ///
    /// - Start bit: 40
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn inverter_enable_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[40..41].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of Inverter_Enable
    #[inline(always)]
    pub fn set_inverter_enable(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[40..41].store_le(value);
        Ok(())
    }
    
    /// Direction_Command
    ///
    /// 0=CW, 1=CCW as veiwed from the shaft end of the motor
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "Bit"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn direction_command(&self) -> M192CommandMessageDirectionCommand {
        let signal = self.raw.view_bits::<Lsb0>()[32..33].load_le::<u8>();
        
        match signal {
            0 => M192CommandMessageDirectionCommand::Cw,
            1 => M192CommandMessageDirectionCommand::Ccw,
            _ => M192CommandMessageDirectionCommand::_Other(self.direction_command_raw()),
        }
    }
    
    /// Get raw value of Direction_Command
    ///
    /// - Start bit: 32
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn direction_command_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[32..33].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of Direction_Command
    #[inline(always)]
    pub fn set_direction_command(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[32..33].store_le(value);
        Ok(())
    }
    
    /// Speed_Command
    ///
    /// Speed commandused when in speed mode
    ///
    /// - Min: -32768
    /// - Max: 32767
    /// - Unit: "rpm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn speed_command(&self) -> i16 {
        self.speed_command_raw()
    }
    
    /// Get raw value of Speed_Command
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn speed_command_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of Speed_Command
    #[inline(always)]
    pub fn set_speed_command(&mut self, value: i16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -32768_i16 || 32767_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 192 });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// Torque_Command
    ///
    /// Torque command when in torque mode
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "Nm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn torque_command(&self) -> f32 {
        self.torque_command_raw()
    }
    
    /// Get raw value of Torque_Command
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn torque_command_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Torque_Command
    #[inline(always)]
    pub fn set_torque_command(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 192 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// Inverter_Discharge
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "Bit"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn inverter_discharge(&self) -> M192CommandMessageInverterDischarge {
        let signal = self.raw.view_bits::<Lsb0>()[41..42].load_le::<u8>();
        
        match signal {
            0 => M192CommandMessageInverterDischarge::DischargeDisable,
            1 => M192CommandMessageInverterDischarge::DischargeEnableIfEepromParameterIsSet,
            _ => M192CommandMessageInverterDischarge::_Other(self.inverter_discharge_raw()),
        }
    }
    
    /// Get raw value of Inverter_Discharge
    ///
    /// - Start bit: 41
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn inverter_discharge_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[41..42].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of Inverter_Discharge
    #[inline(always)]
    pub fn set_inverter_discharge(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[41..42].store_le(value);
        Ok(())
    }
    
    /// Torque_Limit_Command
    ///
    /// Torque Limit, set to 0 to keep default
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "Nm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn torque_limit_command(&self) -> f32 {
        self.torque_limit_command_raw()
    }
    
    /// Get raw value of Torque_Limit_Command
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn torque_limit_command_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Torque_Limit_Command
    #[inline(always)]
    pub fn set_torque_limit_command(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 192 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
    /// Speed_Mode_Enable
    ///
    /// 0 = No change to mode, 1 = change to speed mode from torque mode
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "Bit"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn speed_mode_enable(&self) -> bool {
        self.speed_mode_enable_raw()
    }
    
    /// Get raw value of Speed_Mode_Enable
    ///
    /// - Start bit: 42
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn speed_mode_enable_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[42..43].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of Speed_Mode_Enable
    #[inline(always)]
    pub fn set_speed_mode_enable(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[42..43].store_le(value);
        Ok(())
    }
    
    /// RollingCounter
    ///
    /// Rolling Counter command
    ///
    /// - Min: 0
    /// - Max: 15
    /// - Unit: "Bits"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn rolling_counter(&self) -> u8 {
        self.rolling_counter_raw()
    }
    
    /// Get raw value of RollingCounter
    ///
    /// - Start bit: 44
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn rolling_counter_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[44..48].load_le::<u8>();
        
        signal
    }
    
    /// Set value of RollingCounter
    #[inline(always)]
    pub fn set_rolling_counter(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 192 });
        }
        self.raw.view_bits_mut::<Lsb0>()[44..48].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M192CommandMessage {
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
impl core::fmt::Debug for M192CommandMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M192CommandMessage")
                .field("inverter_enable", &self.inverter_enable())
                .field("direction_command", &self.direction_command())
                .field("speed_command", &self.speed_command())
                .field("torque_command", &self.torque_command())
                .field("inverter_discharge", &self.inverter_discharge())
                .field("torque_limit_command", &self.torque_limit_command())
                .field("speed_mode_enable", &self.speed_mode_enable())
                .field("rolling_counter", &self.rolling_counter())
            .finish()
        } else {
            f.debug_tuple("M192CommandMessage").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M192CommandMessage {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let inverter_enable = u.int_in_range(0..=1)? == 1;
        let direction_command = u.int_in_range(0..=1)? == 1;
        let speed_command = u.int_in_range(-32768..=32767)?;
        let torque_command = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let inverter_discharge = u.int_in_range(0..=1)? == 1;
        let torque_limit_command = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let speed_mode_enable = u.int_in_range(0..=1)? == 1;
        let rolling_counter = u.int_in_range(0..=15)?;
        M192CommandMessage::new(inverter_enable,direction_command,speed_command,torque_command,inverter_discharge,torque_limit_command,speed_mode_enable,rolling_counter).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for Inverter_Enable
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum M192CommandMessageInverterEnable {
    TurnTheInverterOff,
    TurnTheInverterOn,
    _Other(bool),
}

impl Into<bool> for M192CommandMessageInverterEnable {
    fn into(self) -> bool {
        match self {
            M192CommandMessageInverterEnable::TurnTheInverterOff => false,
            M192CommandMessageInverterEnable::TurnTheInverterOn => true,
            M192CommandMessageInverterEnable::_Other(x) => x,
        }
    }
}

/// Defined values for Direction_Command
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum M192CommandMessageDirectionCommand {
    Cw,
    Ccw,
    _Other(bool),
}

impl Into<bool> for M192CommandMessageDirectionCommand {
    fn into(self) -> bool {
        match self {
            M192CommandMessageDirectionCommand::Cw => false,
            M192CommandMessageDirectionCommand::Ccw => true,
            M192CommandMessageDirectionCommand::_Other(x) => x,
        }
    }
}

/// Defined values for Inverter_Discharge
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum M192CommandMessageInverterDischarge {
    DischargeDisable,
    DischargeEnableIfEepromParameterIsSet,
    _Other(bool),
}

impl Into<bool> for M192CommandMessageInverterDischarge {
    fn into(self) -> bool {
        match self {
            M192CommandMessageInverterDischarge::DischargeDisable => false,
            M192CommandMessageInverterDischarge::DischargeEnableIfEepromParameterIsSet => true,
            M192CommandMessageInverterDischarge::_Other(x) => x,
        }
    }
}


/// M171_Fault_Codes
///
/// - ID: 171 (0xab)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M171FaultCodes {
    raw: [u8; 8],
}

impl M171FaultCodes {
    pub const MESSAGE_ID: u32 = 171;
    
    pub const D4_RUN_FAULT_HI_MIN: u16 = 0_u16;
    pub const D4_RUN_FAULT_HI_MAX: u16 = 65535_u16;
    pub const D2_POST_FAULT_HI_MIN: u16 = 0_u16;
    pub const D2_POST_FAULT_HI_MAX: u16 = 65535_u16;
    pub const D3_RUN_FAULT_LO_MIN: u16 = 0_u16;
    pub const D3_RUN_FAULT_LO_MAX: u16 = 65535_u16;
    pub const D1_POST_FAULT_LO_MIN: u16 = 0_u16;
    pub const D1_POST_FAULT_LO_MAX: u16 = 65535_u16;
    
    /// Construct new M171_Fault_Codes from values
    pub fn new(d4_run_fault_hi: u16, d2_post_fault_hi: u16, d3_run_fault_lo: u16, d1_post_fault_lo: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d4_run_fault_hi(d4_run_fault_hi)?;
        res.set_d2_post_fault_hi(d2_post_fault_hi)?;
        res.set_d3_run_fault_lo(d3_run_fault_lo)?;
        res.set_d1_post_fault_lo(d1_post_fault_lo)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D4_Run_Fault_Hi
    ///
    /// Each bit represents a fault. Please refer to PM100 Users Manual for details.
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_run_fault_hi(&self) -> u16 {
        self.d4_run_fault_hi_raw()
    }
    
    /// Get raw value of D4_Run_Fault_Hi
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d4_run_fault_hi_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        signal
    }
    
    /// Set value of D4_Run_Fault_Hi
    #[inline(always)]
    pub fn set_d4_run_fault_hi(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 171 });
        }
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
    /// D2_Post_Fault_Hi
    ///
    /// Each bit represents a fault. Please refer to PM100 Users Manual for details.
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_post_fault_hi(&self) -> u16 {
        self.d2_post_fault_hi_raw()
    }
    
    /// Get raw value of D2_Post_Fault_Hi
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d2_post_fault_hi_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        signal
    }
    
    /// Set value of D2_Post_Fault_Hi
    #[inline(always)]
    pub fn set_d2_post_fault_hi(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 171 });
        }
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// D3_Run_Fault_Lo
    ///
    /// Each bit represents a fault. Please refer to PM100 Users Manual for details.
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_run_fault_lo(&self) -> u16 {
        self.d3_run_fault_lo_raw()
    }
    
    /// Get raw value of D3_Run_Fault_Lo
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d3_run_fault_lo_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        signal
    }
    
    /// Set value of D3_Run_Fault_Lo
    #[inline(always)]
    pub fn set_d3_run_fault_lo(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 171 });
        }
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// D1_Post_Fault_Lo
    ///
    /// Each bit represents a fault. Please refer to PM100 Users Manual for details.
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_post_fault_lo(&self) -> u16 {
        self.d1_post_fault_lo_raw()
    }
    
    /// Get raw value of D1_Post_Fault_Lo
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d1_post_fault_lo_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        signal
    }
    
    /// Set value of D1_Post_Fault_Lo
    #[inline(always)]
    pub fn set_d1_post_fault_lo(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 171 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M171FaultCodes {
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
impl core::fmt::Debug for M171FaultCodes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M171FaultCodes")
                .field("d4_run_fault_hi", &self.d4_run_fault_hi())
                .field("d2_post_fault_hi", &self.d2_post_fault_hi())
                .field("d3_run_fault_lo", &self.d3_run_fault_lo())
                .field("d1_post_fault_lo", &self.d1_post_fault_lo())
            .finish()
        } else {
            f.debug_tuple("M171FaultCodes").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M171FaultCodes {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d4_run_fault_hi = u.int_in_range(0..=65535)?;
        let d2_post_fault_hi = u.int_in_range(0..=65535)?;
        let d3_run_fault_lo = u.int_in_range(0..=65535)?;
        let d1_post_fault_lo = u.int_in_range(0..=65535)?;
        M171FaultCodes::new(d4_run_fault_hi,d2_post_fault_hi,d3_run_fault_lo,d1_post_fault_lo).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M170_Internal_States
///
/// - ID: 170 (0xaa)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M170InternalStates {
    raw: [u8; 8],
}

impl M170InternalStates {
    pub const MESSAGE_ID: u32 = 170;
    
    pub const D2_INVERTER_STATE_MIN: u8 = 0_u8;
    pub const D2_INVERTER_STATE_MAX: u8 = 255_u8;
    pub const D1_VSM_STATE_MIN: u8 = 0_u8;
    pub const D1_VSM_STATE_MAX: u8 = 15_u8;
    pub const D4_INVERTER_DISCHARGE_STATE_MIN: u8 = 0_u8;
    pub const D4_INVERTER_DISCHARGE_STATE_MAX: u8 = 7_u8;
    pub const D5_ROLLING_COUNTER_MIN: u8 = 0_u8;
    pub const D5_ROLLING_COUNTER_MAX: u8 = 15_u8;
    pub const D1_PWM_FREQUENCY_MIN: u8 = 0_u8;
    pub const D1_PWM_FREQUENCY_MAX: u8 = 255_u8;
    
    /// Construct new M170_Internal_States from values
    pub fn new(d7_direction_command: bool, d6_inverter_enable_state: bool, d3_relay_3_status: bool, d3_relay_4_status: bool, d3_relay_2_status: bool, d4_inverter_run_mode: bool, d5_inverter_command_mode: bool, d3_relay_1_status: bool, d2_inverter_state: u8, d1_vsm_state: u8, d6_inverter_enable_lockout: bool, d4_inverter_discharge_state: u8, d3_relay_5_status: bool, d3_relay_6_status: bool, d7_bms_active: bool, d7_bms_torque_limiting: bool, d7_max_speed_limiting: bool, d7_low_speed_limiting: bool, d5_rolling_counter: u8, d1_pwm_frequency: u8, d6_start_mode_active: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d7_direction_command(d7_direction_command)?;
        res.set_d6_inverter_enable_state(d6_inverter_enable_state)?;
        res.set_d3_relay_3_status(d3_relay_3_status)?;
        res.set_d3_relay_4_status(d3_relay_4_status)?;
        res.set_d3_relay_2_status(d3_relay_2_status)?;
        res.set_d4_inverter_run_mode(d4_inverter_run_mode)?;
        res.set_d5_inverter_command_mode(d5_inverter_command_mode)?;
        res.set_d3_relay_1_status(d3_relay_1_status)?;
        res.set_d2_inverter_state(d2_inverter_state)?;
        res.set_d1_vsm_state(d1_vsm_state)?;
        res.set_d6_inverter_enable_lockout(d6_inverter_enable_lockout)?;
        res.set_d4_inverter_discharge_state(d4_inverter_discharge_state)?;
        res.set_d3_relay_5_status(d3_relay_5_status)?;
        res.set_d3_relay_6_status(d3_relay_6_status)?;
        res.set_d7_bms_active(d7_bms_active)?;
        res.set_d7_bms_torque_limiting(d7_bms_torque_limiting)?;
        res.set_d7_max_speed_limiting(d7_max_speed_limiting)?;
        res.set_d7_low_speed_limiting(d7_low_speed_limiting)?;
        res.set_d5_rolling_counter(d5_rolling_counter)?;
        res.set_d1_pwm_frequency(d1_pwm_frequency)?;
        res.set_d6_start_mode_active(d6_start_mode_active)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D7_Direction_Command
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d7_direction_command(&self) -> bool {
        self.d7_direction_command_raw()
    }
    
    /// Get raw value of D7_Direction_Command
    ///
    /// - Start bit: 56
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d7_direction_command_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[56..57].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D7_Direction_Command
    #[inline(always)]
    pub fn set_d7_direction_command(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[56..57].store_le(value);
        Ok(())
    }
    
    /// D6_Inverter_Enable_State
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d6_inverter_enable_state(&self) -> bool {
        self.d6_inverter_enable_state_raw()
    }
    
    /// Get raw value of D6_Inverter_Enable_State
    ///
    /// - Start bit: 48
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d6_inverter_enable_state_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[48..49].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D6_Inverter_Enable_State
    #[inline(always)]
    pub fn set_d6_inverter_enable_state(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[48..49].store_le(value);
        Ok(())
    }
    
    /// D3_Relay_3_Status
    ///
    /// 0=OFF, 1=ON
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_relay_3_status(&self) -> bool {
        self.d3_relay_3_status_raw()
    }
    
    /// Get raw value of D3_Relay_3_Status
    ///
    /// - Start bit: 26
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d3_relay_3_status_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[26..27].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D3_Relay_3_Status
    #[inline(always)]
    pub fn set_d3_relay_3_status(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[26..27].store_le(value);
        Ok(())
    }
    
    /// D3_Relay_4_Status
    ///
    /// 0=OFF, 1=ON
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_relay_4_status(&self) -> bool {
        self.d3_relay_4_status_raw()
    }
    
    /// Get raw value of D3_Relay_4_Status
    ///
    /// - Start bit: 27
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d3_relay_4_status_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[27..28].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D3_Relay_4_Status
    #[inline(always)]
    pub fn set_d3_relay_4_status(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[27..28].store_le(value);
        Ok(())
    }
    
    /// D3_Relay_2_Status
    ///
    /// 0=OFF, 1=ON
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_relay_2_status(&self) -> bool {
        self.d3_relay_2_status_raw()
    }
    
    /// Get raw value of D3_Relay_2_Status
    ///
    /// - Start bit: 25
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d3_relay_2_status_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[25..26].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D3_Relay_2_Status
    #[inline(always)]
    pub fn set_d3_relay_2_status(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[25..26].store_le(value);
        Ok(())
    }
    
    /// D4_Inverter_Run_Mode
    ///
    /// 0=Torque Mode, 1=Speed Mode
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_inverter_run_mode(&self) -> bool {
        self.d4_inverter_run_mode_raw()
    }
    
    /// Get raw value of D4_Inverter_Run_Mode
    ///
    /// - Start bit: 32
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d4_inverter_run_mode_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[32..33].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D4_Inverter_Run_Mode
    #[inline(always)]
    pub fn set_d4_inverter_run_mode(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[32..33].store_le(value);
        Ok(())
    }
    
    /// D5_Inverter_Command_Mode
    ///
    /// 0=CAN mode, 1=VSM mode
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d5_inverter_command_mode(&self) -> bool {
        self.d5_inverter_command_mode_raw()
    }
    
    /// Get raw value of D5_Inverter_Command_Mode
    ///
    /// - Start bit: 40
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d5_inverter_command_mode_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[40..41].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D5_Inverter_Command_Mode
    #[inline(always)]
    pub fn set_d5_inverter_command_mode(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[40..41].store_le(value);
        Ok(())
    }
    
    /// D3_Relay_1_Status
    ///
    /// 0=OFF, 1=ON
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_relay_1_status(&self) -> bool {
        self.d3_relay_1_status_raw()
    }
    
    /// Get raw value of D3_Relay_1_Status
    ///
    /// - Start bit: 24
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d3_relay_1_status_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[24..25].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D3_Relay_1_Status
    #[inline(always)]
    pub fn set_d3_relay_1_status(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[24..25].store_le(value);
        Ok(())
    }
    
    
    /// Get raw value of D2_Inverter_State
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d2_inverter_state_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
        
        signal
    }
    
    /// Set value of D2_Inverter_State
    #[inline(always)]
    pub fn set_d2_inverter_state(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 170 });
        }
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
    /// D1_VSM_State
    ///
    /// Different states for the vehicle state machine
    ///
    /// - Min: 0
    /// - Max: 15
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_vsm_state(&self) -> M170InternalStatesD1VsmState {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        match signal {
            0 => M170InternalStatesD1VsmState::VsmStartState,
            1 => M170InternalStatesD1VsmState::PreChargeInitState,
            2 => M170InternalStatesD1VsmState::PreChargeActiveState,
            3 => M170InternalStatesD1VsmState::PreChargeCompleteState,
            4 => M170InternalStatesD1VsmState::VsmWaitState,
            5 => M170InternalStatesD1VsmState::VsmReadyState,
            6 => M170InternalStatesD1VsmState::MotorRunningState,
            7 => M170InternalStatesD1VsmState::BlinkFaultCodeState,
            14 => M170InternalStatesD1VsmState::ShutdownStateForKeySwitchMode1,
            15 => M170InternalStatesD1VsmState::ResetTheInverter,
            _ => M170InternalStatesD1VsmState::_Other(self.d1_vsm_state_raw()),
        }
    }
    
    /// Get raw value of D1_VSM_State
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d1_vsm_state_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        signal
    }
    
    /// Set value of D1_VSM_State
    #[inline(always)]
    pub fn set_d1_vsm_state(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 170 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// D6_Inverter_Enable_Lockout
    ///
    /// 0=Lockout Disabled, 1=Lockout Enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d6_inverter_enable_lockout(&self) -> bool {
        self.d6_inverter_enable_lockout_raw()
    }
    
    /// Get raw value of D6_Inverter_Enable_Lockout
    ///
    /// - Start bit: 55
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d6_inverter_enable_lockout_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[55..56].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D6_Inverter_Enable_Lockout
    #[inline(always)]
    pub fn set_d6_inverter_enable_lockout(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[55..56].store_le(value);
        Ok(())
    }
    
    /// D4_Inverter_Discharge_State
    ///
    /// 0 = Disabled, 1 = Enabled, 2 = Speed Check, 3 = Active, 4 = Complete
    ///
    /// - Min: 0
    /// - Max: 7
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_inverter_discharge_state(&self) -> u8 {
        self.d4_inverter_discharge_state_raw()
    }
    
    /// Get raw value of D4_Inverter_Discharge_State
    ///
    /// - Start bit: 37
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d4_inverter_discharge_state_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[37..40].load_le::<u8>();
        
        signal
    }
    
    /// Set value of D4_Inverter_Discharge_State
    #[inline(always)]
    pub fn set_d4_inverter_discharge_state(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 7_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 170 });
        }
        self.raw.view_bits_mut::<Lsb0>()[37..40].store_le(value);
        Ok(())
    }
    
    /// D3_Relay_5_Status
    ///
    /// 0=OFF, 1=ON
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_relay_5_status(&self) -> bool {
        self.d3_relay_5_status_raw()
    }
    
    /// Get raw value of D3_Relay_5_Status
    ///
    /// - Start bit: 28
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d3_relay_5_status_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[28..29].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D3_Relay_5_Status
    #[inline(always)]
    pub fn set_d3_relay_5_status(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[28..29].store_le(value);
        Ok(())
    }
    
    /// D3_Relay_6_Status
    ///
    /// 0=OFF, 1=ON
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_relay_6_status(&self) -> bool {
        self.d3_relay_6_status_raw()
    }
    
    /// Get raw value of D3_Relay_6_Status
    ///
    /// - Start bit: 29
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d3_relay_6_status_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[29..30].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D3_Relay_6_Status
    #[inline(always)]
    pub fn set_d3_relay_6_status(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[29..30].store_le(value);
        Ok(())
    }
    
    /// D7_BMS_Active
    ///
    /// 0 = BMS Not Active, 1 = BMS Active
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d7_bms_active(&self) -> bool {
        self.d7_bms_active_raw()
    }
    
    /// Get raw value of D7_BMS_Active
    ///
    /// - Start bit: 57
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d7_bms_active_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[57..58].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D7_BMS_Active
    #[inline(always)]
    pub fn set_d7_bms_active(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[57..58].store_le(value);
        Ok(())
    }
    
    /// D7_BMS_Torque_Limiting
    ///
    /// 0 = Not Limiting, 1 = Limiting
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d7_bms_torque_limiting(&self) -> bool {
        self.d7_bms_torque_limiting_raw()
    }
    
    /// Get raw value of D7_BMS_Torque_Limiting
    ///
    /// - Start bit: 58
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d7_bms_torque_limiting_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[58..59].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D7_BMS_Torque_Limiting
    #[inline(always)]
    pub fn set_d7_bms_torque_limiting(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[58..59].store_le(value);
        Ok(())
    }
    
    /// D7_Max_Speed_Limiting
    ///
    /// 0 = Not Limiting, 1 = torque limiting due to maximum speed
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d7_max_speed_limiting(&self) -> bool {
        self.d7_max_speed_limiting_raw()
    }
    
    /// Get raw value of D7_Max_Speed_Limiting
    ///
    /// - Start bit: 59
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d7_max_speed_limiting_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[59..60].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D7_Max_Speed_Limiting
    #[inline(always)]
    pub fn set_d7_max_speed_limiting(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[59..60].store_le(value);
        Ok(())
    }
    
    /// D7_Low_Speed_Limiting
    ///
    /// 0 = Not Limiting, 1 = Current limiting due to low motor speed
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d7_low_speed_limiting(&self) -> bool {
        self.d7_low_speed_limiting_raw()
    }
    
    /// Get raw value of D7_Low_Speed_Limiting
    ///
    /// - Start bit: 61
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d7_low_speed_limiting_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[61..62].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D7_Low_Speed_Limiting
    #[inline(always)]
    pub fn set_d7_low_speed_limiting(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[61..62].store_le(value);
        Ok(())
    }
    
    /// D5_Rolling_Counter
    ///
    /// The current rolling counter value.
    ///
    /// - Min: 0
    /// - Max: 15
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d5_rolling_counter(&self) -> u8 {
        self.d5_rolling_counter_raw()
    }
    
    /// Get raw value of D5_Rolling_Counter
    ///
    /// - Start bit: 44
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d5_rolling_counter_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[44..48].load_le::<u8>();
        
        signal
    }
    
    /// Set value of D5_Rolling_Counter
    #[inline(always)]
    pub fn set_d5_rolling_counter(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 170 });
        }
        self.raw.view_bits_mut::<Lsb0>()[44..48].store_le(value);
        Ok(())
    }
    
    /// D1_PWM_Frequency
    ///
    /// The inverter PWM frequency
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: "kHz"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_pwm_frequency(&self) -> u8 {
        self.d1_pwm_frequency_raw()
    }
    
    /// Get raw value of D1_PWM_Frequency
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d1_pwm_frequency_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        signal
    }
    
    /// Set value of D1_PWM_Frequency
    #[inline(always)]
    pub fn set_d1_pwm_frequency(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 170 });
        }
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// D6_Start_Mode_Active
    ///
    /// If in Key Switch Mode 1 then indicates status of Start input being applied. 0 = not active, 1 = active
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d6_start_mode_active(&self) -> bool {
        self.d6_start_mode_active_raw()
    }
    
    /// Get raw value of D6_Start_Mode_Active
    ///
    /// - Start bit: 54
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d6_start_mode_active_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[54..55].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D6_Start_Mode_Active
    #[inline(always)]
    pub fn set_d6_start_mode_active(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[54..55].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M170InternalStates {
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
impl core::fmt::Debug for M170InternalStates {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M170InternalStates")
                .field("d7_direction_command", &self.d7_direction_command())
                .field("d6_inverter_enable_state", &self.d6_inverter_enable_state())
                .field("d3_relay_3_status", &self.d3_relay_3_status())
                .field("d3_relay_4_status", &self.d3_relay_4_status())
                .field("d3_relay_2_status", &self.d3_relay_2_status())
                .field("d4_inverter_run_mode", &self.d4_inverter_run_mode())
                .field("d5_inverter_command_mode", &self.d5_inverter_command_mode())
                .field("d3_relay_1_status", &self.d3_relay_1_status())
                .field("d2_inverter_state", &self.d2_inverter_state())
                .field("d1_vsm_state", &self.d1_vsm_state())
                .field("d6_inverter_enable_lockout", &self.d6_inverter_enable_lockout())
                .field("d4_inverter_discharge_state", &self.d4_inverter_discharge_state())
                .field("d3_relay_5_status", &self.d3_relay_5_status())
                .field("d3_relay_6_status", &self.d3_relay_6_status())
                .field("d7_bms_active", &self.d7_bms_active())
                .field("d7_bms_torque_limiting", &self.d7_bms_torque_limiting())
                .field("d7_max_speed_limiting", &self.d7_max_speed_limiting())
                .field("d7_low_speed_limiting", &self.d7_low_speed_limiting())
                .field("d5_rolling_counter", &self.d5_rolling_counter())
                .field("d1_pwm_frequency", &self.d1_pwm_frequency())
                .field("d6_start_mode_active", &self.d6_start_mode_active())
            .finish()
        } else {
            f.debug_tuple("M170InternalStates").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M170InternalStates {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d7_direction_command = u.int_in_range(0..=1)? == 1;
        let d6_inverter_enable_state = u.int_in_range(0..=1)? == 1;
        let d3_relay_3_status = u.int_in_range(0..=1)? == 1;
        let d3_relay_4_status = u.int_in_range(0..=1)? == 1;
        let d3_relay_2_status = u.int_in_range(0..=1)? == 1;
        let d4_inverter_run_mode = u.int_in_range(0..=1)? == 1;
        let d5_inverter_command_mode = u.int_in_range(0..=1)? == 1;
        let d3_relay_1_status = u.int_in_range(0..=1)? == 1;
        let d2_inverter_state = u.int_in_range(0..=255)?;
        let d1_vsm_state = u.int_in_range(0..=15)?;
        let d6_inverter_enable_lockout = u.int_in_range(0..=1)? == 1;
        let d4_inverter_discharge_state = u.int_in_range(0..=7)?;
        let d3_relay_5_status = u.int_in_range(0..=1)? == 1;
        let d3_relay_6_status = u.int_in_range(0..=1)? == 1;
        let d7_bms_active = u.int_in_range(0..=1)? == 1;
        let d7_bms_torque_limiting = u.int_in_range(0..=1)? == 1;
        let d7_max_speed_limiting = u.int_in_range(0..=1)? == 1;
        let d7_low_speed_limiting = u.int_in_range(0..=1)? == 1;
        let d5_rolling_counter = u.int_in_range(0..=15)?;
        let d1_pwm_frequency = u.int_in_range(0..=255)?;
        let d6_start_mode_active = u.int_in_range(0..=1)? == 1;
        M170InternalStates::new(d7_direction_command,d6_inverter_enable_state,d3_relay_3_status,d3_relay_4_status,d3_relay_2_status,d4_inverter_run_mode,d5_inverter_command_mode,d3_relay_1_status,d2_inverter_state,d1_vsm_state,d6_inverter_enable_lockout,d4_inverter_discharge_state,d3_relay_5_status,d3_relay_6_status,d7_bms_active,d7_bms_torque_limiting,d7_max_speed_limiting,d7_low_speed_limiting,d5_rolling_counter,d1_pwm_frequency,d6_start_mode_active).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}


/// Defined values for D1_VSM_State
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum M170InternalStatesD1VsmState {
    VsmStartState,
    PreChargeInitState,
    PreChargeActiveState,
    PreChargeCompleteState,
    VsmWaitState,
    VsmReadyState,
    MotorRunningState,
    BlinkFaultCodeState,
    ShutdownStateForKeySwitchMode1,
    ResetTheInverter,
    _Other(u8),
}

impl Into<u8> for M170InternalStatesD1VsmState {
    fn into(self) -> u8 {
        match self {
            M170InternalStatesD1VsmState::VsmStartState => 0,
            M170InternalStatesD1VsmState::PreChargeInitState => 1,
            M170InternalStatesD1VsmState::PreChargeActiveState => 2,
            M170InternalStatesD1VsmState::PreChargeCompleteState => 3,
            M170InternalStatesD1VsmState::VsmWaitState => 4,
            M170InternalStatesD1VsmState::VsmReadyState => 5,
            M170InternalStatesD1VsmState::MotorRunningState => 6,
            M170InternalStatesD1VsmState::BlinkFaultCodeState => 7,
            M170InternalStatesD1VsmState::ShutdownStateForKeySwitchMode1 => 14,
            M170InternalStatesD1VsmState::ResetTheInverter => 15,
            M170InternalStatesD1VsmState::_Other(x) => x,
        }
    }
}


/// M169_Internal_Voltages
///
/// - ID: 169 (0xa9)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M169InternalVoltages {
    raw: [u8; 8],
}

impl M169InternalVoltages {
    pub const MESSAGE_ID: u32 = 169;
    
    pub const D4_REFERENCE_VOLTAGE_12_0_MIN: f32 = -327.68_f32;
    pub const D4_REFERENCE_VOLTAGE_12_0_MAX: f32 = 327.67_f32;
    pub const D3_REFERENCE_VOLTAGE_5_0_MIN: f32 = -327.68_f32;
    pub const D3_REFERENCE_VOLTAGE_5_0_MAX: f32 = 327.67_f32;
    pub const D2_REFERENCE_VOLTAGE_2_5_MIN: f32 = -327.68_f32;
    pub const D2_REFERENCE_VOLTAGE_2_5_MAX: f32 = 327.67_f32;
    pub const D1_REFERENCE_VOLTAGE_1_5_MIN: f32 = -327.68_f32;
    pub const D1_REFERENCE_VOLTAGE_1_5_MAX: f32 = 327.67_f32;
    
    /// Construct new M169_Internal_Voltages from values
    pub fn new(d4_reference_voltage_12_0: f32, d3_reference_voltage_5_0: f32, d2_reference_voltage_2_5: f32, d1_reference_voltage_1_5: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d4_reference_voltage_12_0(d4_reference_voltage_12_0)?;
        res.set_d3_reference_voltage_5_0(d3_reference_voltage_5_0)?;
        res.set_d2_reference_voltage_2_5(d2_reference_voltage_2_5)?;
        res.set_d1_reference_voltage_1_5(d1_reference_voltage_1_5)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D4_Reference_Voltage_12_0
    ///
    /// 12V Input Voltage
    ///
    /// - Min: -327.68
    /// - Max: 327.67
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_reference_voltage_12_0(&self) -> f32 {
        self.d4_reference_voltage_12_0_raw()
    }
    
    /// Get raw value of D4_Reference_Voltage_12_0
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d4_reference_voltage_12_0_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D4_Reference_Voltage_12_0
    #[inline(always)]
    pub fn set_d4_reference_voltage_12_0(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -327.68_f32 || 327.67_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 169 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
    /// D3_Reference_Voltage_5_0
    ///
    /// Transducer voltage
    ///
    /// - Min: -327.68
    /// - Max: 327.67
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_reference_voltage_5_0(&self) -> f32 {
        self.d3_reference_voltage_5_0_raw()
    }
    
    /// Get raw value of D3_Reference_Voltage_5_0
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d3_reference_voltage_5_0_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D3_Reference_Voltage_5_0
    #[inline(always)]
    pub fn set_d3_reference_voltage_5_0(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -327.68_f32 || 327.67_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 169 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// D2_Reference_Voltage_2_5
    ///
    /// Internal reference voltage
    ///
    /// - Min: -327.68
    /// - Max: 327.67
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_reference_voltage_2_5(&self) -> f32 {
        self.d2_reference_voltage_2_5_raw()
    }
    
    /// Get raw value of D2_Reference_Voltage_2_5
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d2_reference_voltage_2_5_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D2_Reference_Voltage_2_5
    #[inline(always)]
    pub fn set_d2_reference_voltage_2_5(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -327.68_f32 || 327.67_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 169 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// D1_Reference_Voltage_1_5
    ///
    /// Internal reference voltage
    ///
    /// - Min: -327.68
    /// - Max: 327.67
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_reference_voltage_1_5(&self) -> f32 {
        self.d1_reference_voltage_1_5_raw()
    }
    
    /// Get raw value of D1_Reference_Voltage_1_5
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d1_reference_voltage_1_5_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D1_Reference_Voltage_1_5
    #[inline(always)]
    pub fn set_d1_reference_voltage_1_5(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -327.68_f32 || 327.67_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 169 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M169InternalVoltages {
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
impl core::fmt::Debug for M169InternalVoltages {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M169InternalVoltages")
                .field("d4_reference_voltage_12_0", &self.d4_reference_voltage_12_0())
                .field("d3_reference_voltage_5_0", &self.d3_reference_voltage_5_0())
                .field("d2_reference_voltage_2_5", &self.d2_reference_voltage_2_5())
                .field("d1_reference_voltage_1_5", &self.d1_reference_voltage_1_5())
            .finish()
        } else {
            f.debug_tuple("M169InternalVoltages").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M169InternalVoltages {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d4_reference_voltage_12_0 = u.float_in_range(-327.68_f32..=327.67_f32)?;
        let d3_reference_voltage_5_0 = u.float_in_range(-327.68_f32..=327.67_f32)?;
        let d2_reference_voltage_2_5 = u.float_in_range(-327.68_f32..=327.67_f32)?;
        let d1_reference_voltage_1_5 = u.float_in_range(-327.68_f32..=327.67_f32)?;
        M169InternalVoltages::new(d4_reference_voltage_12_0,d3_reference_voltage_5_0,d2_reference_voltage_2_5,d1_reference_voltage_1_5).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M168_Flux_ID_IQ_Info
///
/// - ID: 168 (0xa8)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M168FluxIdIqInfo {
    raw: [u8; 8],
}

impl M168FluxIdIqInfo {
    pub const MESSAGE_ID: u32 = 168;
    
    pub const D4_IQ_MIN: f32 = -3276.8_f32;
    pub const D4_IQ_MAX: f32 = 3276.7_f32;
    pub const D3_ID_MIN: f32 = -3276.8_f32;
    pub const D3_ID_MAX: f32 = 3276.7_f32;
    pub const D2_FLUX_FEEDBACK_MIN: f32 = -32.768_f32;
    pub const D2_FLUX_FEEDBACK_MAX: f32 = 32.767_f32;
    pub const D1_FLUX_COMMAND_MIN: f32 = -32.768_f32;
    pub const D1_FLUX_COMMAND_MAX: f32 = 32.767_f32;
    
    /// Construct new M168_Flux_ID_IQ_Info from values
    pub fn new(d4_iq: f32, d3_id: f32, d2_flux_feedback: f32, d1_flux_command: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d4_iq(d4_iq)?;
        res.set_d3_id(d3_id)?;
        res.set_d2_flux_feedback(d2_flux_feedback)?;
        res.set_d1_flux_command(d1_flux_command)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D4_Iq
    ///
    /// The measured Iq current
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_iq(&self) -> f32 {
        self.d4_iq_raw()
    }
    
    /// Get raw value of D4_Iq
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d4_iq_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D4_Iq
    #[inline(always)]
    pub fn set_d4_iq(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 168 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
    /// D3_Id
    ///
    /// The measured Id current
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_id(&self) -> f32 {
        self.d3_id_raw()
    }
    
    /// Get raw value of D3_Id
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d3_id_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D3_Id
    #[inline(always)]
    pub fn set_d3_id(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 168 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// D2_Flux_Feedback
    ///
    /// The estimated flux
    ///
    /// - Min: -32.768
    /// - Max: 32.767
    /// - Unit: "Wb"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_flux_feedback(&self) -> f32 {
        self.d2_flux_feedback_raw()
    }
    
    /// Get raw value of D2_Flux_Feedback
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d2_flux_feedback_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D2_Flux_Feedback
    #[inline(always)]
    pub fn set_d2_flux_feedback(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -32.768_f32 || 32.767_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 168 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// D1_Flux_Command
    ///
    /// The commanded flux
    ///
    /// - Min: -32.768
    /// - Max: 32.767
    /// - Unit: "Wb"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_flux_command(&self) -> f32 {
        self.d1_flux_command_raw()
    }
    
    /// Get raw value of D1_Flux_Command
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d1_flux_command_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D1_Flux_Command
    #[inline(always)]
    pub fn set_d1_flux_command(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -32.768_f32 || 32.767_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 168 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M168FluxIdIqInfo {
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
impl core::fmt::Debug for M168FluxIdIqInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M168FluxIdIqInfo")
                .field("d4_iq", &self.d4_iq())
                .field("d3_id", &self.d3_id())
                .field("d2_flux_feedback", &self.d2_flux_feedback())
                .field("d1_flux_command", &self.d1_flux_command())
            .finish()
        } else {
            f.debug_tuple("M168FluxIdIqInfo").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M168FluxIdIqInfo {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d4_iq = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d3_id = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d2_flux_feedback = u.float_in_range(-32.768_f32..=32.767_f32)?;
        let d1_flux_command = u.float_in_range(-32.768_f32..=32.767_f32)?;
        M168FluxIdIqInfo::new(d4_iq,d3_id,d2_flux_feedback,d1_flux_command).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M167_Voltage_Info
///
/// - ID: 167 (0xa7)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M167VoltageInfo {
    raw: [u8; 8],
}

impl M167VoltageInfo {
    pub const MESSAGE_ID: u32 = 167;
    
    pub const D4_VBC_VQ_VOLTAGE_MIN: f32 = -3276.8_f32;
    pub const D4_VBC_VQ_VOLTAGE_MAX: f32 = 3276.7_f32;
    pub const D3_VAB_VD_VOLTAGE_MIN: f32 = -3276.8_f32;
    pub const D3_VAB_VD_VOLTAGE_MAX: f32 = 3276.7_f32;
    pub const D2_OUTPUT_VOLTAGE_MIN: f32 = -3276.8_f32;
    pub const D2_OUTPUT_VOLTAGE_MAX: f32 = 3276.7_f32;
    pub const D1_DC_BUS_VOLTAGE_MIN: f32 = -3276.8_f32;
    pub const D1_DC_BUS_VOLTAGE_MAX: f32 = 3276.7_f32;
    
    /// Construct new M167_Voltage_Info from values
    pub fn new(d4_vbc_vq_voltage: f32, d3_vab_vd_voltage: f32, d2_output_voltage: f32, d1_dc_bus_voltage: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d4_vbc_vq_voltage(d4_vbc_vq_voltage)?;
        res.set_d3_vab_vd_voltage(d3_vab_vd_voltage)?;
        res.set_d2_output_voltage(d2_output_voltage)?;
        res.set_d1_dc_bus_voltage(d1_dc_bus_voltage)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D4_VBC_Vq_Voltage
    ///
    /// Measured value of the voltage between Phase B and Phase C
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_vbc_vq_voltage(&self) -> f32 {
        self.d4_vbc_vq_voltage_raw()
    }
    
    /// Get raw value of D4_VBC_Vq_Voltage
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d4_vbc_vq_voltage_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D4_VBC_Vq_Voltage
    #[inline(always)]
    pub fn set_d4_vbc_vq_voltage(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 167 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
    /// D3_VAB_Vd_Voltage
    ///
    /// Measured value of the voltage betwen phase A and Phase B
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_vab_vd_voltage(&self) -> f32 {
        self.d3_vab_vd_voltage_raw()
    }
    
    /// Get raw value of D3_VAB_Vd_Voltage
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d3_vab_vd_voltage_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D3_VAB_Vd_Voltage
    #[inline(always)]
    pub fn set_d3_vab_vd_voltage(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 167 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// D2_Output_Voltage
    ///
    /// The calculated value of the output voltage, in peak line-neutral volts
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_output_voltage(&self) -> f32 {
        self.d2_output_voltage_raw()
    }
    
    /// Get raw value of D2_Output_Voltage
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d2_output_voltage_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D2_Output_Voltage
    #[inline(always)]
    pub fn set_d2_output_voltage(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 167 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// D1_DC_Bus_Voltage
    ///
    /// The actual measured value of the DC bus voltage
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_dc_bus_voltage(&self) -> f32 {
        self.d1_dc_bus_voltage_raw()
    }
    
    /// Get raw value of D1_DC_Bus_Voltage
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d1_dc_bus_voltage_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D1_DC_Bus_Voltage
    #[inline(always)]
    pub fn set_d1_dc_bus_voltage(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 167 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M167VoltageInfo {
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
impl core::fmt::Debug for M167VoltageInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M167VoltageInfo")
                .field("d4_vbc_vq_voltage", &self.d4_vbc_vq_voltage())
                .field("d3_vab_vd_voltage", &self.d3_vab_vd_voltage())
                .field("d2_output_voltage", &self.d2_output_voltage())
                .field("d1_dc_bus_voltage", &self.d1_dc_bus_voltage())
            .finish()
        } else {
            f.debug_tuple("M167VoltageInfo").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M167VoltageInfo {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d4_vbc_vq_voltage = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d3_vab_vd_voltage = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d2_output_voltage = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d1_dc_bus_voltage = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        M167VoltageInfo::new(d4_vbc_vq_voltage,d3_vab_vd_voltage,d2_output_voltage,d1_dc_bus_voltage).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M166_Current_Info
///
/// - ID: 166 (0xa6)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M166CurrentInfo {
    raw: [u8; 8],
}

impl M166CurrentInfo {
    pub const MESSAGE_ID: u32 = 166;
    
    pub const D4_DC_BUS_CURRENT_MIN: f32 = -3276.8_f32;
    pub const D4_DC_BUS_CURRENT_MAX: f32 = 3276.7_f32;
    pub const D3_PHASE_C_CURRENT_MIN: f32 = -3276.8_f32;
    pub const D3_PHASE_C_CURRENT_MAX: f32 = 3276.7_f32;
    pub const D2_PHASE_B_CURRENT_MIN: f32 = -3276.8_f32;
    pub const D2_PHASE_B_CURRENT_MAX: f32 = 3276.7_f32;
    pub const D1_PHASE_A_CURRENT_MIN: f32 = -3276.8_f32;
    pub const D1_PHASE_A_CURRENT_MAX: f32 = 3276.7_f32;
    
    /// Construct new M166_Current_Info from values
    pub fn new(d4_dc_bus_current: f32, d3_phase_c_current: f32, d2_phase_b_current: f32, d1_phase_a_current: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d4_dc_bus_current(d4_dc_bus_current)?;
        res.set_d3_phase_c_current(d3_phase_c_current)?;
        res.set_d2_phase_b_current(d2_phase_b_current)?;
        res.set_d1_phase_a_current(d1_phase_a_current)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D4_DC_Bus_Current
    ///
    /// The Calculated DC Bus Current
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_dc_bus_current(&self) -> f32 {
        self.d4_dc_bus_current_raw()
    }
    
    /// Get raw value of D4_DC_Bus_Current
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d4_dc_bus_current_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D4_DC_Bus_Current
    #[inline(always)]
    pub fn set_d4_dc_bus_current(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 166 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
    /// D3_Phase_C_Current
    ///
    /// The measured value of Phase C current
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_phase_c_current(&self) -> f32 {
        self.d3_phase_c_current_raw()
    }
    
    /// Get raw value of D3_Phase_C_Current
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d3_phase_c_current_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D3_Phase_C_Current
    #[inline(always)]
    pub fn set_d3_phase_c_current(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 166 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// D2_Phase_B_Current
    ///
    /// The measured value of Phase B current
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_phase_b_current(&self) -> f32 {
        self.d2_phase_b_current_raw()
    }
    
    /// Get raw value of D2_Phase_B_Current
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d2_phase_b_current_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D2_Phase_B_Current
    #[inline(always)]
    pub fn set_d2_phase_b_current(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 166 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// D1_Phase_A_Current
    ///
    /// The measured value of Phase A current
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_phase_a_current(&self) -> f32 {
        self.d1_phase_a_current_raw()
    }
    
    /// Get raw value of D1_Phase_A_Current
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d1_phase_a_current_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D1_Phase_A_Current
    #[inline(always)]
    pub fn set_d1_phase_a_current(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 166 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M166CurrentInfo {
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
impl core::fmt::Debug for M166CurrentInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M166CurrentInfo")
                .field("d4_dc_bus_current", &self.d4_dc_bus_current())
                .field("d3_phase_c_current", &self.d3_phase_c_current())
                .field("d2_phase_b_current", &self.d2_phase_b_current())
                .field("d1_phase_a_current", &self.d1_phase_a_current())
            .finish()
        } else {
            f.debug_tuple("M166CurrentInfo").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M166CurrentInfo {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d4_dc_bus_current = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d3_phase_c_current = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d2_phase_b_current = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d1_phase_a_current = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        M166CurrentInfo::new(d4_dc_bus_current,d3_phase_c_current,d2_phase_b_current,d1_phase_a_current).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M165_Motor_Position_Info
///
/// - ID: 165 (0xa5)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M165MotorPositionInfo {
    raw: [u8; 8],
}

impl M165MotorPositionInfo {
    pub const MESSAGE_ID: u32 = 165;
    
    pub const D4_DELTA_RESOLVER_FILTERED_MIN: f32 = -3276.8_f32;
    pub const D4_DELTA_RESOLVER_FILTERED_MAX: f32 = 3276.7_f32;
    pub const D3_ELECTRICAL_OUTPUT_FREQUENCY_MIN: f32 = -3276.8_f32;
    pub const D3_ELECTRICAL_OUTPUT_FREQUENCY_MAX: f32 = 3276.7_f32;
    pub const D2_MOTOR_SPEED_MIN: i16 = -32768_i16;
    pub const D2_MOTOR_SPEED_MAX: i16 = 32767_i16;
    pub const D1_MOTOR_ANGLE_ELECTRICAL_MIN: f32 = 0_f32;
    pub const D1_MOTOR_ANGLE_ELECTRICAL_MAX: f32 = 6553.5_f32;
    
    /// Construct new M165_Motor_Position_Info from values
    pub fn new(d4_delta_resolver_filtered: f32, d3_electrical_output_frequency: f32, d2_motor_speed: i16, d1_motor_angle_electrical: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d4_delta_resolver_filtered(d4_delta_resolver_filtered)?;
        res.set_d3_electrical_output_frequency(d3_electrical_output_frequency)?;
        res.set_d2_motor_speed(d2_motor_speed)?;
        res.set_d1_motor_angle_electrical(d1_motor_angle_electrical)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D4_Delta_Resolver_Filtered
    ///
    /// Used in calibration of resolver angle adjustment.
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "deg"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_delta_resolver_filtered(&self) -> f32 {
        self.d4_delta_resolver_filtered_raw()
    }
    
    /// Get raw value of D4_Delta_Resolver_Filtered
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d4_delta_resolver_filtered_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D4_Delta_Resolver_Filtered
    #[inline(always)]
    pub fn set_d4_delta_resolver_filtered(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 165 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
    /// D3_Electrical_Output_Frequency
    ///
    /// The actual electrical frequency of the inverter
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "hz"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_electrical_output_frequency(&self) -> f32 {
        self.d3_electrical_output_frequency_raw()
    }
    
    /// Get raw value of D3_Electrical_Output_Frequency
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d3_electrical_output_frequency_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D3_Electrical_Output_Frequency
    #[inline(always)]
    pub fn set_d3_electrical_output_frequency(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 165 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// D2_Motor_Speed
    ///
    /// The measured speed of the motor
    ///
    /// - Min: -32768
    /// - Max: 32767
    /// - Unit: "rpm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_motor_speed(&self) -> i16 {
        self.d2_motor_speed_raw()
    }
    
    /// Get raw value of D2_Motor_Speed
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d2_motor_speed_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of D2_Motor_Speed
    #[inline(always)]
    pub fn set_d2_motor_speed(&mut self, value: i16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -32768_i16 || 32767_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 165 });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// D1_Motor_Angle_Electrical
    ///
    /// The Electrical Angle of the motor as read by the encoder or resolver
    ///
    /// - Min: 0
    /// - Max: 6553.5
    /// - Unit: "deg"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_motor_angle_electrical(&self) -> f32 {
        self.d1_motor_angle_electrical_raw()
    }
    
    /// Get raw value of D1_Motor_Angle_Electrical
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d1_motor_angle_electrical_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D1_Motor_Angle_Electrical
    #[inline(always)]
    pub fn set_d1_motor_angle_electrical(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 6553.5_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 165 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M165MotorPositionInfo {
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
impl core::fmt::Debug for M165MotorPositionInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M165MotorPositionInfo")
                .field("d4_delta_resolver_filtered", &self.d4_delta_resolver_filtered())
                .field("d3_electrical_output_frequency", &self.d3_electrical_output_frequency())
                .field("d2_motor_speed", &self.d2_motor_speed())
                .field("d1_motor_angle_electrical", &self.d1_motor_angle_electrical())
            .finish()
        } else {
            f.debug_tuple("M165MotorPositionInfo").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M165MotorPositionInfo {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d4_delta_resolver_filtered = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d3_electrical_output_frequency = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d2_motor_speed = u.int_in_range(-32768..=32767)?;
        let d1_motor_angle_electrical = u.float_in_range(0_f32..=6553.5_f32)?;
        M165MotorPositionInfo::new(d4_delta_resolver_filtered,d3_electrical_output_frequency,d2_motor_speed,d1_motor_angle_electrical).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M164_Digital_Input_Status
///
/// - ID: 164 (0xa4)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M164DigitalInputStatus {
    raw: [u8; 8],
}

impl M164DigitalInputStatus {
    pub const MESSAGE_ID: u32 = 164;
    
    
    /// Construct new M164_Digital_Input_Status from values
    pub fn new(d5_digital_input_5: bool, d4_digital_input_4: bool, d3_digital_input_3: bool, d2_digital_input_2: bool, d1_digital_input_1: bool, d6_digital_input_6: bool, d7_digital_input_7: bool, d8_digital_input_8: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d5_digital_input_5(d5_digital_input_5)?;
        res.set_d4_digital_input_4(d4_digital_input_4)?;
        res.set_d3_digital_input_3(d3_digital_input_3)?;
        res.set_d2_digital_input_2(d2_digital_input_2)?;
        res.set_d1_digital_input_1(d1_digital_input_1)?;
        res.set_d6_digital_input_6(d6_digital_input_6)?;
        res.set_d7_digital_input_7(d7_digital_input_7)?;
        res.set_d8_digital_input_8(d8_digital_input_8)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D5_Digital_Input_5
    ///
    /// Status of Digital Input #5
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "boolean"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d5_digital_input_5(&self) -> bool {
        self.d5_digital_input_5_raw()
    }
    
    /// Get raw value of D5_Digital_Input_5
    ///
    /// - Start bit: 32
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d5_digital_input_5_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[32..33].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D5_Digital_Input_5
    #[inline(always)]
    pub fn set_d5_digital_input_5(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[32..33].store_le(value);
        Ok(())
    }
    
    /// D4_Digital_Input_4
    ///
    /// Status of Digital Input #4
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "boolean"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_digital_input_4(&self) -> bool {
        self.d4_digital_input_4_raw()
    }
    
    /// Get raw value of D4_Digital_Input_4
    ///
    /// - Start bit: 24
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d4_digital_input_4_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[24..25].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D4_Digital_Input_4
    #[inline(always)]
    pub fn set_d4_digital_input_4(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[24..25].store_le(value);
        Ok(())
    }
    
    /// D3_Digital_Input_3
    ///
    /// Status of Digital Input #3
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "boolean"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_digital_input_3(&self) -> bool {
        self.d3_digital_input_3_raw()
    }
    
    /// Get raw value of D3_Digital_Input_3
    ///
    /// - Start bit: 16
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d3_digital_input_3_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[16..17].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D3_Digital_Input_3
    #[inline(always)]
    pub fn set_d3_digital_input_3(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[16..17].store_le(value);
        Ok(())
    }
    
    /// D2_Digital_Input_2
    ///
    /// Status of Digital Input #2
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "boolean"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_digital_input_2(&self) -> bool {
        self.d2_digital_input_2_raw()
    }
    
    /// Get raw value of D2_Digital_Input_2
    ///
    /// - Start bit: 8
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d2_digital_input_2_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[8..9].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D2_Digital_Input_2
    #[inline(always)]
    pub fn set_d2_digital_input_2(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[8..9].store_le(value);
        Ok(())
    }
    
    /// D1_Digital_Input_1
    ///
    /// Status of Digital Input #1
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "boolean"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_digital_input_1(&self) -> bool {
        self.d1_digital_input_1_raw()
    }
    
    /// Get raw value of D1_Digital_Input_1
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d1_digital_input_1_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D1_Digital_Input_1
    #[inline(always)]
    pub fn set_d1_digital_input_1(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
    /// D6_Digital_Input_6
    ///
    /// Status of Digital Input #6
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "boolean"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d6_digital_input_6(&self) -> bool {
        self.d6_digital_input_6_raw()
    }
    
    /// Get raw value of D6_Digital_Input_6
    ///
    /// - Start bit: 40
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d6_digital_input_6_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[40..41].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D6_Digital_Input_6
    #[inline(always)]
    pub fn set_d6_digital_input_6(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[40..41].store_le(value);
        Ok(())
    }
    
    /// D7_Digital_Input_7
    ///
    /// Status of Digital Input #7
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "boolean"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d7_digital_input_7(&self) -> bool {
        self.d7_digital_input_7_raw()
    }
    
    /// Get raw value of D7_Digital_Input_7
    ///
    /// - Start bit: 48
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d7_digital_input_7_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[48..49].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D7_Digital_Input_7
    #[inline(always)]
    pub fn set_d7_digital_input_7(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[48..49].store_le(value);
        Ok(())
    }
    
    /// D8_Digital_Input_8
    ///
    /// Status of Digital Input #8
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "boolean"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d8_digital_input_8(&self) -> bool {
        self.d8_digital_input_8_raw()
    }
    
    /// Get raw value of D8_Digital_Input_8
    ///
    /// - Start bit: 56
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d8_digital_input_8_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[56..57].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D8_Digital_Input_8
    #[inline(always)]
    pub fn set_d8_digital_input_8(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[56..57].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M164DigitalInputStatus {
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
impl core::fmt::Debug for M164DigitalInputStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M164DigitalInputStatus")
                .field("d5_digital_input_5", &self.d5_digital_input_5())
                .field("d4_digital_input_4", &self.d4_digital_input_4())
                .field("d3_digital_input_3", &self.d3_digital_input_3())
                .field("d2_digital_input_2", &self.d2_digital_input_2())
                .field("d1_digital_input_1", &self.d1_digital_input_1())
                .field("d6_digital_input_6", &self.d6_digital_input_6())
                .field("d7_digital_input_7", &self.d7_digital_input_7())
                .field("d8_digital_input_8", &self.d8_digital_input_8())
            .finish()
        } else {
            f.debug_tuple("M164DigitalInputStatus").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M164DigitalInputStatus {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d5_digital_input_5 = u.int_in_range(0..=1)? == 1;
        let d4_digital_input_4 = u.int_in_range(0..=1)? == 1;
        let d3_digital_input_3 = u.int_in_range(0..=1)? == 1;
        let d2_digital_input_2 = u.int_in_range(0..=1)? == 1;
        let d1_digital_input_1 = u.int_in_range(0..=1)? == 1;
        let d6_digital_input_6 = u.int_in_range(0..=1)? == 1;
        let d7_digital_input_7 = u.int_in_range(0..=1)? == 1;
        let d8_digital_input_8 = u.int_in_range(0..=1)? == 1;
        M164DigitalInputStatus::new(d5_digital_input_5,d4_digital_input_4,d3_digital_input_3,d2_digital_input_2,d1_digital_input_1,d6_digital_input_6,d7_digital_input_7,d8_digital_input_8).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M163_Analog_Input_Voltages
///
/// - ID: 163 (0xa3)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M163AnalogInputVoltages {
    raw: [u8; 8],
}

impl M163AnalogInputVoltages {
    pub const MESSAGE_ID: u32 = 163;
    
    pub const D4_ANALOG_INPUT_4_MIN: f32 = 0_f32;
    pub const D4_ANALOG_INPUT_4_MAX: f32 = 10.23_f32;
    pub const D3_ANALOG_INPUT_3_MIN: f32 = 0_f32;
    pub const D3_ANALOG_INPUT_3_MAX: f32 = 10.23_f32;
    pub const D2_ANALOG_INPUT_2_MIN: f32 = 0_f32;
    pub const D2_ANALOG_INPUT_2_MAX: f32 = 10.23_f32;
    pub const D1_ANALOG_INPUT_1_MIN: f32 = 0_f32;
    pub const D1_ANALOG_INPUT_1_MAX: f32 = 10.23_f32;
    pub const D5_ANALOG_INPUT_5_MIN: f32 = 0_f32;
    pub const D5_ANALOG_INPUT_5_MAX: f32 = 10.23_f32;
    pub const D6_ANALOG_INPUT_6_MIN: f32 = 0_f32;
    pub const D6_ANALOG_INPUT_6_MAX: f32 = 10.23_f32;
    
    /// Construct new M163_Analog_Input_Voltages from values
    pub fn new(d4_analog_input_4: f32, d3_analog_input_3: f32, d2_analog_input_2: f32, d1_analog_input_1: f32, d5_analog_input_5: f32, d6_analog_input_6: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d4_analog_input_4(d4_analog_input_4)?;
        res.set_d3_analog_input_3(d3_analog_input_3)?;
        res.set_d2_analog_input_2(d2_analog_input_2)?;
        res.set_d1_analog_input_1(d1_analog_input_1)?;
        res.set_d5_analog_input_5(d5_analog_input_5)?;
        res.set_d6_analog_input_6(d6_analog_input_6)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D4_Analog_Input_4
    ///
    /// Voltage on Analog Input #4
    ///
    /// - Min: 0
    /// - Max: 10.23
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_analog_input_4(&self) -> f32 {
        self.d4_analog_input_4_raw()
    }
    
    /// Get raw value of D4_Analog_Input_4
    ///
    /// - Start bit: 32
    /// - Signal size: 10 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d4_analog_input_4_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..42].load_le::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D4_Analog_Input_4
    #[inline(always)]
    pub fn set_d4_analog_input_4(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 10.23_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 163 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..42].store_le(value);
        Ok(())
    }
    
    /// D3_Analog_Input_3
    ///
    /// Voltage on Analog Input #3
    ///
    /// - Min: 0
    /// - Max: 10.23
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_analog_input_3(&self) -> f32 {
        self.d3_analog_input_3_raw()
    }
    
    /// Get raw value of D3_Analog_Input_3
    ///
    /// - Start bit: 20
    /// - Signal size: 10 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d3_analog_input_3_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[20..30].load_le::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D3_Analog_Input_3
    #[inline(always)]
    pub fn set_d3_analog_input_3(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 10.23_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 163 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[20..30].store_le(value);
        Ok(())
    }
    
    /// D2_Analog_Input_2
    ///
    /// Voltage on Analog Input #2
    ///
    /// - Min: 0
    /// - Max: 10.23
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_analog_input_2(&self) -> f32 {
        self.d2_analog_input_2_raw()
    }
    
    /// Get raw value of D2_Analog_Input_2
    ///
    /// - Start bit: 10
    /// - Signal size: 10 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d2_analog_input_2_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[10..20].load_le::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D2_Analog_Input_2
    #[inline(always)]
    pub fn set_d2_analog_input_2(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 10.23_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 163 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[10..20].store_le(value);
        Ok(())
    }
    
    /// D1_Analog_Input_1
    ///
    /// Voltage on Analog Input #1
    ///
    /// - Min: 0
    /// - Max: 10.23
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_analog_input_1(&self) -> f32 {
        self.d1_analog_input_1_raw()
    }
    
    /// Get raw value of D1_Analog_Input_1
    ///
    /// - Start bit: 0
    /// - Signal size: 10 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d1_analog_input_1_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..10].load_le::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D1_Analog_Input_1
    #[inline(always)]
    pub fn set_d1_analog_input_1(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 10.23_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 163 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..10].store_le(value);
        Ok(())
    }
    
    /// D5_Analog_Input_5
    ///
    /// Voltage on Analog Input #5
    ///
    /// - Min: 0
    /// - Max: 10.23
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d5_analog_input_5(&self) -> f32 {
        self.d5_analog_input_5_raw()
    }
    
    /// Get raw value of D5_Analog_Input_5
    ///
    /// - Start bit: 42
    /// - Signal size: 10 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d5_analog_input_5_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[42..52].load_le::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D5_Analog_Input_5
    #[inline(always)]
    pub fn set_d5_analog_input_5(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 10.23_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 163 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[42..52].store_le(value);
        Ok(())
    }
    
    /// D6_Analog_Input_6
    ///
    /// Voltage on Analog Input #6
    ///
    /// - Min: 0
    /// - Max: 10.23
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d6_analog_input_6(&self) -> f32 {
        self.d6_analog_input_6_raw()
    }
    
    /// Get raw value of D6_Analog_Input_6
    ///
    /// - Start bit: 52
    /// - Signal size: 10 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d6_analog_input_6_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[52..62].load_le::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D6_Analog_Input_6
    #[inline(always)]
    pub fn set_d6_analog_input_6(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 10.23_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 163 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[52..62].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M163AnalogInputVoltages {
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
impl core::fmt::Debug for M163AnalogInputVoltages {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M163AnalogInputVoltages")
                .field("d4_analog_input_4", &self.d4_analog_input_4())
                .field("d3_analog_input_3", &self.d3_analog_input_3())
                .field("d2_analog_input_2", &self.d2_analog_input_2())
                .field("d1_analog_input_1", &self.d1_analog_input_1())
                .field("d5_analog_input_5", &self.d5_analog_input_5())
                .field("d6_analog_input_6", &self.d6_analog_input_6())
            .finish()
        } else {
            f.debug_tuple("M163AnalogInputVoltages").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M163AnalogInputVoltages {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d4_analog_input_4 = u.float_in_range(0_f32..=10.23_f32)?;
        let d3_analog_input_3 = u.float_in_range(0_f32..=10.23_f32)?;
        let d2_analog_input_2 = u.float_in_range(0_f32..=10.23_f32)?;
        let d1_analog_input_1 = u.float_in_range(0_f32..=10.23_f32)?;
        let d5_analog_input_5 = u.float_in_range(0_f32..=10.23_f32)?;
        let d6_analog_input_6 = u.float_in_range(0_f32..=10.23_f32)?;
        M163AnalogInputVoltages::new(d4_analog_input_4,d3_analog_input_3,d2_analog_input_2,d1_analog_input_1,d5_analog_input_5,d6_analog_input_6).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M162_Temperature_Set_3
///
/// - ID: 162 (0xa2)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M162TemperatureSet3 {
    raw: [u8; 8],
}

impl M162TemperatureSet3 {
    pub const MESSAGE_ID: u32 = 162;
    
    pub const D4_TORQUE_SHUDDER_MIN: f32 = -3276.8_f32;
    pub const D4_TORQUE_SHUDDER_MAX: f32 = 3276.7_f32;
    pub const D3_MOTOR_TEMPERATURE_MIN: f32 = -3276.8_f32;
    pub const D3_MOTOR_TEMPERATURE_MAX: f32 = 3276.7_f32;
    pub const D2_RTD5_TEMPERATURE_MIN: f32 = -3276.8_f32;
    pub const D2_RTD5_TEMPERATURE_MAX: f32 = 3276.7_f32;
    pub const D1_RTD4_TEMPERATURE_MIN: f32 = -3276.8_f32;
    pub const D1_RTD4_TEMPERATURE_MAX: f32 = 3276.7_f32;
    
    /// Construct new M162_Temperature_Set_3 from values
    pub fn new(d4_torque_shudder: f32, d3_motor_temperature: f32, d2_rtd5_temperature: f32, d1_rtd4_temperature: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d4_torque_shudder(d4_torque_shudder)?;
        res.set_d3_motor_temperature(d3_motor_temperature)?;
        res.set_d2_rtd5_temperature(d2_rtd5_temperature)?;
        res.set_d1_rtd4_temperature(d1_rtd4_temperature)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D4_Torque_Shudder
    ///
    /// Shudder compensation value of torque
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "Nm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_torque_shudder(&self) -> f32 {
        self.d4_torque_shudder_raw()
    }
    
    /// Get raw value of D4_Torque_Shudder
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d4_torque_shudder_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D4_Torque_Shudder
    #[inline(always)]
    pub fn set_d4_torque_shudder(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 162 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
    /// D3_Motor_Temperature
    ///
    /// Motor Temperature Sensor
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "degC"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_motor_temperature(&self) -> f32 {
        self.d3_motor_temperature_raw()
    }
    
    /// Get raw value of D3_Motor_Temperature
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d3_motor_temperature_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D3_Motor_Temperature
    #[inline(always)]
    pub fn set_d3_motor_temperature(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 162 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// D2_RTD5_Temperature
    ///
    /// RTD 5 (PT1000) Temperature (Gen 2 only)
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "degC"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_rtd5_temperature(&self) -> f32 {
        self.d2_rtd5_temperature_raw()
    }
    
    /// Get raw value of D2_RTD5_Temperature
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d2_rtd5_temperature_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D2_RTD5_Temperature
    #[inline(always)]
    pub fn set_d2_rtd5_temperature(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 162 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// D1_RTD4_Temperature
    ///
    /// RTD 4 (PT1000) Temperature (Gen 2 only)
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "degC"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_rtd4_temperature(&self) -> f32 {
        self.d1_rtd4_temperature_raw()
    }
    
    /// Get raw value of D1_RTD4_Temperature
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d1_rtd4_temperature_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D1_RTD4_Temperature
    #[inline(always)]
    pub fn set_d1_rtd4_temperature(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 162 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M162TemperatureSet3 {
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
impl core::fmt::Debug for M162TemperatureSet3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M162TemperatureSet3")
                .field("d4_torque_shudder", &self.d4_torque_shudder())
                .field("d3_motor_temperature", &self.d3_motor_temperature())
                .field("d2_rtd5_temperature", &self.d2_rtd5_temperature())
                .field("d1_rtd4_temperature", &self.d1_rtd4_temperature())
            .finish()
        } else {
            f.debug_tuple("M162TemperatureSet3").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M162TemperatureSet3 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d4_torque_shudder = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d3_motor_temperature = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d2_rtd5_temperature = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d1_rtd4_temperature = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        M162TemperatureSet3::new(d4_torque_shudder,d3_motor_temperature,d2_rtd5_temperature,d1_rtd4_temperature).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M161_Temperature_Set_2
///
/// - ID: 161 (0xa1)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M161TemperatureSet2 {
    raw: [u8; 8],
}

impl M161TemperatureSet2 {
    pub const MESSAGE_ID: u32 = 161;
    
    pub const D4_RTD3_TEMPERATURE_MIN: f32 = -3276.8_f32;
    pub const D4_RTD3_TEMPERATURE_MAX: f32 = 3276.7_f32;
    pub const D3_RTD2_TEMPERATURE_MIN: f32 = -3276.8_f32;
    pub const D3_RTD2_TEMPERATURE_MAX: f32 = 3276.7_f32;
    pub const D2_RTD1_TEMPERATURE_MIN: f32 = -3276.8_f32;
    pub const D2_RTD1_TEMPERATURE_MAX: f32 = 3276.7_f32;
    pub const D1_CONTROL_BOARD_TEMPERATURE_MIN: f32 = -3276.8_f32;
    pub const D1_CONTROL_BOARD_TEMPERATURE_MAX: f32 = 3276.7_f32;
    
    /// Construct new M161_Temperature_Set_2 from values
    pub fn new(d4_rtd3_temperature: f32, d3_rtd2_temperature: f32, d2_rtd1_temperature: f32, d1_control_board_temperature: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d4_rtd3_temperature(d4_rtd3_temperature)?;
        res.set_d3_rtd2_temperature(d3_rtd2_temperature)?;
        res.set_d2_rtd1_temperature(d2_rtd1_temperature)?;
        res.set_d1_control_board_temperature(d1_control_board_temperature)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D4_RTD3_Temperature
    ///
    /// RTD input 3 (PT1000) Temperature (Gen 2 only)
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "degC"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_rtd3_temperature(&self) -> f32 {
        self.d4_rtd3_temperature_raw()
    }
    
    /// Get raw value of D4_RTD3_Temperature
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d4_rtd3_temperature_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D4_RTD3_Temperature
    #[inline(always)]
    pub fn set_d4_rtd3_temperature(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 161 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
    /// D3_RTD2_Temperature
    ///
    /// RTD input 2 (PT1000) Temperature
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "degC"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_rtd2_temperature(&self) -> f32 {
        self.d3_rtd2_temperature_raw()
    }
    
    /// Get raw value of D3_RTD2_Temperature
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d3_rtd2_temperature_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D3_RTD2_Temperature
    #[inline(always)]
    pub fn set_d3_rtd2_temperature(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 161 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// D2_RTD1_Temperature
    ///
    /// RTD input 1 (PT1000) Temperature
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "degC"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_rtd1_temperature(&self) -> f32 {
        self.d2_rtd1_temperature_raw()
    }
    
    /// Get raw value of D2_RTD1_Temperature
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d2_rtd1_temperature_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D2_RTD1_Temperature
    #[inline(always)]
    pub fn set_d2_rtd1_temperature(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 161 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// D1_Control_Board_Temperature
    ///
    /// Control Board Temperature
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "degC"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_control_board_temperature(&self) -> f32 {
        self.d1_control_board_temperature_raw()
    }
    
    /// Get raw value of D1_Control_Board_Temperature
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d1_control_board_temperature_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D1_Control_Board_Temperature
    #[inline(always)]
    pub fn set_d1_control_board_temperature(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 161 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M161TemperatureSet2 {
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
impl core::fmt::Debug for M161TemperatureSet2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M161TemperatureSet2")
                .field("d4_rtd3_temperature", &self.d4_rtd3_temperature())
                .field("d3_rtd2_temperature", &self.d3_rtd2_temperature())
                .field("d2_rtd1_temperature", &self.d2_rtd1_temperature())
                .field("d1_control_board_temperature", &self.d1_control_board_temperature())
            .finish()
        } else {
            f.debug_tuple("M161TemperatureSet2").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M161TemperatureSet2 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d4_rtd3_temperature = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d3_rtd2_temperature = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d2_rtd1_temperature = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d1_control_board_temperature = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        M161TemperatureSet2::new(d4_rtd3_temperature,d3_rtd2_temperature,d2_rtd1_temperature,d1_control_board_temperature).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M160_Temperature_Set_1
///
/// - ID: 160 (0xa0)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M160TemperatureSet1 {
    raw: [u8; 8],
}

impl M160TemperatureSet1 {
    pub const MESSAGE_ID: u32 = 160;
    
    pub const D4_GATE_DRIVER_BOARD_MIN: f32 = -3276.8_f32;
    pub const D4_GATE_DRIVER_BOARD_MAX: f32 = 3276.7_f32;
    pub const D3_MODULE_C_MIN: f32 = -3276.8_f32;
    pub const D3_MODULE_C_MAX: f32 = 3276.7_f32;
    pub const D2_MODULE_B_MIN: f32 = -3276.8_f32;
    pub const D2_MODULE_B_MAX: f32 = 3276.7_f32;
    pub const D1_MODULE_A_MIN: f32 = -3276.8_f32;
    pub const D1_MODULE_A_MAX: f32 = 3276.7_f32;
    
    /// Construct new M160_Temperature_Set_1 from values
    pub fn new(d4_gate_driver_board: f32, d3_module_c: f32, d2_module_b: f32, d1_module_a: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d4_gate_driver_board(d4_gate_driver_board)?;
        res.set_d3_module_c(d3_module_c)?;
        res.set_d2_module_b(d2_module_b)?;
        res.set_d1_module_a(d1_module_a)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D4_Gate_Driver_Board
    ///
    /// Gate Driver Board Temperature
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "degC"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_gate_driver_board(&self) -> f32 {
        self.d4_gate_driver_board_raw()
    }
    
    /// Get raw value of D4_Gate_Driver_Board
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d4_gate_driver_board_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D4_Gate_Driver_Board
    #[inline(always)]
    pub fn set_d4_gate_driver_board(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 160 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
    /// D3_Module_C
    ///
    /// IGBT Module C Temperature
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "degC"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_module_c(&self) -> f32 {
        self.d3_module_c_raw()
    }
    
    /// Get raw value of D3_Module_C
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d3_module_c_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D3_Module_C
    #[inline(always)]
    pub fn set_d3_module_c(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 160 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// D2_Module_B
    ///
    /// IGBT Module B Temperature
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "degC"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_module_b(&self) -> f32 {
        self.d2_module_b_raw()
    }
    
    /// Get raw value of D2_Module_B
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d2_module_b_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D2_Module_B
    #[inline(always)]
    pub fn set_d2_module_b(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 160 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// D1_Module_A
    ///
    /// IGBT Module A Temperature
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "degC"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_module_a(&self) -> f32 {
        self.d1_module_a_raw()
    }
    
    /// Get raw value of D1_Module_A
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d1_module_a_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of D1_Module_A
    #[inline(always)]
    pub fn set_d1_module_a(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 160 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M160TemperatureSet1 {
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
impl core::fmt::Debug for M160TemperatureSet1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M160TemperatureSet1")
                .field("d4_gate_driver_board", &self.d4_gate_driver_board())
                .field("d3_module_c", &self.d3_module_c())
                .field("d2_module_b", &self.d2_module_b())
                .field("d1_module_a", &self.d1_module_a())
            .finish()
        } else {
            f.debug_tuple("M160TemperatureSet1").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M160TemperatureSet1 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d4_gate_driver_board = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d3_module_c = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d2_module_b = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let d1_module_a = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        M160TemperatureSet1::new(d4_gate_driver_board,d3_module_c,d2_module_b,d1_module_a).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M174_Firmware_Info
///
/// - ID: 174 (0xae)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M174FirmwareInfo {
    raw: [u8; 8],
}

impl M174FirmwareInfo {
    pub const MESSAGE_ID: u32 = 174;
    
    pub const D1_PROJECT_CODE_EEP_VER_MIN: u16 = 0_u16;
    pub const D1_PROJECT_CODE_EEP_VER_MAX: u16 = 65535_u16;
    pub const D2_SW_VERSION_MIN: u16 = 0_u16;
    pub const D2_SW_VERSION_MAX: u16 = 65535_u16;
    pub const D3_DATE_CODE_MMDD_MIN: u16 = 0_u16;
    pub const D3_DATE_CODE_MMDD_MAX: u16 = 65535_u16;
    pub const D4_DATE_CODE_YYYY_MIN: u16 = 0_u16;
    pub const D4_DATE_CODE_YYYY_MAX: u16 = 65535_u16;
    
    /// Construct new M174_Firmware_Info from values
    pub fn new(d1_project_code_eep_ver: u16, d2_sw_version: u16, d3_date_code_mmdd: u16, d4_date_code_yyyy: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d1_project_code_eep_ver(d1_project_code_eep_ver)?;
        res.set_d2_sw_version(d2_sw_version)?;
        res.set_d3_date_code_mmdd(d3_date_code_mmdd)?;
        res.set_d4_date_code_yyyy(d4_date_code_yyyy)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D1_Project_Code_EEP_Ver
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_project_code_eep_ver(&self) -> u16 {
        self.d1_project_code_eep_ver_raw()
    }
    
    /// Get raw value of D1_Project_Code_EEP_Ver
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d1_project_code_eep_ver_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        signal
    }
    
    /// Set value of D1_Project_Code_EEP_Ver
    #[inline(always)]
    pub fn set_d1_project_code_eep_ver(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 174 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// D2_SW_Version
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_sw_version(&self) -> u16 {
        self.d2_sw_version_raw()
    }
    
    /// Get raw value of D2_SW_Version
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d2_sw_version_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        signal
    }
    
    /// Set value of D2_SW_Version
    #[inline(always)]
    pub fn set_d2_sw_version(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 174 });
        }
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// D3_DateCode_MMDD
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_date_code_mmdd(&self) -> u16 {
        self.d3_date_code_mmdd_raw()
    }
    
    /// Get raw value of D3_DateCode_MMDD
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d3_date_code_mmdd_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        signal
    }
    
    /// Set value of D3_DateCode_MMDD
    #[inline(always)]
    pub fn set_d3_date_code_mmdd(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 174 });
        }
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// D4_DateCode_YYYY
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_date_code_yyyy(&self) -> u16 {
        self.d4_date_code_yyyy_raw()
    }
    
    /// Get raw value of D4_DateCode_YYYY
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d4_date_code_yyyy_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        signal
    }
    
    /// Set value of D4_DateCode_YYYY
    #[inline(always)]
    pub fn set_d4_date_code_yyyy(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 174 });
        }
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M174FirmwareInfo {
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
impl core::fmt::Debug for M174FirmwareInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M174FirmwareInfo")
                .field("d1_project_code_eep_ver", &self.d1_project_code_eep_ver())
                .field("d2_sw_version", &self.d2_sw_version())
                .field("d3_date_code_mmdd", &self.d3_date_code_mmdd())
                .field("d4_date_code_yyyy", &self.d4_date_code_yyyy())
            .finish()
        } else {
            f.debug_tuple("M174FirmwareInfo").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M174FirmwareInfo {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d1_project_code_eep_ver = u.int_in_range(0..=65535)?;
        let d2_sw_version = u.int_in_range(0..=65535)?;
        let d3_date_code_mmdd = u.int_in_range(0..=65535)?;
        let d4_date_code_yyyy = u.int_in_range(0..=65535)?;
        M174FirmwareInfo::new(d1_project_code_eep_ver,d2_sw_version,d3_date_code_mmdd,d4_date_code_yyyy).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M175_Diag_Data
///
/// - ID: 175 (0xaf)
/// - Size: 8 bytes
/// - Transmitter: INV
#[derive(Clone, Copy)]
pub struct M175DiagData {
    raw: [u8; 8],
}

impl M175DiagData {
    pub const MESSAGE_ID: u32 = 175;
    
    pub const D1_BUFFER_RECORD_MIN: u8 = 0_u8;
    pub const D1_BUFFER_RECORD_MAX: u8 = 255_u8;
    pub const D2_BUFFER_SEGMENT_MIN: u8 = 0_u8;
    pub const D2_BUFFER_SEGMENT_MAX: u8 = 5_u8;
    pub const D3_DIAG_DATA_1_MIN: i16 = -32768_i16;
    pub const D3_DIAG_DATA_1_MAX: i16 = 32767_i16;
    pub const D4_DIAG_DATA_2_MIN: i16 = -32768_i16;
    pub const D4_DIAG_DATA_2_MAX: i16 = 32767_i16;
    pub const D5_DIAG_DATA_3_MIN: i16 = -32768_i16;
    pub const D5_DIAG_DATA_3_MAX: i16 = 32767_i16;
    
    /// Construct new M175_Diag_Data from values
    pub fn new(d1_buffer_record: u8, d2_buffer_segment: u8, d3_diag_data_1: i16, d4_diag_data_2: i16, d5_diag_data_3: i16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d1_buffer_record(d1_buffer_record)?;
        res.set_d2_buffer_segment(d2_buffer_segment)?;
        res.set_d3_diag_data_1(d3_diag_data_1)?;
        res.set_d4_diag_data_2(d4_diag_data_2)?;
        res.set_d5_diag_data_3(d5_diag_data_3)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D1_Buffer_Record
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_buffer_record(&self) -> u8 {
        self.d1_buffer_record_raw()
    }
    
    /// Get raw value of D1_Buffer_Record
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d1_buffer_record_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        signal
    }
    
    /// Set value of D1_Buffer_Record
    #[inline(always)]
    pub fn set_d1_buffer_record(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 175 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// D2_Buffer_Segment
    ///
    /// - Min: 0
    /// - Max: 5
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_buffer_segment(&self) -> u8 {
        self.d2_buffer_segment_raw()
    }
    
    /// Get raw value of D2_Buffer_Segment
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d2_buffer_segment_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        signal
    }
    
    /// Set value of D2_Buffer_Segment
    #[inline(always)]
    pub fn set_d2_buffer_segment(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 5_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 175 });
        }
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// D3_Diag_Data_1
    ///
    /// - Min: -32768
    /// - Max: 32767
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_diag_data_1(&self) -> i16 {
        self.d3_diag_data_1_raw()
    }
    
    /// Get raw value of D3_Diag_Data_1
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d3_diag_data_1_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of D3_Diag_Data_1
    #[inline(always)]
    pub fn set_d3_diag_data_1(&mut self, value: i16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -32768_i16 || 32767_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 175 });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// D4_Diag_Data_2
    ///
    /// - Min: -32768
    /// - Max: 32767
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_diag_data_2(&self) -> i16 {
        self.d4_diag_data_2_raw()
    }
    
    /// Get raw value of D4_Diag_Data_2
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d4_diag_data_2_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of D4_Diag_Data_2
    #[inline(always)]
    pub fn set_d4_diag_data_2(&mut self, value: i16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -32768_i16 || 32767_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 175 });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// D5_Diag_Data_3
    ///
    /// - Min: -32768
    /// - Max: 32767
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d5_diag_data_3(&self) -> i16 {
        self.d5_diag_data_3_raw()
    }
    
    /// Get raw value of D5_Diag_Data_3
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn d5_diag_data_3_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of D5_Diag_Data_3
    #[inline(always)]
    pub fn set_d5_diag_data_3(&mut self, value: i16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -32768_i16 || 32767_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 175 });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M175DiagData {
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
impl core::fmt::Debug for M175DiagData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M175DiagData")
                .field("d1_buffer_record", &self.d1_buffer_record())
                .field("d2_buffer_segment", &self.d2_buffer_segment())
                .field("d3_diag_data_1", &self.d3_diag_data_1())
                .field("d4_diag_data_2", &self.d4_diag_data_2())
                .field("d5_diag_data_3", &self.d5_diag_data_3())
            .finish()
        } else {
            f.debug_tuple("M175DiagData").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M175DiagData {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d1_buffer_record = u.int_in_range(0..=255)?;
        let d2_buffer_segment = u.int_in_range(0..=5)?;
        let d3_diag_data_1 = u.int_in_range(-32768..=32767)?;
        let d4_diag_data_2 = u.int_in_range(-32768..=32767)?;
        let d5_diag_data_3 = u.int_in_range(-32768..=32767)?;
        M175DiagData::new(d1_buffer_record,d2_buffer_segment,d3_diag_data_1,d4_diag_data_2,d5_diag_data_3).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M187_U2C_Command_Txd
///
/// - ID: 471 (0x1d7)
/// - Size: 8 bytes
/// - Transmitter: INV
///
/// Enable message sent to Delphi DC/DC converter.
#[derive(Clone, Copy)]
pub struct M187U2cCommandTxd {
    raw: [u8; 8],
}

impl M187U2cCommandTxd {
    pub const MESSAGE_ID: u32 = 471;
    
    pub const D1_ID_BYTE_MIN: u8 = 0_u8;
    pub const D1_ID_BYTE_MAX: u8 = 255_u8;
    pub const D2_SETPOINT_CALC_MIN: u8 = 0_u8;
    pub const D2_SETPOINT_CALC_MAX: u8 = 255_u8;
    
    /// Construct new M187_U2C_Command_Txd from values
    pub fn new(d1_id_byte: u8, d2_setpoint_calc: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d1_id_byte(d1_id_byte)?;
        res.set_d2_setpoint_calc(d2_setpoint_calc)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D1_ID_Byte
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_id_byte(&self) -> u8 {
        self.d1_id_byte_raw()
    }
    
    /// Get raw value of D1_ID_Byte
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d1_id_byte_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        signal
    }
    
    /// Set value of D1_ID_Byte
    #[inline(always)]
    pub fn set_d1_id_byte(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 471 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// D2_Setpoint_Calc
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_setpoint_calc(&self) -> u8 {
        self.d2_setpoint_calc_raw()
    }
    
    /// Get raw value of D2_Setpoint_Calc
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d2_setpoint_calc_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        signal
    }
    
    /// Set value of D2_Setpoint_Calc
    #[inline(always)]
    pub fn set_d2_setpoint_calc(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 471 });
        }
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M187U2cCommandTxd {
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
impl core::fmt::Debug for M187U2cCommandTxd {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M187U2cCommandTxd")
                .field("d1_id_byte", &self.d1_id_byte())
                .field("d2_setpoint_calc", &self.d2_setpoint_calc())
            .finish()
        } else {
            f.debug_tuple("M187U2cCommandTxd").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M187U2cCommandTxd {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d1_id_byte = u.int_in_range(0..=255)?;
        let d2_setpoint_calc = u.int_in_range(0..=255)?;
        M187U2cCommandTxd::new(d1_id_byte,d2_setpoint_calc).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M188_U2C_Message_Rxd
///
/// - ID: 469 (0x1d5)
/// - Size: 8 bytes
/// - Transmitter: U2C
///
/// Response from Delphi DC/DC converter.
#[derive(Clone, Copy)]
pub struct M188U2cMessageRxd {
    raw: [u8; 8],
}

impl M188U2cMessageRxd {
    pub const MESSAGE_ID: u32 = 469;
    
    pub const D3_HV_INPUT_CURRENT_MIN: u8 = 0_u8;
    pub const D3_HV_INPUT_CURRENT_MAX: u8 = 255_u8;
    pub const D4_DTC_STATUS_MIN: u8 = 0_u8;
    pub const D4_DTC_STATUS_MAX: u8 = 7_u8;
    pub const D5_DTC_INDEX_MIN: u8 = 0_u8;
    pub const D5_DTC_INDEX_MAX: u8 = 31_u8;
    pub const D6_14V_MONITOR_MIN: u8 = 0_u8;
    pub const D6_14V_MONITOR_MAX: u8 = 255_u8;
    pub const D8_14V_CURRENT_MONITOR_MIN: u8 = 0_u8;
    pub const D8_14V_CURRENT_MONITOR_MAX: u8 = 127_u8;
    
    /// Construct new M188_U2C_Message_Rxd from values
    pub fn new(d1_hv_input_current_sensor_validity: bool, d2_14v_master_fault: bool, d3_hv_input_current: u8, d4_dtc_status: u8, d5_dtc_index: u8, d6_14v_monitor: u8, d7_14v_conditional: bool, d8_14v_current_monitor: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d1_hv_input_current_sensor_validity(d1_hv_input_current_sensor_validity)?;
        res.set_d2_14v_master_fault(d2_14v_master_fault)?;
        res.set_d3_hv_input_current(d3_hv_input_current)?;
        res.set_d4_dtc_status(d4_dtc_status)?;
        res.set_d5_dtc_index(d5_dtc_index)?;
        res.set_d6_14v_monitor(d6_14v_monitor)?;
        res.set_d7_14v_conditional(d7_14v_conditional)?;
        res.set_d8_14v_current_monitor(d8_14v_current_monitor)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D1_HV_Input_Current_Sensor_Validity
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_hv_input_current_sensor_validity(&self) -> bool {
        self.d1_hv_input_current_sensor_validity_raw()
    }
    
    /// Get raw value of D1_HV_Input_Current_Sensor_Validity
    ///
    /// - Start bit: 4
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d1_hv_input_current_sensor_validity_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[4..5].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D1_HV_Input_Current_Sensor_Validity
    #[inline(always)]
    pub fn set_d1_hv_input_current_sensor_validity(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[4..5].store_le(value);
        Ok(())
    }
    
    /// D2_14V_Master_Fault
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_14v_master_fault(&self) -> bool {
        self.d2_14v_master_fault_raw()
    }
    
    /// Get raw value of D2_14V_Master_Fault
    ///
    /// - Start bit: 7
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d2_14v_master_fault_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[7..8].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D2_14V_Master_Fault
    #[inline(always)]
    pub fn set_d2_14v_master_fault(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[7..8].store_le(value);
        Ok(())
    }
    
    /// D3_HV_Input_Current
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d3_hv_input_current(&self) -> u8 {
        self.d3_hv_input_current_raw()
    }
    
    /// Get raw value of D3_HV_Input_Current
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d3_hv_input_current_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        signal
    }
    
    /// Set value of D3_HV_Input_Current
    #[inline(always)]
    pub fn set_d3_hv_input_current(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 469 });
        }
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// D4_DTC_Status
    ///
    /// - Min: 0
    /// - Max: 7
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d4_dtc_status(&self) -> u8 {
        self.d4_dtc_status_raw()
    }
    
    /// Get raw value of D4_DTC_Status
    ///
    /// - Start bit: 16
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d4_dtc_status_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..19].load_le::<u8>();
        
        signal
    }
    
    /// Set value of D4_DTC_Status
    #[inline(always)]
    pub fn set_d4_dtc_status(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 7_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 469 });
        }
        self.raw.view_bits_mut::<Lsb0>()[16..19].store_le(value);
        Ok(())
    }
    
    /// D5_DTC_Index
    ///
    /// - Min: 0
    /// - Max: 31
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d5_dtc_index(&self) -> u8 {
        self.d5_dtc_index_raw()
    }
    
    /// Get raw value of D5_DTC_Index
    ///
    /// - Start bit: 19
    /// - Signal size: 5 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d5_dtc_index_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[19..24].load_le::<u8>();
        
        signal
    }
    
    /// Set value of D5_DTC_Index
    #[inline(always)]
    pub fn set_d5_dtc_index(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 31_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 469 });
        }
        self.raw.view_bits_mut::<Lsb0>()[19..24].store_le(value);
        Ok(())
    }
    
    /// D6_14V_Monitor
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d6_14v_monitor(&self) -> u8 {
        self.d6_14v_monitor_raw()
    }
    
    /// Get raw value of D6_14V_Monitor
    ///
    /// - Start bit: 24
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d6_14v_monitor_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
        
        signal
    }
    
    /// Set value of D6_14V_Monitor
    #[inline(always)]
    pub fn set_d6_14v_monitor(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 469 });
        }
        self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
        Ok(())
    }
    
    /// D7_14V_Conditional
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d7_14v_conditional(&self) -> bool {
        self.d7_14v_conditional_raw()
    }
    
    /// Get raw value of D7_14V_Conditional
    ///
    /// - Start bit: 39
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d7_14v_conditional_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[39..40].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of D7_14V_Conditional
    #[inline(always)]
    pub fn set_d7_14v_conditional(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[39..40].store_le(value);
        Ok(())
    }
    
    /// D8_14V_Current_Monitor
    ///
    /// - Min: 0
    /// - Max: 127
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d8_14v_current_monitor(&self) -> u8 {
        self.d8_14v_current_monitor_raw()
    }
    
    /// Get raw value of D8_14V_Current_Monitor
    ///
    /// - Start bit: 48
    /// - Signal size: 7 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d8_14v_current_monitor_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..55].load_le::<u8>();
        
        signal
    }
    
    /// Set value of D8_14V_Current_Monitor
    #[inline(always)]
    pub fn set_d8_14v_current_monitor(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 127_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 469 });
        }
        self.raw.view_bits_mut::<Lsb0>()[48..55].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M188U2cMessageRxd {
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
impl core::fmt::Debug for M188U2cMessageRxd {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M188U2cMessageRxd")
                .field("d1_hv_input_current_sensor_validity", &self.d1_hv_input_current_sensor_validity())
                .field("d2_14v_master_fault", &self.d2_14v_master_fault())
                .field("d3_hv_input_current", &self.d3_hv_input_current())
                .field("d4_dtc_status", &self.d4_dtc_status())
                .field("d5_dtc_index", &self.d5_dtc_index())
                .field("d6_14v_monitor", &self.d6_14v_monitor())
                .field("d7_14v_conditional", &self.d7_14v_conditional())
                .field("d8_14v_current_monitor", &self.d8_14v_current_monitor())
            .finish()
        } else {
            f.debug_tuple("M188U2cMessageRxd").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M188U2cMessageRxd {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d1_hv_input_current_sensor_validity = u.int_in_range(0..=1)? == 1;
        let d2_14v_master_fault = u.int_in_range(0..=1)? == 1;
        let d3_hv_input_current = u.int_in_range(0..=255)?;
        let d4_dtc_status = u.int_in_range(0..=7)?;
        let d5_dtc_index = u.int_in_range(0..=31)?;
        let d6_14v_monitor = u.int_in_range(0..=255)?;
        let d7_14v_conditional = u.int_in_range(0..=1)? == 1;
        let d8_14v_current_monitor = u.int_in_range(0..=127)?;
        M188U2cMessageRxd::new(d1_hv_input_current_sensor_validity,d2_14v_master_fault,d3_hv_input_current,d4_dtc_status,d5_dtc_index,d6_14v_monitor,d7_14v_conditional,d8_14v_current_monitor).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BMS_Current_Limit
///
/// - ID: 514 (0x202)
/// - Size: 8 bytes
/// - Transmitter: BMS
///
/// Message sent by BMS for inverter DC current limiting.
#[derive(Clone, Copy)]
pub struct BmsCurrentLimit {
    raw: [u8; 8],
}

impl BmsCurrentLimit {
    pub const MESSAGE_ID: u32 = 514;
    
    pub const D1_MAX_DISCHARGE_CURRENT_MIN: u16 = 0_u16;
    pub const D1_MAX_DISCHARGE_CURRENT_MAX: u16 = 1000_u16;
    pub const D2_MAX_CHARGE_CURRENT_MIN: u16 = 0_u16;
    pub const D2_MAX_CHARGE_CURRENT_MAX: u16 = 1000_u16;
    
    /// Construct new BMS_Current_Limit from values
    pub fn new(d1_max_discharge_current: u16, d2_max_charge_current: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_d1_max_discharge_current(d1_max_discharge_current)?;
        res.set_d2_max_charge_current(d2_max_charge_current)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// D1_Max_Discharge_Current
    ///
    /// Maximum discharge current from BMS
    ///
    /// - Min: 0
    /// - Max: 1000
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d1_max_discharge_current(&self) -> u16 {
        self.d1_max_discharge_current_raw()
    }
    
    /// Get raw value of D1_Max_Discharge_Current
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d1_max_discharge_current_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        signal
    }
    
    /// Set value of D1_Max_Discharge_Current
    #[inline(always)]
    pub fn set_d1_max_discharge_current(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 1000_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 514 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// D2_Max_Charge_Current
    ///
    /// Maximum charge current from BMS
    ///
    /// - Min: 0
    /// - Max: 1000
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn d2_max_charge_current(&self) -> u16 {
        self.d2_max_charge_current_raw()
    }
    
    /// Get raw value of D2_Max_Charge_Current
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d2_max_charge_current_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        signal
    }
    
    /// Set value of D2_Max_Charge_Current
    #[inline(always)]
    pub fn set_d2_max_charge_current(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 1000_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 514 });
        }
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsCurrentLimit {
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
impl core::fmt::Debug for BmsCurrentLimit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsCurrentLimit")
                .field("d1_max_discharge_current", &self.d1_max_discharge_current())
                .field("d2_max_charge_current", &self.d2_max_charge_current())
            .finish()
        } else {
            f.debug_tuple("BmsCurrentLimit").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsCurrentLimit {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let d1_max_discharge_current = u.int_in_range(0..=1000)?;
        let d2_max_charge_current = u.int_in_range(0..=1000)?;
        BmsCurrentLimit::new(d1_max_discharge_current,d2_max_charge_current).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// M176_Fast_Info
///
/// - ID: 176 (0xb0)
/// - Size: 8 bytes
/// - Transmitter: INV
///
/// To enable fast message set CAN ACTIVE MSGS HI WORD to 0xFFFE. Setting to default value of 0xFFFF will disable the fast message.
#[derive(Clone, Copy)]
pub struct M176FastInfo {
    raw: [u8; 8],
}

impl M176FastInfo {
    pub const MESSAGE_ID: u32 = 176;
    
    pub const FAST_TORQUE_COMMAND_MIN: f32 = -3276.8_f32;
    pub const FAST_TORQUE_COMMAND_MAX: f32 = 3276.7_f32;
    pub const FAST_TORQUE_FEEDBACK_MIN: f32 = -3276.8_f32;
    pub const FAST_TORQUE_FEEDBACK_MAX: f32 = 3276.7_f32;
    pub const FAST_MOTOR_SPEED_MIN: i16 = -32768_i16;
    pub const FAST_MOTOR_SPEED_MAX: i16 = 32767_i16;
    pub const FAST_DC_BUS_VOLTAGE_MIN: f32 = -3276.8_f32;
    pub const FAST_DC_BUS_VOLTAGE_MAX: f32 = 3276.7_f32;
    
    /// Construct new M176_Fast_Info from values
    pub fn new(fast_torque_command: f32, fast_torque_feedback: f32, fast_motor_speed: i16, fast_dc_bus_voltage: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_fast_torque_command(fast_torque_command)?;
        res.set_fast_torque_feedback(fast_torque_feedback)?;
        res.set_fast_motor_speed(fast_motor_speed)?;
        res.set_fast_dc_bus_voltage(fast_dc_bus_voltage)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// Fast_Torque_Command
    ///
    /// The commanded torque
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "Nm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fast_torque_command(&self) -> f32 {
        self.fast_torque_command_raw()
    }
    
    /// Get raw value of Fast_Torque_Command
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn fast_torque_command_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Fast_Torque_Command
    #[inline(always)]
    pub fn set_fast_torque_command(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 176 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// Fast_Torque_Feedback
    ///
    /// The estimated torque
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "Nm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fast_torque_feedback(&self) -> f32 {
        self.fast_torque_feedback_raw()
    }
    
    /// Get raw value of Fast_Torque_Feedback
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn fast_torque_feedback_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Fast_Torque_Feedback
    #[inline(always)]
    pub fn set_fast_torque_feedback(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 176 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// Fast_Motor_Speed
    ///
    /// Motor speed
    ///
    /// - Min: -32768
    /// - Max: 32767
    /// - Unit: "rpm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fast_motor_speed(&self) -> i16 {
        self.fast_motor_speed_raw()
    }
    
    /// Get raw value of Fast_Motor_Speed
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn fast_motor_speed_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of Fast_Motor_Speed
    #[inline(always)]
    pub fn set_fast_motor_speed(&mut self, value: i16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -32768_i16 || 32767_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 176 });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// Fast_DC_Bus_Voltage
    ///
    /// DC Bus Voltage
    ///
    /// - Min: -3276.8
    /// - Max: 3276.7
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fast_dc_bus_voltage(&self) -> f32 {
        self.fast_dc_bus_voltage_raw()
    }
    
    /// Get raw value of Fast_DC_Bus_Voltage
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn fast_dc_bus_voltage_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Fast_DC_Bus_Voltage
    #[inline(always)]
    pub fn set_fast_dc_bus_voltage(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -3276.8_f32 || 3276.7_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 176 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M176FastInfo {
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
impl core::fmt::Debug for M176FastInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("M176FastInfo")
                .field("fast_torque_command", &self.fast_torque_command())
                .field("fast_torque_feedback", &self.fast_torque_feedback())
                .field("fast_motor_speed", &self.fast_motor_speed())
                .field("fast_dc_bus_voltage", &self.fast_dc_bus_voltage())
            .finish()
        } else {
            f.debug_tuple("M176FastInfo").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for M176FastInfo {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let fast_torque_command = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let fast_torque_feedback = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        let fast_motor_speed = u.int_in_range(-32768..=32767)?;
        let fast_dc_bus_voltage = u.float_in_range(-3276.8_f32..=3276.7_f32)?;
        M176FastInfo::new(fast_torque_command,fast_torque_feedback,fast_motor_speed,fast_dc_bus_voltage).map_err(|_| arbitrary::Error::IncorrectFormat)
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

