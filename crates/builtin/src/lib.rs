/// The `copy` built-in function copies elements from a source slice into a
/// destination slice. (As a special case, it also will copy bytes from a
/// string to a slice of bytes.) The source and destination may overlap. `copy`
/// returns the number of elements copied, which will be the minimum of
/// `src.len()` and `dst.len()`.
pub fn copy<T>(dst: &mut [T], src: &[T]) -> usize
where
    T: Copy,
{
    let ell = std::cmp::min(dst.len(), src.len());
    dst[..ell].copy_from_slice(&src[..ell]);

    ell
}
