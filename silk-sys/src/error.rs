use thiserror::Error;

#[derive(Error, Debug)]
pub enum SilkError {
    #[error("Invalid")]
    Invalid,
    #[error("EncInputInvalidNoOfSamples")]
    EncInputInvalidNoOfSamples,
    #[error("EncFsNotSupported")]
    EncFsNotSupported,
    #[error("EncPacketSizeNotSupported")]
    EncPacketSizeNotSupported,
    #[error("EncPayloadBufTooShort")]
    EncPayloadBufTooShort,
    #[error("EncInvalidLossRate")]
    EncInvalidLossRate,
    #[error("EncInvalidComplexitySetting")]
    EncInvalidComplexitySetting,
    #[error("EncInvalidInbandFecSetting")]
    EncInvalidInbandFecSetting,
    #[error("EncInvalidDtxSetting")]
    EncInvalidDtxSetting,
    #[error("EncInternalError")]
    EncInternalError,
    #[error("DecInvalidSamplingFrequency")]
    DecInvalidSamplingFrequency,
    #[error("DecPayloadTooLarge")]
    DecPayloadTooLarge,
    #[error("DecPayloadError")]
    DecPayloadError,
    #[error("OTHER {0}")]
    Other(i32),
}

impl From<i32> for SilkError {
    fn from(code: i32) -> Self {
        match code {
            -1 => Self::EncInputInvalidNoOfSamples,
            -2 => Self::EncFsNotSupported,
            -3 => Self::EncPacketSizeNotSupported,
            -4 => Self::EncPayloadBufTooShort,
            -5 => Self::EncInvalidLossRate,
            -6 => Self::EncInvalidComplexitySetting,
            -7 => Self::EncInvalidInbandFecSetting,
            -8 => Self::EncInvalidDtxSetting,
            -9 => Self::EncInternalError,
            -10 => Self::DecInvalidSamplingFrequency,
            -11 => Self::DecPayloadTooLarge,
            -12 => Self::DecPayloadError,
            _ => Self::Other(code),
        }
    }
}