fn main() {
	println!("The answer to life, universe and everything: {}", get_answer());
}

fn get_answer() -> u32 {
	42
}

#[cfg(test)]
mod test {
	use crate::*;

	#[test]
	fn test_answer() {
		assert_eq!(get_answer(), 42)
	}
}
