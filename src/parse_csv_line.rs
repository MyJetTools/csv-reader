use rust_extensions::array_of_bytes_iterator::{ArrayOfBytesIterator, SliceIterator};

pub struct CsvLineIterator<'s> {
    line: SliceIterator<'s>,
    separator: u8,
}

impl<'s> CsvLineIterator<'s> {
    pub fn new(src: &'s str) -> Self {
        Self {
            line: SliceIterator::new(src.as_bytes()),
            separator: b',',
        }
    }

    pub fn get_next_line(&mut self) -> Option<String> {
        if self.line.peek_value()?.value == b',' {
            self.line.get_next()?;
        }

        let start_pos = super::utils::skip_spaces(&mut self.line)?;

        let item = match start_pos.value {
            b'"' => super::utils::find_end_of_string_with_quotes(&mut self.line),
            b',' => {
                return Some("".to_string());
            }
            _ => {
                super::utils::find_value(&mut self.line, self.separator);
                self.line.get_slice_to_current_pos(start_pos.pos).to_vec()
            }
        };

        let item = String::from_utf8(item).unwrap();
        return Some(item);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_mixed_line() {
        let src = "first, second, \"third line\", \"fourth line\", \"fifth \"\" line\"";

        let mut line = super::CsvLineIterator::new(src);

        assert_eq!("first", line.get_next_line().unwrap());

        assert_eq!("second", line.get_next_line().unwrap());

        assert_eq!("third line", line.get_next_line().unwrap());

        assert_eq!("fourth line", line.get_next_line().unwrap());

        assert_eq!("fifth \" line", line.get_next_line().unwrap());

        assert_eq!(None, line.get_next_line());
    }

    #[test]
    fn test_next_line() {
        let src = "first, second,,";

        let mut line = super::CsvLineIterator::new(src);

        assert_eq!("first", line.get_next_line().unwrap());

        assert_eq!("second", line.get_next_line().unwrap());

        assert_eq!("", line.get_next_line().unwrap());

        assert_eq!(None, line.get_next_line());
    }
}
