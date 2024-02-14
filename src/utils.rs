use rust_extensions::array_of_bytes_iterator::{ArrayOfBytesIterator, NextValue, SliceIterator};

pub fn skip_spaces(slice_iterator: &mut SliceIterator) -> Option<NextValue> {
    while let Some(item) = slice_iterator.peek_value() {
        if item.value > 32 {
            return Some(item);
        }

        slice_iterator.get_next();
    }

    None
}

pub fn find_end_of_string_with_quotes(slice_iterator: &mut SliceIterator) -> Vec<u8> {
    slice_iterator.get_next();
    let mut result = Vec::new();
    while let Some(item) = slice_iterator.get_next() {
        if item.value == b'"' {
            match slice_iterator.peek_value() {
                Some(next_value) => {
                    if next_value.value == b'"' {
                        result.push(b'"');
                        slice_iterator.get_next();
                        continue;
                    } else if next_value.value == b',' {
                        slice_iterator.get_next();
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }

        result.push(item.value);
    }

    result
}

pub fn find_value(slice_iterator: &mut SliceIterator, value: u8) -> Option<usize> {
    while let Some(item) = slice_iterator.peek_value() {
        if item.value == value {
            return Some(item.pos);
        }
        slice_iterator.get_next();
    }

    None
}
