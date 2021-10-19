use generic_array::{typenum::U9, ArrayLength, GenericArray};

pub trait DecimalSep
{
	const SEPARATOR: char;
}
pub struct CommaSep;
impl DecimalSep for CommaSep
{
	const SEPARATOR: char = ',';
}
pub struct PointSep;
impl DecimalSep for PointSep
{
	const SEPARATOR: char = '.';
}
pub trait PrefixType
{
	type N: ArrayLength<&'static str>;
	const PREFIXSIZE: u32;
	fn prefixes() -> GenericArray<&'static str, Self::N>;
}
pub struct SIPrefixes;
impl PrefixType for SIPrefixes
{
	type N = U9;
	const PREFIXSIZE: u32 = 1000;
	fn prefixes() -> GenericArray<&'static str, Self::N>
	{
		["", "k", "M", "G", "T", "P", "E", "Z", "Y"].into()
	}
}
pub struct BinPrefixes;
impl PrefixType for BinPrefixes
{
	type N = U9;
	const PREFIXSIZE: u32 = 1024;
	fn prefixes() -> GenericArray<&'static str, Self::N>
	{
		["", "K", "M", "G", "T", "P", "E", "Z", "Y"].into()
	}
}
