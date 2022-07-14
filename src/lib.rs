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

pub type Alphabet = [char; 256];

pub trait Base {
	const ALPHABET: Alphabet;
	
	fn get_index(c: char) -> Option<u8>;

	fn decode(input: &str) -> Result<Vec<u8>, DecodeError> {
		let s = input.chars().count();
		let mut output = vec![0; s];
		for (i, c) in input.chars().enumerate() {
			output[i] = Self::get_index(c).ok_or(DecodeError {
				codepoint: c,
				index: i,
			})?;
		}

		Ok(output)
	}

	fn encode(input: &Vec<u8>) -> String {
		let s = input
			.iter()
			.map(|&x| Self::ALPHABET[x as usize].len_utf8())
			.sum();
		let mut output: Vec<u8> = vec![0; s];
		let mut i = 0;
		for &v in input.iter() {
			let c = Self::ALPHABET[v as usize];
			c.encode_utf8(&mut output[i..]);
			i += c.len_utf8();
		}

		String::from_utf8(output).unwrap()
	}
}

#[derive(Debug, Default)]
pub struct Emoji;

macro_rules! gen_alphabet {
	($name:ident, $alphabet:literal) => {
		impl Base for $name {
			const ALPHABET: [char; 256] = const_str::to_char_array!($alphabet);

			fn get_index(c: char) -> Option<u8> {
				match_lookup::gen_char_match!(c, $alphabet).map(|c| c as u8)
			}
		}
	};
}

gen_alphabet!(Emoji, "🚀🪐☄🛰🌌🌑🌒🌓🌔🌕🌖🌗🌘🌍🌏🌎🐉☀💻🖥💾💿😂❤😍🤣😊🙏💕😭😘👍😅👏😁🔥🥰💔💖💙😢🤔😆🙄💪😉☺👌🤗💜😔😎😇🌹🤦🎉💞✌✨🤷😱😌🌸🙌😋💗💚😏💛🙂💓🤩😄😀🖤😃💯🙈👇🎶😒🤭❣😜💋👀😪😑💥🙋😞😩😡🤪👊🥳😥🤤👉💃😳✋😚😝😴🌟😬🙃🍀🌷😻😓⭐✅🥺🌈😈🤘💦✔😣🏃💐☹🎊💘😠☝😕🌺🎂🌻😐🖕💝🙊😹🗣💫💀👑🎵🤞😛🔴😤🌼😫⚽🤙☕🏆🤫👈😮🙆🍻🍃🐶💁😲🌿🧡🎁⚡🌞🎈❌✊👋😰🤨😶🤝🚶💰🍓💢🤟🙁🚨💨🤬✈🎀🍺🤓😙💟🌱😖👶🥴▶➡❓💎💸⬇😨🌚🦋😷🕺⚠🙅😟😵👎🤲🤠🤧📌🔵💅🧐🐾🍒😗🤑🌊🤯🐷☎💧😯💆👆🎤🙇🍑❄🌴💣🐸💌📍🥀🤢👅💡💩👐📸👻🤐🤮🎼🥵🚩🍎🍊👼💍📣🥂");

#[cfg(test)]
mod tests {
	use crate::{Base, Emoji};

	#[test]
	fn byte1_rt() {
		let mut org = vec![0u8; 1];
		for i in 0..255 {
			org[0] = i;
			let e = Emoji::encode(&org);
			let r = match Emoji::decode(e.as_str()) {
				Ok(x) => x,
				Err(e) => {
					panic!("{}", e);
				}
			};
			assert_eq!(org, r)
		}
	}

	#[test]
	fn index() {
		for (i, &c) in Emoji::ALPHABET.iter().enumerate(){
			println!("i{}:{}, {:?}", i, c, Emoji::get_index(c));
			assert!(i as u8 == Emoji::get_index(c).unwrap());
		}
	}

	#[test]
	fn juan() {
		let hi = "hi juan!";
		println!("{}: {}", hi, Emoji::encode(&hi.as_bytes().to_owned()));
	}
}
