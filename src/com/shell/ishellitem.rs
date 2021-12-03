#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::co;
use crate::com::funcs::CoTaskMemFree;
use crate::com::iunknown::{ComInterface, ComPtr, IUnknownT, IUnknownVT};
use crate::com::shell;
use crate::ffi::{HRES, PCVOID, PSTR, PVOID, shell32};
use crate::privs::ok_to_hrresult;
use crate::various::WString;

/// [`IShellItem`](crate::shell::IShellItem) virtual table.
#[repr(C)]
pub struct IShellItemVT {
	pub IUnknownVT: IUnknownVT,
	pub BindToHandler: fn(ComPtr, PVOID, PCVOID, PCVOID, *mut ComPtr) -> HRES,
	pub GetParent: fn(ComPtr, *mut ComPtr) -> HRES,
	pub GetDisplayName: fn(ComPtr, u32, *mut PSTR) -> HRES,
	pub GetAttributes: fn(ComPtr, u32, *mut u32) -> HRES,
	pub Compare: fn(ComPtr, PVOID, u32, *mut i32) -> HRES,
}

/// [`IShellItem`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitem)
/// COM interface over [`IShellItemVT`](crate::shell::vt::IShellItemVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IShellItem(ComPtr);

impl_iunknown!(IShellItem, 0x43826d1e, 0xe718, 0x42ee, 0xbc55, 0xa1e261c37bfe);
impl IShellItemT for IShellItem {}

impl IShellItem {
	/// Calls
	/// [`SHCreateItemFromParsingName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shcreateitemfromparsingname)
	/// function to create a new shell item, using the given folder or file path.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::shell;
	///
	/// let shi = shell::IShellItem::from_path("C:\\Temp\\test.txt")?;
	/// ```
	pub fn from_path(file_or_folder_path: &str) -> HrResult<IShellItem> {
		let mut ppv_queried = ComPtr::null();
		ok_to_hrresult(
			unsafe {
				shell32::SHCreateItemFromParsingName(
					WString::from_str(file_or_folder_path).as_ptr(),
					std::ptr::null_mut(),
					&Self::IID as *const _ as _,
					&mut ppv_queried as *mut _ as _,
				)
			},
		).map(|_| Self::from(ppv_queried))
	}
}

/// Exposes the [`IShellItem`](crate::shell::IShellItem) methods.
pub trait IShellItemT: IUnknownT {
	/// [`IShellItem::GetAttributes`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getattributes)
	/// method.
	fn GetAttributes(&self,
		sfgao_mask: shell::co::SFGAO) -> HrResult<shell::co::SFGAO>
	{
		let mut attrs = u32::default();
		match co::HRESULT(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IShellItemVT);
				(vt.GetAttributes)(self.ptr(), sfgao_mask.0, &mut attrs)
			},
		) {
			co::HRESULT::S_OK
			| co::HRESULT::S_FALSE => Ok(shell::co::SFGAO(attrs)),
			hr => Err(hr),
		}
	}

	/// [`IShellItem::GetDisplayName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getdisplayname)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::shell;
	///
	/// let shi = shell::ShellItem::from_path("C:\\Temp\\test.txt")?;
	/// let full_path = shi.GetDisplayName(shell::co::SIGDN::FILESYSPATH)?;
	/// println!("{}", full_path);
	/// ```
	fn GetDisplayName(&self, sigdn_name: shell::co::SIGDN) -> HrResult<String> {
		let mut pstr: *mut u16 = std::ptr::null_mut();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellItemVT);
			ok_to_hrresult(
				(vt.GetDisplayName)(self.ptr(), sigdn_name.0, &mut pstr),
			)
		}.map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr);
			name.to_string()
		})
	}

	/// [`IShellItem::GetParent`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getparent)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::shell;
	///
	/// let shi = shell::IShellItem::from_path("C:\\Temp\\test.txt")?;
	/// let parent_shi = shi.GetParent()?;
	/// let full_path = parent_shi.GetDisplayName(shell::co::SIGDN::FILESYSPATH)?;
	/// println!("{}", full_path);
	/// ```
	fn GetParent(&self) -> HrResult<IShellItem> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellItemVT);
			ok_to_hrresult((vt.GetParent)(self.ptr(), &mut ppv_queried))
		}.map(|_| IShellItem::from(ppv_queried))
	}
}
