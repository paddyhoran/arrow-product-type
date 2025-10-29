use bitvec::vec::BitVec;

pub struct Flags {
    /// Indicates the dimensions that are in use.
    flags: BitVec,
    /// Indicates the run-lengths for each dimension.
    run_lengths: Vec<usize>,
}

/// Returns a run-length `Vec` indicating the run-lengths for each dimension represented in
/// `flags` / `sizes`.
///
/// `flags` indicates which dimensions are in use, the returned run-lengths will have the same
/// size as `flags`.
///
/// `sizes` indicates the size of each dimension.  `flags` and `sizes` should be the same length
/// and should be ordered in a consistent manner.
///
/// The run-length for a specific slot will be `0` if the corresponding slot in `flags` is `false`.
fn compute_run_lengths(flags: &BitVec, sizes: &[usize]) -> Vec<usize> {
    let dims = sizes
        .iter()
        .zip(flags.iter())
        .map(|(size, flag)| if *flag { *size } else { 1 })
        .collect::<Vec<_>>();

    let max_array_size = dims.iter().copied().reduce(|a, b| a * b).unwrap_or(1);

    let mut current_size = max_array_size;
    let mut run_lengths = Vec::with_capacity(dims.len());
    for (flag, dim) in flags.iter().zip(dims.iter()) {
        if *flag {
            let run_length = current_size / *dim;
            run_lengths.push(run_length);
            current_size = run_length;
        } else {
            run_lengths.push(0);
        }
    }
    run_lengths
}

#[cfg(test)]
mod tests {

    use super::*;

    fn create_bitvec(flags: &[bool]) -> BitVec {
        let mut vec = BitVec::with_capacity(flags.len());
        for f in flags.iter() {
            vec.push(*f);
        }
        vec
    }

    /// Tests the expected run-lengths when you have only a single dimension.
    #[test]
    fn test_1d_run_lengths() {
        let sizes = vec![3, 2, 3];

        let flags = create_bitvec(&[true, false, false]);
        let run_lengths = compute_run_lengths(&flags, &sizes);
        assert_eq!(run_lengths, vec![1, 0, 0]);

        let flags = create_bitvec(&[false, true, false]);
        let run_lengths = compute_run_lengths(&flags, &sizes);
        assert_eq!(run_lengths, vec![0, 1, 0]);

        let flags = create_bitvec(&[false, false, true]);
        let run_lengths = compute_run_lengths(&flags, &sizes);
        assert_eq!(run_lengths, vec![0, 0, 1]);
    }
}
