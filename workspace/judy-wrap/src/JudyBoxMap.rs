// This file is part of judy. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT. No part of judy-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of judy. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT.


#[derive(Debug)]
pub struct JudyBoxMap<T>(JudyL, PhantomData<T>);

impl<T> Default for JudyBoxMap<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		JudyBoxMap(Default::default(), PhantomData)
	}
}

impl<T> Drop for JudyBoxMap<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let mut index = 0;
		let mut pointerToValue = unsafe { JudyLFirst((self.0).0, &mut index, null_mut()) };
		
		while !pointerToValue.is_null()
		{
			let mutableReference = unsafe { *pointerToValue } as *mut T;
			drop(unsafe { Box::from_raw(mutableReference) });
			
			pointerToValue = unsafe { JudyLNext((self.0).0, &mut index, null_mut()) };
		}
	}
}

impl<'a, T> IntoIterator for &'a JudyBoxMap<T>
{
    type Item = &'a T;
    
	type IntoIter = JudyBoxMapIterator<'a, T>;
	
	#[inline(always)]
    fn into_iter(self) -> Self::IntoIter
	{
		JudyBoxMapIterator::new(self)
	}
}

impl<T> JudyBoxMap<T>
{
	#[inline(always)]
	pub fn count(&self) -> c_ulong
	{
		self.0.count()
	}
	
	#[inline(always)]
	pub fn countRange(&self, fromIndexInclusive: c_ulong, toIndexInclusive: c_ulong) -> c_ulong
	{
		self.0.countRange(fromIndexInclusive, toIndexInclusive)
	}
	
	#[inline(always)]
	pub fn insert(&mut self, index: c_ulong, value: Box<T>) -> bool
	{
		let pointerToValue = self.0.insert(index);
		if (unsafe { *pointerToValue }).is_null()
		{
			unsafe { *pointerToValue = Box::into_raw(value) as *mut _ };
			true
		}
		else
		{
			drop(unsafe { Box::from_raw(*(pointerToValue as *mut *mut T)) });
			unsafe { *pointerToValue = Box::into_raw(value) as *mut _ };
			false
		}
	}
	
	#[inline(always)]
	pub fn insertAssumingNoPreviousValue(&mut self, index: c_ulong, value: Box<T>)
	{
		let pointerToValue = self.0.insert(index);
		unsafe { *pointerToValue = Box::into_raw(value) as *mut _ };
	}
	
	/// Returns true if previously set
	#[inline(always)]
	pub fn remove(&mut self, index: c_ulong) -> bool
	{
		let pointerToValue = self.0.get(index);
		if pointerToValue.is_null()
		{
			false
		}
		else
		{
			drop(unsafe { Box::from_raw(*(pointerToValue as *mut *mut T)) });
			unsafe { JudyLDel(&mut (self.0).0, index, null_mut()) };
			true
		}
	}
	
	#[inline(always)]
	pub fn get<'a>(&'a self, index: c_ulong) -> Option<&'a T>
	{
		let pointerToValue = self.0.get(index);
		if pointerToValue.is_null()
		{
			None
		}
		else
		{
			let mutableReference = unsafe { *pointerToValue } as *mut T;
			Some(unsafe { &*mutableReference })
		}
	}
	
	#[inline(always)]
	pub fn get_mut<'a>(&'a mut self, index: c_ulong) -> Option<&'a mut T>
	{
		let pointerToValue = self.0.get(index);
		if pointerToValue.is_null()
		{
			None
		}
		else
		{
			let mutableReference = unsafe { *pointerToValue } as *mut T;
			Some(unsafe { &mut *mutableReference })
		}
	}
	
	#[inline(always)]
	pub fn firstSetIndex<'a>(&'a self) -> Option<(c_ulong, &'a T)>
	{
		self.firstSetIndexEqualToOrGreaterThan(0)
	}
	
	#[inline(always)]
	pub fn firstSetIndexEqualToOrGreaterThan<'a>(&'a self, mut index: c_ulong) -> Option<(c_ulong, &'a T)>
	{
		let pointerToValue = unsafe { JudyLFirst((self.0).0, &mut index, null_mut()) };
		if pointerToValue.is_null()
		{
			None
		}
		else
		{
			let mutableReference = unsafe { *pointerToValue } as *mut T;
			Some((index, unsafe { &*mutableReference }))
		}
	}
	
	#[inline(always)]
	pub fn nextSetIndexGreaterThan<'a>(&'a self, mut index: c_ulong) -> Option<(c_ulong, &'a T)>
	{
		let pointerToValue = unsafe { JudyLNext((self.0).0, &mut index, null_mut()) };
		if pointerToValue.is_null()
		{
			None
		}
		else
		{
			let mutableReference = unsafe { *pointerToValue } as *mut T;
			Some((index, unsafe { &*mutableReference }))
		}
	}
	
	#[inline(always)]
	pub fn lastSetIndex<'a>(&'a self) -> Option<(c_ulong, &'a T)>
	{
		self.lastSetIndexEqualToOrLessThan(JudyL::MaximumIndex)
	}
	
	#[inline(always)]
	pub fn lastSetIndexEqualToOrLessThan<'a>(&'a self, mut index: c_ulong) -> Option<(c_ulong, &'a T)>
	{
		let pointerToValue = unsafe { JudyLLast((self.0).0, &mut index, null_mut()) };
		if pointerToValue.is_null()
		{
			None
		}
		else
		{
			let mutableReference = unsafe { *pointerToValue } as *mut T;
			Some((index, unsafe { &*mutableReference }))
		}
	}
	
	#[inline(always)]
	pub fn previousSetIndexLessThan<'a>(&'a self, mut index: c_ulong) -> Option<(c_ulong, &'a T)>
	{
		let pointerToValue = unsafe { JudyLPrev((self.0).0, &mut index, null_mut()) };
		if pointerToValue.is_null()
		{
			None
		}
		else
		{
			let mutableReference = unsafe { *pointerToValue } as *mut T;
			Some((index, unsafe { &*mutableReference }))
		}
	}
	
	#[inline(always)]
	pub fn firstUnsetIndex(&self) -> Option<c_ulong>
	{
		self.0.firstUnsetIndex()
	}
	
	#[inline(always)]
	pub fn firstUnsetIndexEqualToOrGreaterThan(&self, index: c_ulong) -> Option<c_ulong>
	{
		self.0.firstUnsetIndexEqualToOrGreaterThan(index)
	}
	
	#[inline(always)]
	pub fn nextUnsetIndexGreaterThan(&self, index: c_ulong) -> Option<c_ulong>
	{
		self.0.nextUnsetIndexGreaterThan(index)
	}
	
	#[inline(always)]
	pub fn lastUnsetIndex(&self) -> Option<c_ulong>
	{
		self.0.lastUnsetIndex()
	}
	
	#[inline(always)]
	pub fn lastUnsetIndexEqualToOrLessThan(&self, index: c_ulong) -> Option<c_ulong>
	{
		self.0.lastUnsetIndexEqualToOrLessThan(index)
	}
	
	#[inline(always)]
	pub fn previousUnsetIndexLessThan(&self, index: c_ulong) -> Option<c_ulong>
	{
		self.0.previousUnsetIndexLessThan(index)
	}
}
