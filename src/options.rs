use passwords::PasswordGenerator;
use structopt::StructOpt;

#[derive(StructOpt, Clone, Copy, Debug, Default)]
#[structopt(name = "pwg")]
/// A password generator for the command-line.
pub struct Options {
    #[structopt(short, long, default_value = "21")]
    /// Password length
    length: usize,

    #[structopt(long)]
    /// Exclude numbers
    exclude_numbers: bool,

    #[structopt(long)]
    /// Exclude lowercase letters
    exclude_lowercase: bool,

    #[structopt(long)]
    /// Exclude uppercase letters
    exclude_uppercase: bool,

    #[structopt(long)]
    /// Exclude symbols
    exclude_symbols: bool,

    #[structopt(long)]
    /// Include spaces
    spaces: bool,

    #[structopt(long)]
    /// Exclude similar characters
    exclude_similar: bool,
}

impl From<Options> for PasswordGenerator {
    fn from(options: Options) -> Self {
        PasswordGenerator {
            length: options.length,
            numbers: !options.exclude_numbers,
            lowercase_letters: !options.exclude_lowercase,
            uppercase_letters: !options.exclude_uppercase,
            symbols: !options.exclude_symbols,
            spaces: options.spaces,
            exclude_similar_characters: options.exclude_similar,
            strict: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length() {
        let mut options = Options::default();
        options.length = 42;
        let pg: PasswordGenerator = options.into();
        assert_eq!(options.length, pg.length);
        assert_eq!(pg.length, 42);
    }

    #[test]
    fn numbers() {
        let mut options = Options::default();
        let pg: PasswordGenerator = options.into();

        // numbers should be allowed by default
        assert!(pg.numbers);

        // confirm exclusion of numbers
        options.exclude_numbers = true;
        let pg: PasswordGenerator = options.into();
        assert!(!pg.numbers);
    }

    #[test]
    fn lowercase_letters() {
        let mut options = Options::default();
        let pg: PasswordGenerator = options.into();

        // lowercase letters should be allowed by default
        assert!(pg.lowercase_letters);

        // confirm exclusion of lowercase letters
        options.exclude_lowercase = true;
        let pg: PasswordGenerator = options.into();
        assert!(!pg.lowercase_letters);
    }

    #[test]
    fn uppercase_letters() {
        let mut options = Options::default();
        let pg: PasswordGenerator = options.into();

        // uppercase letters should be allowed by default
        assert!(pg.uppercase_letters);

        // confirm exclusion of uppercase letters
        options.exclude_uppercase = true;
        let pg: PasswordGenerator = options.into();
        assert!(!pg.uppercase_letters);
    }

    #[test]
    fn symbols() {
        let mut options = Options::default();
        let pg: PasswordGenerator = options.into();

        // symbols should be allowed by default
        assert!(pg.symbols);

        // confirm exclusion of symbols
        options.exclude_symbols = true;
        let pg: PasswordGenerator = options.into();
        assert!(!pg.symbols);
    }

    #[test]
    fn spaces() {
        let mut options = Options::default();
        let pg: PasswordGenerator = options.into();

        // spaces should be excluded by default
        assert!(!pg.spaces);

        // confirm inclusion of spaces
        options.spaces = true;
        let pg: PasswordGenerator = options.into();
        assert!(pg.spaces);
    }

    #[test]
    fn similar_characters() {
        let mut options = Options::default();
        let pg: PasswordGenerator = options.into();

        // similar characters should be allowed by default
        assert!(!pg.exclude_similar_characters);

        // confirm inclusion of similar characters
        options.exclude_similar = true;
        let pg: PasswordGenerator = options.into();
        assert!(pg.exclude_similar_characters);
    }

    #[test]
    fn strict() {
        let options = Options::default();
        let pg: PasswordGenerator = options.into();

        // strict should be false
        assert!(!pg.strict);
    }
}
