use std::string::FromUtf8Error;

use thiserror::Error;

use crate::{
    message::MessageType,
    sctp::{AssociationError, PayloadType, StreamError},
};

#[derive(Error, Eq, PartialEq, Clone, Debug)]
pub enum ChannelTypeError {
    // Marshal buffer was too short
    UnexpectedEndOfBuffer { expected: usize, actual: usize },

    // Remote requested a channel type that we don't support
    InvalidChannelType { invalid_type: u8 },
}

impl std::fmt::Display for ChannelTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEndOfBuffer { expected, actual } => {
                writeln!(
                    f,
                    "Marshal buffer was too short: (expected: {:?}, actual: {:?})",
                    expected, actual
                )
            }
            Self::InvalidChannelType { invalid_type } => {
                writeln!(f, "Invalid channel type: {:?}", invalid_type)
            }
        }
    }
}

#[derive(Error, Eq, PartialEq, Clone, Debug)]
pub enum DataChannelAckError {}

#[derive(Error, Eq, PartialEq, Clone, Debug)]
pub enum DataChannelOpenError {
    // Marshal buffer was too short
    UnexpectedEndOfBuffer { expected: usize, actual: usize },

    // Declared length and actual length don't match
    ExpectedAndActualLengthMismatch { expected: usize, actual: usize },

    // DataChannel messages with a Payload Protocol Identifier we don't know how to handle
    InvalidPayloadProtocolIdentifier,

    // Remote requested a channel type that we don't support
    ChannelType(#[from] ChannelTypeError),
}

impl std::fmt::Display for DataChannelOpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEndOfBuffer { expected, actual } => {
                writeln!(
                    f,
                    "Marshal buffer was too short: (expected: {:?}, actual: {:?})",
                    expected, actual
                )
            }
            Self::InvalidPayloadProtocolIdentifier => writeln!(
                f,
                "DataChannel message payload protocol identifier is value we can't handle"
            ),
            Self::ExpectedAndActualLengthMismatch { expected, actual } => {
                writeln!(
                    f,
                    "Expected and actual length do not match: (expected: {:?}, actual: {:?})",
                    expected, actual
                )
            }
            Self::ChannelType(error) => error.fmt(f),
        }
    }
}

#[derive(Error, Eq, PartialEq, Clone, Debug)]
pub enum DataChannelError {
    InvalidMessageType { invalid_type: MessageType },
    InvalidPayloadProtocolIdentifier { invalid_identifier: PayloadType },
    Message(#[from] MessageError),
    Stream(#[from] StreamError),
    String(#[from] FromUtf8Error),
    Association(#[from] AssociationError),
}

impl std::fmt::Display for DataChannelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataChannelError::InvalidMessageType { invalid_type } => {
                writeln!(f, "Invalid message type: {:?}", invalid_type)
            }
            DataChannelError::InvalidPayloadProtocolIdentifier { invalid_identifier } => {
                writeln!(
                    f,
                    "Invalid payload protocol identifier: {:?}",
                    invalid_identifier
                )
            }
            DataChannelError::Message(error) => error.fmt(f),
            DataChannelError::Stream(error) => error.fmt(f),
            DataChannelError::String(error) => error.fmt(f),
            DataChannelError::Association(error) => error.fmt(f),
        }
    }
}

#[derive(Error, Eq, PartialEq, Clone, Debug)]
pub enum MessageTypeError {
    // Marshal buffer was too short
    UnexpectedEndOfBuffer { expected: usize, actual: usize },

    // DataChannel message has a type we don't support
    InvalidMessageType { invalid_type: u8 },
}

impl std::fmt::Display for MessageTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEndOfBuffer { expected, actual } => {
                writeln!(
                    f,
                    "Marshal buffer was too short: (expected: {:?}, actual: {:?})",
                    expected, actual
                )
            }
            Self::InvalidMessageType { invalid_type } => {
                writeln!(f, "Invalid message type: {:?}", invalid_type)
            }
        }
    }
}

#[derive(Error, Eq, PartialEq, Clone, Debug)]
pub enum MessageError {
    // Marshal buffer was too short
    UnexpectedEndOfBuffer { expected: usize, actual: usize },

    // Declared length and actual length don't match
    ExpectedAndActualLengthMismatch { expected: usize, actual: usize },

    // DataChannel messages with a Payload Protocol Identifier we don't know how to handle
    InvalidPayloadProtocolIdentifier,

    // DataChannel message has a type we don't support
    MessageType(#[from] MessageTypeError),

    // Invalid DATA_CHANNEL_OPEN message body
    DataChannelOpen(#[from] DataChannelOpenError),
}

impl std::fmt::Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEndOfBuffer { expected, actual } => {
                writeln!(
                    f,
                    "Marshal buffer was too short: (expected: {:?}, actual: {:?})",
                    expected, actual
                )
            }
            Self::InvalidPayloadProtocolIdentifier => writeln!(
                f,
                "DataChannel message payload protocol identifier is value we can't handle"
            ),
            Self::ExpectedAndActualLengthMismatch { expected, actual } => {
                writeln!(
                    f,
                    "Expected and actual length do not match: (expected: {:?}, actual: {:?})",
                    expected, actual
                )
            }
            Self::MessageType(error) => error.fmt(f),
            Self::DataChannelOpen(error) => error.fmt(f),
        }
    }
}
