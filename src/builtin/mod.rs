pub fn copy<T>(dst: &mut [T], src: &[T]) -> usize
where
    T: Copy,
{
    let ell = std::cmp::min(dst.len(), src.len());
    dst[..ell].copy_from_slice(&src[..ell]);

    ell
}
