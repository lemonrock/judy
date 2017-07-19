// This file is part of judy. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT. No part of judy-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of judy. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT.


#[derive(Debug)]
pub struct JudyL(Pvoid_t);

impl Default for JudyL
{
	#[inline(always)]
	fn default() -> Self
	{
		JudyL(null_mut())
	}
}

impl Drop for JudyL
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { JudyLFreeArray(&mut self.0, null_mut()) };
	}
}

impl JudyL
{
	pub const MaximumIndex: c_ulong = (-1 as c_long as c_ulong);
	
	#[inline(always)]
	pub fn memoryActiveInBytes(&self) -> c_ulong
	{
		unsafe { JudyLMemActive(self.0) }
	}
	
	#[inline(always)]
	pub fn memoryUsedInBytes(&self) -> c_ulong
	{
		unsafe { JudyLMemUsed(self.0) }
	}
	
	#[inline(always)]
	pub fn count(&self) -> c_ulong
	{
		self.countRange(0, Self::MaximumIndex)
	}
	
	#[inline(always)]
	pub fn countRange(&self, fromIndexInclusive: c_ulong, toIndexInclusive: c_ulong) -> c_ulong
	{
		unsafe { JudyLCount(self.0, fromIndexInclusive, toIndexInclusive, null_mut()) }
	}
	
	/// NOTE: nthIndex of 0 will only match the last index in a fully populated array
	/// NOTE: nthIndex of 1 will locate the first populated index
	#[inline(always)]
	pub fn locateNthIndex(&self, nthIndex: c_ulong) -> Option<*mut *mut c_void>
	{
		let mut index = unsafe { uninitialized() };
		let found = unsafe { JudyLByCount(self.0, nthIndex, &mut index, null_mut()) };
		if found.is_null()
		{
			None
		}
		else
		{
			Some(found)
		}
	}
	
	/// If not previously inserted, inserts a value, initializes it to zero (0) and returns a short-lived pointer to this value
	/// If previously inserted returns a short-lived pointer to a value
	#[inline(always)]
	pub fn insert(&mut self, index: c_ulong) -> *mut *mut c_void
	{
		let pointerToValue = unsafe { JudyLIns(&mut self.0, index, null_mut()) };
		pointerToValue
	}
	
	#[inline(always)]
	pub fn insertBulk(&mut self, indices: &[c_ulong]) -> Option<Vec<*mut *mut c_void>>
	{
		let mut result: Vec<*mut *mut c_void> = Vec::with_capacity(indices.len());
		
		let successfullySet = unsafe { JudyLInsArray(&mut self.0, indices.len() as c_ulong, indices.as_ptr(), result.as_ptr() as *const _, null_mut()) };
		debug_assert!(successfullySet == 0 || successfullySet == 1, "successfullySet was '{}' instead of 0 or 1", successfullySet);
		if successfullySet == 1
		{
			unsafe { result.set_len(indices.len())};
			Some(result)
		}
		else
		{
			None
		}
	}
	
	/// Returns true if previously set
	#[inline(always)]
	pub fn remove(&mut self, index: c_ulong) -> bool
	{
		let successfullyUnset = unsafe { JudyLDel(&mut self.0, index, null_mut()) };
		debug_assert!(successfullyUnset == 0 || successfullyUnset == 1, "successfullyUnset was '{}' instead of 0 or 1", successfullyUnset);
		successfullyUnset == 1
	}
	
	#[inline(always)]
	fn get(&self, index: c_ulong) -> *mut *mut c_void
	{
		unsafe { JudyLGet(self.0, index, null_mut()) }
	}
	
	#[inline(always)]
	pub fn firstSetIndex(&self) -> Option<(c_ulong, *mut *mut c_void)>
	{
		self.firstSetIndexEqualToOrGreaterThan(0)
	}
	
	#[inline(always)]
	pub fn firstSetIndexEqualToOrGreaterThan(&self, mut index: c_ulong) -> Option<(c_ulong, *mut *mut c_void)>
	{
		let value = unsafe { JudyLFirst(self.0, &mut index, null_mut()) };
		if value.is_null()
		{
			None
		}
		else
		{
			Some((index, value))
		}
	}
	
	#[inline(always)]
	pub fn nextSetIndexGreaterThan(&self, mut index: c_ulong) -> Option<(c_ulong, *mut *mut c_void)>
	{
		let value = unsafe { JudyLNext(self.0, &mut index, null_mut()) };
		if value.is_null()
		{
			None
		}
		else
		{
			Some((index, value))
		}
	}
	
	#[inline(always)]
	pub fn lastSetIndex(&self) -> Option<(c_ulong, *mut *mut c_void)>
	{
		self.lastSetIndexEqualToOrLessThan(Self::MaximumIndex)
	}
	
	#[inline(always)]
	pub fn lastSetIndexEqualToOrLessThan(&self, mut index: c_ulong) -> Option<(c_ulong, *mut *mut c_void)>
	{
		let value = unsafe { JudyLLast(self.0, &mut index, null_mut()) };
		if value.is_null()
		{
			None
		}
		else
		{
			Some((index, value))
		}
	}
	
	#[inline(always)]
	pub fn previousSetIndexLessThan(&self, mut index: c_ulong) -> Option<(c_ulong, *mut *mut c_void)>
	{
		let value = unsafe { JudyLPrev(self.0, &mut index, null_mut()) };
		if value.is_null()
		{
			None
		}
		else
		{
			Some((index, value))
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
		let has = unsafe { JudyLFirstEmpty(self.0, &mut index, null_mut()) };
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
		let has = unsafe { JudyLNextEmpty(self.0, &mut index, null_mut()) };
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
		let has = unsafe { JudyLLastEmpty(self.0, &mut index, null_mut()) };
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
		let has = unsafe { JudyLPrevEmpty(self.0, &mut index, null_mut()) };
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
