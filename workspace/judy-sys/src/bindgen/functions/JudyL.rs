// This file is part of judy. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT. No part of judy-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of judy. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT.


extern "C"
{
	pub fn JudyLByCount(PArray: Pcvoid_t, Count: Word_t, PIndex: *mut Word_t, PJError: PJError_t) -> PPvoid_t;
	pub fn JudyLCount(PArray: Pcvoid_t, Index1: Word_t, Index2: Word_t, PJError: PJError_t) -> Word_t;
	pub fn JudyLDel(PPArray: PPvoid_t, Index: Word_t, PJError: PJError_t) -> c_int;
	pub fn JudyLFirst(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> PPvoid_t;
	pub fn JudyLFirstEmpty(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> c_int;
	pub fn JudyLFreeArray(PPArray: PPvoid_t, PJError: PJError_t) -> Word_t;
	pub fn JudyLGet(PArray: Pcvoid_t, Index: Word_t, PJError: PJError_t) -> PPvoid_t;
	pub fn JudyLIns(PPArray: PPvoid_t, Index: Word_t, PJError: PJError_t) -> PPvoid_t;
	pub fn JudyLInsArray(PPArray: PPvoid_t, Count: Word_t, PIndex: *const Word_t, PValue: *const Word_t, PJError: PJError_t) -> c_int;
	pub fn JudyLLast(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> PPvoid_t;
	pub fn JudyLLastEmpty(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> c_int;
	pub fn JudyLMemActive(PArray: Pcvoid_t) -> Word_t;
	pub fn JudyLMemUsed(PArray: Pcvoid_t) -> Word_t;
	pub fn JudyLNext(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> PPvoid_t;
	pub fn JudyLNextEmpty(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> c_int;
	pub fn JudyLPrev(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> PPvoid_t;
	pub fn JudyLPrevEmpty(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> c_int;
}
