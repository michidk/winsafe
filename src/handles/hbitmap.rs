#![allow(non_snake_case)]

use crate::handles::HGDIOBJ;

handle_type! {
	/// Handle to a
	/// [bitmap](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbitmap).
	/// Exposes methods.
	HBITMAP
}

convert_hgdiobj!(HBITMAP);