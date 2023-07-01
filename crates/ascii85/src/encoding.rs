use crate::errors::CorruptInputError;

/// Decodes `src` into `dst`, returning both the number
/// of bytes written to `dst` and the number consumed from `src`.
/// If `src` contains invalid ascii85 data, `decode` will return  a
/// [CorruptInputError](crate::CorruptInputError).
/// `decode` ignores space and control characters in `src`.
/// Often, ascii85-encoded data is wrapped in <~ and ~> symbols.
/// `decode` expects these to have been stripped by the caller.
///
/// If flush is true, `decode` assumes that `src` represents the
/// end of the input stream and processes it completely rather
/// than wait for the completion of another 32-bit block.
///
/// [new_decoder](crate::new_decoder) wraps an io::Read
/// interface around `decode`.
pub fn decode(
    dst: &mut [u8],
    src: &[u8],
    flush: bool,
) -> Result<(usize, usize), CorruptInputError> {
    let (mut v, mut nb, mut ndst, mut nsrc) = (0u32, 0, 0usize, 0usize);

    for (i, &b) in src.iter().enumerate() {
        if dst.len() - ndst < 4 {
            return Ok((ndst, nsrc));
        }

        if b <= b' ' {
            continue;
        } else if b == b'z' && nb == 0 {
            nb = 5;
            v = 0;
        } else if (b'!'..=b'u').contains(&b) {
            nb += 1;
            v = v * 85 + (b - b'!') as u32;
        } else {
            return Err(CorruptInputError::new(src, i, ndst));
        }

        if nb == 5 {
            nsrc = i + 1;
            dst[ndst] = (v >> 24) as u8;
            dst[ndst + 1] = (v >> 16) as u8;
            dst[ndst + 2] = (v >> 8) as u8;
            dst[ndst + 3] = v as u8;
            ndst += 4;
            nb = 0;
            v = 0;
        }
    }

    if !flush || nb == 0 {
        return Ok((ndst, nsrc));
    }

    // The number of output bytes in the last fragment
    // is the number of leftover input bytes - 1:
    // the extra byte provides enough bits to cover
    // the inefficiency of the encoding for the block.
    if nb == 1 {
        return Err(CorruptInputError::new(src, src.len(), ndst));
    }

    for _i in nb..5 {
        // The short encoding truncated the output value.
        // We have to assume the worst case values (digit 84)
        // in order to ensure that the top bits are correct.
        v = v * 85 + 84
    }

    for _i in 0..(nb - 1) {
        dst[ndst] = (v >> 24) as u8;
        v <<= 8;
        ndst += 1;
    }

    nsrc = src.len();

    Ok((ndst, nsrc))
}

/// Encodes `src` into at most [max_encoded_len(src.len())](max_encoded_len)
/// bytes of `dst`, returning the actual number of bytes written.
//
/// The encoding handles 4-byte chunks, using a special encoding
/// for the last fragment, so `encode` is not appropriate for use on
/// individual blocks of a large data stream. Use
/// [new_encoder()](crate::new_encoder) instead.
//
/// Often, ascii85-encoded data is wrapped in <~ and ~> symbols.
/// `encode` does not add these.
pub fn encode(dst: &mut [u8], src: &[u8]) -> usize {
    if src.is_empty() {
        return 0;
    }

    let mut written = 0usize;
    let (mut dst_idx, mut src_idx) = (0, 0);
    while src_idx < src.len() {
        let (dst, src) = (&mut dst[dst_idx..], &src[src_idx..]);

        dst[0..5].fill(0);

        // Unpack 4 bytes into uint32 to repack into base 85 5-byte.
        let mut v = 0u32;
        if src.len() >= 4 {
            v |= src[3] as u32;
        }
        if src.len() >= 3 {
            v |= (src[2] as u32) << 8;
        }
        if src.len() >= 2 {
            v |= (src[1] as u32) << 16;
        }
        if !src.is_empty() {
            v |= (src[0] as u32) << 24;
        }

        // Special case: zero (!!!!!) shortens to z.
        if v == 0 && src.len() >= 4 {
            dst[0] = b'z';
            dst_idx += 1;
            src_idx += 4;
            written += 1;
            continue;
        }

        // Otherwise, 5 base 85 digits starting at !.
        for i in (0..=4).rev() {
            dst[i] = (('!' as u32) + (v % 85)) as u8;
            v /= 85;
        }

        // If src was short, discard the low destination bytes.
        let mut m = 5usize;
        if src.len() < 4 {
            m -= 4 - src.len();
            src_idx += src.len();
        } else {
            src_idx += 4;
        }
        dst_idx += m;
        written += m;
    }

    written
}

/// Returns the maximum length of an encoding of `n` source bytes.
pub fn max_encoded_len(n: usize) -> usize {
    (n + 3) / 4 * 5
}
