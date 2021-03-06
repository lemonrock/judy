// This file is part of judy. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT. No part of judy-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of judy. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct J_UDY_ERROR_STRUCT
{
	pub je_Errno: JU_Errno_t,
	pub je_ErrID: c_int,
	pub je_reserved: [Word_t; 4usize],
}

impl Clone for J_UDY_ERROR_STRUCT
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for J_UDY_ERROR_STRUCT
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
