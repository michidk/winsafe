use crate::kernel::decl::HINSTANCE;

pub(crate) const CLR_DEFAULT: u32 = 0xff00_0000;
pub(crate) const GDT_ERROR: i32 = -1;
pub(crate) const HINST_COMMCTRL: HINSTANCE = HINSTANCE(-1 as _);
pub(crate) const L_MAX_URL_LENGTH: usize = 2048 + 32 + 4;
pub(crate) const MAX_LINKID_TEXT: usize = 48;
