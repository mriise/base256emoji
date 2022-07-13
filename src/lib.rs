use std::fmt;

#[derive(Debug, Clone)]
pub struct DecodeError {
	codepoint: char,
	index: usize,
}

impl fmt::Display for DecodeError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{} at index {} is not part of the alphabet",
			self.codepoint, self.index
		)
	}
}

type Alphabet = [char; 256];

pub trait Base {
	const ALPHABET: Alphabet;

	fn decode(input: &str) -> Result<Vec<u8>, DecodeError> {
		let s = input.chars().count();
		let mut output = Vec::with_capacity(s);
		for (i, c) in input.chars().enumerate() {
			output[i] = match Self::ALPHABET.iter().position(|&x| x == c) {
				Some(c) => c.try_into().unwrap(),
				None => {
					return Err(DecodeError {
						codepoint: c,
						index: i,
					});
				}
			}
		}

		Ok(output)
	}

	fn encode(input: Vec<u8>) -> String {
		let s = input
			.iter()
			.map(|&x| Self::ALPHABET[x as usize].len_utf8())
			.sum();
		let mut output: Vec<u8> = Vec::with_capacity(s);
		let mut i = 0;
		for &v in input.iter() {
			let c = Self::ALPHABET[v as usize];
			c.encode_utf8(&mut output[i..]);
			i += c.len_utf8();
		}

		String::from_utf8(output).unwrap()
	}
}
