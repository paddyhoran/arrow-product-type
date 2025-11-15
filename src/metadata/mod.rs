use flags::Flags;
use possible_dimensions::PossibleDimensions;


mod flags;
mod possible_dimensions;


/// Holds meta-data that allows the actual data
/// array to be interpreted.
pub struct MetaData {

    /// The dimensions that the data actually "varies by".
    flags: Flags,

    /// The current possible dimensions.
    dims: PossibleDimensions,
}
