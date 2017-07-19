// This file is part of judy. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT. No part of judy-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of judy. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT.


#[derive(Debug)]
pub struct Judy1(Pvoid_t);

impl Default for Judy1
{
	#[inline(always)]
	fn default() -> Self
	{
		Judy1(null_mut())
	}
}

impl Drop for Judy1
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { Judy1FreeArray(&mut self.0, null_mut()) };
	}
}

impl Judy1
{
	pub const MaximumIndex: c_ulong = (-1 as c_long as c_ulong);
	
	#[inline(always)]
	pub fn memoryActiveInBytes(&self) -> c_ulong
	{
		unsafe { Judy1MemActive(self.0) }
	}
	
	#[inline(always)]
	pub fn memoryUsedInBytes(&self) -> c_ulong
	{
		unsafe { Judy1MemUsed(self.0) }
	}
	
	#[inline(always)]
	pub fn count(&self) -> c_ulong
	{
		self.countRange(0, Self::MaximumIndex)
	}
	
	#[inline(always)]
	pub fn countRange(&self, fromIndexInclusive: c_ulong, toIndexInclusive: c_ulong) -> c_ulong
	{
		unsafe { Judy1Count(self.0, fromIndexInclusive, toIndexInclusive, null_mut()) }
	}
	
	/// NOTE: nthIndex of 0 will only match the last index in a fully populated array
	/// NOTE: nthIndex of 1 will locate the first populated index
	#[inline(always)]
	pub fn locateNthIndex(&self, nthIndex: c_ulong) -> Option<c_ulong>
	{
		let mut index = unsafe { uninitialized() };
		let found = unsafe { Judy1ByCount(self.0, nthIndex, &mut index, null_mut()) };
		debug_assert!(found == 0 || found == 1, "found was '{}' instead of 0 or 1", found);
		if found == 1
		{
			Some(index)
		}
		else
		{
			None
		}
	}
	
	/// Returns true if not previously set
	#[inline(always)]
	pub fn insert(&mut self, index: c_ulong) -> bool
	{
		let successfullySet = unsafe { Judy1Set(&mut self.0, index, null_mut()) };
		debug_assert!(successfullySet == 0 || successfullySet == 1, "successfullySet was '{}' instead of 0 or 1", successfullySet);
		successfullySet == 1
	}
	
	/// Returns true if not previously set
	#[inline(always)]
	pub fn insertBulk(&mut self, indices: &[c_ulong]) -> bool
	{
		let successfullySet = unsafe { Judy1SetArray(&mut self.0, indices.len() as c_ulong, indices.as_ptr(), null_mut()) };
		debug_assert!(successfullySet == 0 || successfullySet == 1, "successfullySet was '{}' instead of 0 or 1", successfullySet);
		successfullySet == 1
	}
	
	/// Returns true if previously set
	#[inline(always)]
	pub fn remove(&mut self, index: c_ulong) -> bool
	{
		let successfullyUnset = unsafe { Judy1Unset(&mut self.0, index, null_mut()) };
		debug_assert!(successfullyUnset == 0 || successfullyUnset == 1, "successfullyUnset was '{}' instead of 0 or 1", successfullyUnset);
		successfullyUnset == 1
	}
	
	#[inline(always)]
	pub fn has(&self, index: c_ulong) -> bool
	{
		let has = unsafe { Judy1Test(self.0, index, null_mut()) };
		debug_assert!(has == 0 || has == 1, "has was '{}' instead of 0 or 1", has);
		has == 1
	}
	
	#[inline(always)]
	pub fn firstSetIndex(&self) -> Option<c_ulong>
	{
		self.firstSetIndexEqualToOrGreaterThan(0)
	}
	
	#[inline(always)]
	pub fn firstSetIndexEqualToOrGreaterThan(&self, mut index: c_ulong) -> Option<c_ulong>
	{
		let has = unsafe { Judy1First(self.0, &mut index, null_mut()) };
		debug_assert!(has == 0 || has == 1, "has was '{}' instead of 0 or 1", has);
		if has == 1
		{
			Some(index)
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub fn nextSetIndexGreaterThan(&self, mut index: c_ulong) -> Option<c_ulong>
	{
		let has = unsafe { Judy1Next(self.0, &mut index, null_mut()) };
		debug_assert!(has == 0 || has == 1, "has was '{}' instead of 0 or 1", has);
		if has == 1
		{
			Some(index)
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub fn lastSetIndex(&self) -> Option<c_ulong>
	{
		self.lastSetIndexEqualToOrLessThan(Self::MaximumIndex)
	}
	
	#[inline(always)]
	pub fn lastSetIndexEqualToOrLessThan(&self, mut index: c_ulong) -> Option<c_ulong>
	{
		let has = unsafe { Judy1Last(self.0, &mut index, null_mut()) };
		debug_assert!(has == 0 || has == 1, "has was '{}' instead of 0 or 1", has);
		if has == 1
		{
			Some(index)
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub fn previousSetIndexLessThan(&self, mut index: c_ulong) -> Option<c_ulong>
	{
		let has = unsafe { Judy1Prev(self.0, &mut index, null_mut()) };
		debug_assert!(has == 0 || has == 1, "has was '{}' instead of 0 or 1", has);
		if has == 1
		{
			Some(index)
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub fn firstUnsetIndex(&self) -> Option<c_ulong>
	{
		self.firstUnsetIndexEqualToOrGreaterThan(0)
	}
	
	#[inline(always)]
	pub fn firstUnsetIndexEqualToOrGreaterThan(&self, mut index: c_ulong) -> Option<c_ulong>
	{
		let has = unsafe { Judy1FirstEmpty(self.0, &mut index, null_mut()) };
		debug_assert!(has == 0 || has == 1, "has was '{}' instead of 0 or 1", has);
		if has == 1
		{
			Some(index)
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub fn nextUnsetIndexGreaterThan(&self, mut index: c_ulong) -> Option<c_ulong>
	{
		let has = unsafe { Judy1NextEmpty(self.0, &mut index, null_mut()) };
		debug_assert!(has == 0 || has == 1, "has was '{}' instead of 0 or 1", has);
		if has == 1
		{
			Some(index)
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub fn lastUnsetIndex(&self) -> Option<c_ulong>
	{
		self.lastUnsetIndexEqualToOrLessThan(Self::MaximumIndex)
	}
	
	#[inline(always)]
	pub fn lastUnsetIndexEqualToOrLessThan(&self, mut index: c_ulong) -> Option<c_ulong>
	{
		let has = unsafe { Judy1LastEmpty(self.0, &mut index, null_mut()) };
		debug_assert!(has == 0 || has == 1, "has was '{}' instead of 0 or 1", has);
		if has == 1
		{
			Some(index)
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub fn previousUnsetIndexLessThan(&self, mut index: c_ulong) -> Option<c_ulong>
	{
		let has = unsafe { Judy1PrevEmpty(self.0, &mut index, null_mut()) };
		debug_assert!(has == 0 || has == 1, "has was '{}' instead of 0 or 1", has);
		if has == 1
		{
			Some(index)
		}
		else
		{
			None
		}
	}
}
