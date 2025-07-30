pub mod error;

pub use error::CliError;

#[doc(hidden)]
pub mod __internal_prelude {
    pub use indexmap;
    pub use serde;
    pub use std;
}

#[macro_export]
#[doc(hidden)]
macro_rules! __internal_impl_getter {
	// base case: no more fields to process
    ($snug_type:ty, $shift:expr, ) => {};

	// recursion case: if field num bits is one it returns a bool
	(
        $snug_type:ty,
        $shift:expr,
        $field_name:ident = 1,
        $( $rest:tt )*
    ) => {
        pub const fn $field_name(&self) -> bool {
            (self.data >> $shift) & 1 != 0
        }

        $crate::__internal_impl_getter! {
 			$snug_type,
            $shift + 1,
            $( $rest )*
		}
    };

	// recursion case: if field num bits is plural it returns snug type
    (
        $snug_type:ty,
        $shift:expr,
        $field_name:ident = $field_num_bits:literal,
        $( $rest:tt )*
    ) => {
        pub const fn $field_name(&self) -> $snug_type {
            let mask: $snug_type = (1 << $field_num_bits) - 1;
            (self.data >> $shift) & mask
        }

        $crate::__internal_impl_getter! {
 			$snug_type,
            $shift + $field_num_bits,
            $( $rest )*
		}
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __internal_field_type {
    ($snug_type:ty, 1) => {
        bool
    };
    ($snug_type:ty, $field_num_bits:expr) => {
        $snug_type
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __internal_field_value {
    ($val:expr, 1) => {
        $val != 0
    };
    ($val:expr, $field_num_bits:literal) => {
        $val
    };
}

/// Macro to create a dictionary.
/// Can create individual words or store them and access by their id.
/// Capable of loading a list of words from toml.
///
/// # Examples
///
/// ```
/// use mac_dictionary::dictionary;
///
/// dictionary! {
///     r#type: u64,
///     id = 30,
///     wow = 3,
///     binary = 1,
///		ctx = 5,
/// }
///
/// fn main() {
/// 	let word: Word = Word::new(1000, 0, true, 3);
///
/// 	assert_eq!(word.id(), 1000);
/// 	assert_eq!(word.wow(), 0);
/// 	assert!(word.binary());
/// 	assert_eq!(word.ctx(), 3);
/// }
/// ```
#[macro_export]
macro_rules! dictionary {
    (
        r#type: $snug_type:ty,
        $( $field_name:ident = $field_num_bits:tt ),*
        $(,)?
    ) => {
		pub use __internal_dictionary::*;

		mod __internal_dictionary {
			use $crate::__internal_prelude::{
                serde::Deserialize,
				indexmap::IndexMap,
				std::path::Path,
				std::sync::OnceLock,
				std::fs,
				std::fmt
            };

			use $crate::error::CliError;

			const SUM: u32 = 0 $( + $field_num_bits )*;
			const _: () = assert!(SUM <= (std::mem::size_of::<$snug_type>() * 8) as u32,
					"Total bits for fields exceeds the capacity of the dictionary type!");

			#[derive(Default, Deserialize)]
			pub struct Word {
				data: $snug_type,
			}

			impl Word {
				pub const MISSING: Self = Self { data: 0 };

				pub fn new(
					$( $field_name: $crate::__internal_field_type!($snug_type, $field_num_bits) ),*
				) -> Self {
					let mut shift: $snug_type = 0;
					let data: $snug_type = 0 $(
							| {
								let field_value: $snug_type = $field_name as $snug_type;

								assert!(
									field_value < 2f32.powi($field_num_bits) as $snug_type,
									concat!("Value for '", stringify!(field_value), "' exceeds its allocated ",
										stringify!($field_num_bits), " bits!")
								);

								let mask: $snug_type = (1 << $field_num_bits) - 1;
								let value: $snug_type = (field_value & mask) << shift;

								shift += $field_num_bits as $snug_type;
								value
							}
						)*;

					Self { data }
				}

				$crate::__internal_impl_getter! {
					$snug_type,
					0,
					$( $field_name = $field_num_bits, )*
				}
			}

			impl fmt::Display for Word {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    let fields = vec![
                        $(
                            format!("{}: {}", stringify!($field_name), self.$field_name())
                        ),*
                    ];
                    write!(f, "{{ {} }}", fields.join(", "))
                }
            }

			// -- WordToml --

			#[derive(Deserialize)]
			struct WordTomlMap {
				#[serde(flatten)]
				words: IndexMap<String, WordToml>,
			}

			#[derive(Deserialize)]
			struct WordToml {
				$( $field_name: $snug_type ),*
			}

			impl From<WordToml> for Word {
				fn from(word_toml: WordToml) -> Self {
					Self::new(
						$( $crate::__internal_field_value!(word_toml.$field_name, $field_num_bits) ),*
					)
				}
			}

			// -- Dictionary --

			static DICTIONARY: OnceLock<Vec<Word>> = OnceLock::new();

			/// Get the definition of a word from the loaded dictionary given its position.
			pub fn definition(value: usize) -> &'static Word {
				match DICTIONARY.get() {
					Some(dictionary) => dictionary.get(value).unwrap_or(&Word::MISSING),
					None => {
						eprintln!("Need to initialize dictionary");
						&Word::MISSING
					}
				}
			}

			/// Initialize the static dictionary given a toml path.
			#[must_use]
			pub fn initialize_dictionary<P: AsRef<Path>>(path: P) -> Result<(), CliError> {
				DICTIONARY.set(load_words(path)?).map_err(|_| {
					CliError::IoError(std::io::Error::new(
						std::io::ErrorKind::AlreadyExists,
						"Dictionary was already initialized",
					))
				})?;

				Ok(())
			}

			/// Get an ordered vector of the created words from a toml file given a path.
			#[must_use]
			pub fn load_words<P: AsRef<Path>>(path: P) -> Result<Vec<Word>, CliError> {
				let contents: String = fs::read_to_string(path)?;
				let word_toml_map: WordTomlMap = toml::from_str(&contents)?;
				let named_toml_words: Vec<(String, WordToml)> = word_toml_map.words.into_iter().collect();

				named_toml_words
					.into_iter()
					.enumerate()
					.map(|(n, (_, word_toml))| {
						if n > (u8::MAX as usize) {
							return Err(CliError::TooManyWordsError {
								count: n,
								max_allowed: u8::MAX,
							});
						}

						Ok(Word::from(word_toml))
					})
					.collect::<Result<Vec<Word>, CliError>>()
			}
		}
    };
}
