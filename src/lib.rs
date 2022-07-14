#![recursion_limit = "256"]
use std::fmt;

use heapless::FnvIndexMap;

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

pub trait Base: Default {
	const ALPHABET: Alphabet;
	// should be moved to std oncecell at some point

	fn get_index(&self, c: char) -> Option<u8>;

	fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeError> {
		let s = input.chars().count();
		let mut output = vec![0; s];
		for (i, c) in input.chars().enumerate() {
			output[i] = Self::get_index(self, c).ok_or(DecodeError {
				codepoint: c,
				index: i,
			})?;
		}

		Ok(output)
	}

	fn encode(&self, input: &Vec<u8>) -> String {
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

#[derive(Debug)]
pub struct Emoji {
	lookup: FnvIndexMap<char, u8, 256>
}

#[derive(Debug, Default)]
pub struct EmojiE;

#[derive(Debug, Default)]
pub struct EmojiM;

impl Default for Emoji {
    fn default() -> Self {
		let mut lookup = FnvIndexMap::new();

		for (i, &c) in Self::ALPHABET.iter().enumerate() {
			lookup.insert(c, i as u8).unwrap();
		}

        Self { lookup }
    }
}

impl Base for Emoji {
	const ALPHABET: Alphabet = [
		'🚀', '🪐', '☄', '🛰', '🌌', '🌑', '🌒', '🌓', '🌔', '🌕', '🌖', '🌗', '🌘', '🌍', '🌏',
		'🌎', '🐉', '☀', '💻', '🖥', '💾', '💿', '😂', '❤', '😍', '🤣', '😊', '🙏', '💕', '😭',
		'😘', '👍', '😅', '👏', '😁', '🔥', '🥰', '💔', '💖', '💙', '😢', '🤔', '😆', '🙄',
		'💪', '😉', '☺', '👌', '🤗', '💜', '😔', '😎', '😇', '🌹', '🤦', '🎉', '💞', '✌', '✨',
		'🤷', '😱', '😌', '🌸', '🙌', '😋', '💗', '💚', '😏', '💛', '🙂', '💓', '🤩', '😄',
		'😀', '🖤', '😃', '💯', '🙈', '👇', '🎶', '😒', '🤭', '❣', '😜', '💋', '👀', '😪',
		'😑', '💥', '🙋', '😞', '😩', '😡', '🤪', '👊', '🥳', '😥', '🤤', '👉', '💃', '😳',
		'✋', '😚', '😝', '😴', '🌟', '😬', '🙃', '🍀', '🌷', '😻', '😓', '⭐', '✅', '🥺',
		'🌈', '😈', '🤘', '💦', '✔', '😣', '🏃', '💐', '☹', '🎊', '💘', '😠', '☝', '😕', '🌺',
		'🎂', '🌻', '😐', '🖕', '💝', '🙊', '😹', '🗣', '💫', '💀', '👑', '🎵', '🤞', '😛',
		'🔴', '😤', '🌼', '😫', '⚽', '🤙', '☕', '🏆', '🤫', '👈', '😮', '🙆', '🍻', '🍃',
		'🐶', '💁', '😲', '🌿', '🧡', '🎁', '⚡', '🌞', '🎈', '❌', '✊', '👋', '😰', '🤨',
		'😶', '🤝', '🚶', '💰', '🍓', '💢', '🤟', '🙁', '🚨', '💨', '🤬', '✈', '🎀', '🍺',
		'🤓', '😙', '💟', '🌱', '😖', '👶', '🥴', '▶', '➡', '❓', '💎', '💸', '⬇', '😨', '🌚',
		'🦋', '😷', '🕺', '⚠', '🙅', '😟', '😵', '👎', '🤲', '🤠', '🤧', '📌', '🔵', '💅',
		'🧐', '🐾', '🍒', '😗', '🤑', '🌊', '🤯', '🐷', '☎', '💧', '😯', '💆', '👆', '🎤',
		'🙇', '🍑', '❄', '🌴', '💣', '🐸', '💌', '📍', '🥀', '🤢', '👅', '💡', '💩', '👐',
		'📸', '👻', '🤐', '🤮', '🎼', '🥵', '🚩', '🍎', '🍊', '👼', '💍', '📣', '🥂',
	];

	fn get_index(&self, c: char) -> Option<u8> {
		self.lookup.get(&c).copied()
	}
}

impl Base for EmojiE {
	const ALPHABET: Alphabet = [
		'🚀', '🪐', '☄', '🛰', '🌌', '🌑', '🌒', '🌓', '🌔', '🌕', '🌖', '🌗', '🌘', '🌍', '🌏',
		'🌎', '🐉', '☀', '💻', '🖥', '💾', '💿', '😂', '❤', '😍', '🤣', '😊', '🙏', '💕', '😭',
		'😘', '👍', '😅', '👏', '😁', '🔥', '🥰', '💔', '💖', '💙', '😢', '🤔', '😆', '🙄',
		'💪', '😉', '☺', '👌', '🤗', '💜', '😔', '😎', '😇', '🌹', '🤦', '🎉', '💞', '✌', '✨',
		'🤷', '😱', '😌', '🌸', '🙌', '😋', '💗', '💚', '😏', '💛', '🙂', '💓', '🤩', '😄',
		'😀', '🖤', '😃', '💯', '🙈', '👇', '🎶', '😒', '🤭', '❣', '😜', '💋', '👀', '😪',
		'😑', '💥', '🙋', '😞', '😩', '😡', '🤪', '👊', '🥳', '😥', '🤤', '👉', '💃', '😳',
		'✋', '😚', '😝', '😴', '🌟', '😬', '🙃', '🍀', '🌷', '😻', '😓', '⭐', '✅', '🥺',
		'🌈', '😈', '🤘', '💦', '✔', '😣', '🏃', '💐', '☹', '🎊', '💘', '😠', '☝', '😕', '🌺',
		'🎂', '🌻', '😐', '🖕', '💝', '🙊', '😹', '🗣', '💫', '💀', '👑', '🎵', '🤞', '😛',
		'🔴', '😤', '🌼', '😫', '⚽', '🤙', '☕', '🏆', '🤫', '👈', '😮', '🙆', '🍻', '🍃',
		'🐶', '💁', '😲', '🌿', '🧡', '🎁', '⚡', '🌞', '🎈', '❌', '✊', '👋', '😰', '🤨',
		'😶', '🤝', '🚶', '💰', '🍓', '💢', '🤟', '🙁', '🚨', '💨', '🤬', '✈', '🎀', '🍺',
		'🤓', '😙', '💟', '🌱', '😖', '👶', '🥴', '▶', '➡', '❓', '💎', '💸', '⬇', '😨', '🌚',
		'🦋', '😷', '🕺', '⚠', '🙅', '😟', '😵', '👎', '🤲', '🤠', '🤧', '📌', '🔵', '💅',
		'🧐', '🐾', '🍒', '😗', '🤑', '🌊', '🤯', '🐷', '☎', '💧', '😯', '💆', '👆', '🎤',
		'🙇', '🍑', '❄', '🌴', '💣', '🐸', '💌', '📍', '🥀', '🤢', '👅', '💡', '💩', '👐',
		'📸', '👻', '🤐', '🤮', '🎼', '🥵', '🚩', '🍎', '🍊', '👼', '💍', '📣', '🥂',
	];

	fn get_index(&self, c: char) -> Option<u8> {
		Self::ALPHABET.iter().position(|&x| c == x).map(|c| c as u8)
	}
}

macro_rules! gen_alphabet {
	($name:ident, $alphabet:literal) => {
		impl Base for $name {
			const ALPHABET: [char; 256] = const_str::to_char_array!($alphabet);

			fn get_index(&self, c: char) -> Option<u8> {
				match_lookup::gen_char_match!(c, $alphabet).map(|c| c as u8)
			}
		}
	};
}

gen_alphabet!(EmojiM, "🚀🪐☄🛰🌌🌑🌒🌓🌔🌕🌖🌗🌘🌍🌏🌎🐉☀💻🖥💾💿😂❤😍🤣😊🙏💕😭😘👍😅👏😁🔥🥰💔💖💙😢🤔😆🙄💪😉☺👌🤗💜😔😎😇🌹🤦🎉💞✌✨🤷😱😌🌸🙌😋💗💚😏💛🙂💓🤩😄😀🖤😃💯🙈👇🎶😒🤭❣😜💋👀😪😑💥🙋😞😩😡🤪👊🥳😥🤤👉💃😳✋😚😝😴🌟😬🙃🍀🌷😻😓⭐✅🥺🌈😈🤘💦✔😣🏃💐☹🎊💘😠☝😕🌺🎂🌻😐🖕💝🙊😹🗣💫💀👑🎵🤞😛🔴😤🌼😫⚽🤙☕🏆🤫👈😮🙆🍻🍃🐶💁😲🌿🧡🎁⚡🌞🎈❌✊👋😰🤨😶🤝🚶💰🍓💢🤟🙁🚨💨🤬✈🎀🍺🤓😙💟🌱😖👶🥴▶➡❓💎💸⬇😨🌚🦋😷🕺⚠🙅😟😵👎🤲🤠🤧📌🔵💅🧐🐾🍒😗🤑🌊🤯🐷☎💧😯💆👆🎤🙇🍑❄🌴💣🐸💌📍🥀🤢👅💡💩👐📸👻🤐🤮🎼🥵🚩🍎🍊👼💍📣🥂");


#[cfg(test)]
mod tests {
	use crate::Base;
	use crate::Emoji;

	#[test]
	fn byte1_rt() {
		let mut org = vec![0u8; 1];
		for i in 0..255 {
			org[0] = i;
			let a = Emoji::default();
			let r = match a.decode(a.encode(&org).as_str()) {
				Ok(x) => x,
				Err(e) => {
					panic!("{}", e);
				}
			};
			assert_eq!(org, r)
		}
	}
}
