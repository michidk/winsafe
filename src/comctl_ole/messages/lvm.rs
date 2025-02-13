use crate::co;
use crate::comctl::decl::{LVITEM, LVITEMINDEX};
use crate::msg::WndMsg;
use crate::ole::decl::HrResult;
use crate::prelude::MsgSend;

/// [`LVM_SETITEMINDEXSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setitemindexstate)
/// message parameters.
///
/// Return type: `HrResult<()>`.
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]
pub struct SetItemIndexState<'a, 'b, 'c> {
	pub lvitemindex: &'a LVITEMINDEX,
	pub lvitem: &'c LVITEM<'b>,
}

unsafe impl<'a, 'b, 'c> MsgSend for SetItemIndexState<'a, 'b, 'c> {
	type RetType = HrResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match co::HRESULT(v as _) {
			co::HRESULT::S_OK => Ok(()),
			e => Err(e),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETITEMINDEXSTATE.into(),
			wparam: self.lvitemindex as *const _ as _,
			lparam: self.lvitem as *const _ as _,
		}
	}
}
