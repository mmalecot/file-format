//! Utils.

/// Checks if the `data` array contains the `target` sequence using the Boyer-Moore algorithm.
pub(crate) fn contains(data: &[u8], target: &[u8]) -> bool {
    let data_len = data.len();
    let target_len = target.len();
    if target_len > data_len {
        return false;
    }
    let mut skip_table = [0; 256];
    for (index, &val) in target.iter().enumerate().rev() {
        skip_table[val as usize] = index;
    }
    let mut data_index = target_len - 1;
    while data_index < data_len {
        let mut target_index = target_len;
        while target_index > 0 && target[target_index - 1] == data[data_index] {
            target_index -= 1;
            data_index -= 1;
        }
        if target_index == 0 {
            return true;
        }
        if target_index < skip_table[data[data_index] as usize] + 1 {
            data_index += target_len - target_index;
        } else {
            data_index += target_len - skip_table[data[data_index] as usize] - 1;
        }
    }
    false
}
