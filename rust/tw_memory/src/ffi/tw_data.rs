// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use crate::ffi::c_byte_array_ref::CByteArrayRef;
use crate::ffi::RawPtrTrait;

/// Defines a resizable block of data.
///
/// The implementation of these methods should be language-specific to minimize translation overhead.
/// For instance it should be a `jbyteArray` for Java and an `NSData` for Swift.
#[derive(Debug, Default)]
pub struct TWData(Vec<u8>);

impl TWData {
    /// Returns an empty `TWData` instance.
    pub fn new() -> TWData {
        TWData(Vec::new())
    }

    /// Creates a `TWData` from a raw byte array.
    pub unsafe fn from_raw_data(bytes: *const u8, size: usize) -> Option<TWData> {
        CByteArrayRef::new(bytes, size).to_vec().map(TWData)
    }

    /// Converts `TWData` into `Vec<u8>` without additional allocation.
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }

    /// Returns the data slice.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    /// Returns a pointer to the data.
    pub fn data(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Returns a length of the data.
    pub fn size(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<u8>> for TWData {
    fn from(data: Vec<u8>) -> Self {
        TWData(data)
    }
}

impl RawPtrTrait for TWData {}

/// Creates a block of data from a byte array.
///
/// \param bytes Non-null raw bytes buffer
/// \param size size of the buffer
/// \return Non-null filled block of data.
#[no_mangle]
pub unsafe extern "C" fn tw_data_create_with_bytes(bytes: *const u8, size: usize) -> *mut TWData {
    TWData::from_raw_data(bytes, size)
        .map(|data| data.into_ptr())
        .unwrap_or_else(std::ptr::null_mut)
}

/// Returns the raw pointer to the contents of data.
///
/// \param data A non-null valid block of data
/// \return the raw pointer to the contents of data
#[no_mangle]
pub unsafe extern "C" fn tw_data_bytes(data: *mut TWData) -> *const u8 {
    TWData::from_ptr_as_ref(data)
        .map(TWData::data)
        .unwrap_or_else(std::ptr::null)
}

/// Returns the size in bytes.
///
/// \param data A non-null valid block of data
/// \return the size of the given block of data
#[no_mangle]
pub unsafe extern "C" fn tw_data_size(data: *mut TWData) -> usize {
    TWData::from_ptr_as_ref(data)
        .map(|data| data.size())
        .unwrap_or_default()
}
