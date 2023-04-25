use crate::de::{BorrowDecoder, Decoder};
use crate::enc::Encoder;
use crate::error::{DecodeError, EncodeError};
use crate::{BorrowDecode, Decode, Encode};
use alloc::vec::Vec;
use generic_array::{ArrayLength, GenericArray};

impl<T: Decode + 'static, N: ArrayLength> Decode for GenericArray<T, N> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let buf: Vec<T> = crate::de::Decode::decode(decoder)?;
        let buf_len = buf.len();
        GenericArray::from_exact_iter(buf.into_iter()).ok_or(DecodeError::ArrayLengthMismatch {
            required: N::USIZE,
            found: buf_len,
        })
    }
}

impl<T: Encode + 'static, N: ArrayLength> Encode for GenericArray<T, N> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::Encode::encode(self.as_slice(), encoder)
    }
}

impl<'de, T, N> BorrowDecode<'de> for GenericArray<T, N>
where
    T: BorrowDecode<'de>,
    N: ArrayLength,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        if len != N::USIZE {
            return Err(DecodeError::ArrayLengthMismatch {
                required: N::USIZE,
                found: len,
            });
        }
        decoder.claim_container_read::<T>(len)?;
        let mut vec: Vec<T> = Vec::with_capacity(len);
        for _ in 0..len {
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());
            vec.push(T::borrow_decode(decoder)?);
        }
        generic_array_from_vec(vec)
    }
}

fn generic_array_from_vec<T, N: ArrayLength>(
    it: Vec<T>,
) -> Result<GenericArray<T, N>, DecodeError> {
    let len = it.len();
    GenericArray::from_exact_iter(it).ok_or(DecodeError::ArrayLengthMismatch {
        required: N::USIZE,
        found: len,
    })
}
