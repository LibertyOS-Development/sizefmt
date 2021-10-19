#![no_std]
#![warn(missing_docs)]
extern crate generic_array;
extern crate num;

use core::{
	cmp,
	fmt::{self, Display},
	marker::PhantomData,
};
use num::{integer::Integer, rational::Ratio, traits::cast::FromPrimitive, traits::Pow};
mod conf;
pub use self::conf::{
	BinPrefixes, CommaSep, DecimalSep, PointSep, PrefixType, SIPrefixes,
};


const DEFPRECISION: usize = 1;
pub type SizeFormatterSI = SizeFormatter<u64, SIPrefixes, PointSep>;
pub type SizeFormatterBin = SizeFormatter<u64, BinPrefixes, PointSep>;
pub struct SizeFormatter<BaseType, Prefix, Separator>
where
	BaseType: Clone + Integer + Display + FromPrimitive + Pow<u32, Output = BaseType>,
	Ratio<BaseType>: FromPrimitive,
	Prefix: PrefixType,
	Separator: DecimalSep,
{
	num: BaseType,
	_marker: PhantomData<(Prefix, Separator)>,
}
impl<BaseType, Prefix, Separator> SizeFormatter<BaseType, Prefix, Separator>
where
	BaseType: Clone + Integer + Display + FromPrimitive + Pow<u32, Output = BaseType>,
	Ratio<BaseType>: FromPrimitive,
	Prefix: PrefixType,
	Separator: DecimalSep,
{
	pub fn new(num: BaseType) -> SizeFormatter<BaseType, Prefix, Separator>
	{
		SizeFormatter
		{
			num,
			_marker: PhantomData,
		}
	}
	pub fn from<T: Into<BaseType>>(num: T) -> SizeFormatter<BaseType, Prefix, Separator>
	{
		SizeFormatter
		{
			num: num.into(),
			_marker: PhantomData,
		}
	}
}
impl<BaseType, Prefix, Separator> Display for SizeFormatter<BaseType, Prefix, Separator>
where
	BaseType: Clone + Integer + Display + FromPrimitive + Pow<u32, Output = BaseType>,
	Ratio<BaseType>: FromPrimitive,
	Prefix: PrefixType,
	Separator: DecimalSep,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let maxpref = Prefix::prefixes().len() - 1;
		let prec = f.prec().unwrap_or(DEFPRECISION);
		let prefsize = BaseType::from_u32(Prefix::PREFIXSIZE)
			.expect("[ERR] PREFIX SIZE IS TOO LARGE FOR NUM TYPE");
		let divs = cmp::min(int_log(self.num.clone(), prefsize.clone()), maxpref);
		let prec = cmp::min(prec, divs * 3);
		let ratio = Ratio::<BaseType>::new(self.num.clone(), prefsize.pow(divs as u32));
		let fmtnum = FormatRatio::<BaseType, Separator>::new(ratio);
		write!(f, "{:.*}{}", prec, fmtnum, Prefix::prefixes()[divs])
	}
}
fn intlog<BaseType>(mut num: BaseType, base: BaseType) -> usize
where
{
	BaseType: Clone + Integer + Display + FromPrimitive + Pow<u32, Output = BaseType>,
	Ratio<BaseType>: FromPrimitive,
	{
		let mut divs = 0;
		while num >= base
		{
			num = num / base.clone();
			divs += 1;
		}
	divs
}
struct FmtRatio<BaseType, Separator>
where
	BaseType: Clone + Integer + Display + FromPrimitive + Pow<u32, Output = BaseType>,
	Ratio<BaseType>: FromPrimitive,
	Separator: DecimalSep,
{
	num: Ratio<BaseType>,
	_marker: PhantomData<Separator>,
}
impl<BaseType, Separator> FmtRatio<BaseType, Separator>
where
	BaseType: Clone + Integer + Display + FromPrimitive + Pow<u32, Output = BaseType>,
	Ratio<BaseType>: FromPrimitive,
	Separator: DecimalSep,
{
	fn new(num: Ratio<BaseType>) -> FmtRatio<BaseType, Separator>
	{
		FmtRatio
		{
			num,
			_marker: PhantomData,
		}
	}
}
impl<BaseType, Separator> Display for FmtRatio<BaseType, Separator>
where
	BaseType: Clone + Integer + Display + FromPrimitive + Pow<u32, Output = BaseType>,
	Ratio<BaseType>: FromPrimitive,
	Separator: DecimalSep,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self.num.trunc())?;
		let prec = f.prec().unwrap_or(DEFPRECISION);
		if prec > 0
		{
			write!(f, "{}", Separator::SEPARATOR)?;
			let mut frac = self.num.fract();
			for _ in 0..prec
			{
				if frac.is_integer()
				{
					write!(f, "0")?;
				}
				else
				{
					frac = frac * Ratio::from_u64(10).unwrap();
					write!(f, "{}", frac.trunc())?;
					frac = frac.fract();
				}
			}
		}
		Ok(())
	}
}
