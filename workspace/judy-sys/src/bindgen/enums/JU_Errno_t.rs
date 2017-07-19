// This file is part of judy. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT. No part of judy-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of judy. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JU_Errno_t
{
	JU_ERRNO_NONE = 0,
	JU_ERRNO_FULL = 1,
	JU_ERRNO_NOMEM = 2,
	JU_ERRNO_NULLPPARRAY = 3,
	JU_ERRNO_NONNULLPARRAY = 10,
	JU_ERRNO_NULLPINDEX = 4,
	JU_ERRNO_NULLPVALUE = 11,
	JU_ERRNO_NOTJUDY1 = 5,
	JU_ERRNO_NOTJUDYL = 6,
	JU_ERRNO_NOTJUDYSL = 7,
	JU_ERRNO_UNSORTED = 12,
	JU_ERRNO_OVERRUN = 8,
	JU_ERRNO_CORRUPT = 9,
}
