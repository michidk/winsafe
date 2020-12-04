#![allow(non_snake_case)]

use crate::co;

const_type!(LANG, u16,
	"[`FormatMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
	`dwLanguageId`.");
impl LANG {
	/// [`MAKELANGID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-makelangid)
	/// macro.
	pub fn MAKELANGID(self, sublang: co::SUBLANG) -> u32 {
		((u16::from(sublang) << 10) | self.0) as u32
	}

	const_val!(NEUTRAL, 0x00);
	const_val!(INVARIANT, 0x7f);
	const_val!(AFRIKAANS, 0x36);
	const_val!(ALBANIAN, 0x1c);
	const_val!(ALSATIAN, 0x84);
	const_val!(AMHARIC, 0x5e);
	const_val!(ARABIC, 0x01);
	const_val!(ARMENIAN, 0x2b);
	const_val!(ASSAMESE, 0x4d);
	const_val!(AZERI, 0x2c);
	const_val!(AZERBAIJANI, 0x2c);
	const_val!(BANGLA, 0x45);
	const_val!(BASHKIR, 0x6d);
	const_val!(BASQUE, 0x2d);
	const_val!(BELARUSIAN, 0x23);
	const_val!(BENGALI, 0x45);
	const_val!(BRETON, 0x7e);
	const_val!(BOSNIAN, 0x1a);
	const_val!(BOSNIAN_NEUTRAL, 0x781a);
	const_val!(BULGARIAN, 0x02);
	const_val!(CATALAN, 0x03);
	const_val!(CENTRAL_KURDISH, 0x92);
	const_val!(CHEROKEE, 0x5c);
	const_val!(CHINESE, 0x04);
	const_val!(CHINESE_SIMPLIFIED, 0x04);
	const_val!(CHINESE_TRADITIONAL, 0x7c04);
	const_val!(CORSICAN, 0x83);
	const_val!(CROATIAN, 0x1a);
	const_val!(CZECH, 0x05);
	const_val!(DANISH, 0x06);
	const_val!(DARI, 0x8c);
	const_val!(DIVEHI, 0x65);
	const_val!(DUTCH, 0x13);
	const_val!(ENGLISH, 0x09);
	const_val!(ESTONIAN, 0x25);
	const_val!(FAEROESE, 0x38);
	const_val!(FARSI, 0x29);
	const_val!(FILIPINO, 0x64);
	const_val!(FINNISH, 0x0b);
	const_val!(FRENCH, 0x0c);
	const_val!(FRISIAN, 0x62);
	const_val!(FULAH, 0x67);
	const_val!(GALICIAN, 0x56);
	const_val!(GEORGIAN, 0x37);
	const_val!(GERMAN, 0x07);
	const_val!(GREEK, 0x08);
	const_val!(GREENLANDIC, 0x6f);
	const_val!(GUJARATI, 0x47);
	const_val!(HAUSA, 0x68);
	const_val!(HAWAIIAN, 0x75);
	const_val!(HEBREW, 0x0d);
	const_val!(HINDI, 0x39);
	const_val!(HUNGARIAN, 0x0e);
	const_val!(ICELANDIC, 0x0f);
	const_val!(IGBO, 0x70);
	const_val!(INDONESIAN, 0x21);
	const_val!(INUKTITUT, 0x5d);
	const_val!(IRISH, 0x3c);
	const_val!(ITALIAN, 0x10);
	const_val!(JAPANESE, 0x11);
	const_val!(KANNADA, 0x4b);
	const_val!(KASHMIRI, 0x60);
	const_val!(KAZAK, 0x3f);
	const_val!(KHMER, 0x53);
	const_val!(KICHE, 0x86);
	const_val!(KINYARWANDA, 0x87);
	const_val!(KONKANI, 0x57);
	const_val!(KOREAN, 0x12);
	const_val!(KYRGYZ, 0x40);
	const_val!(LAO, 0x54);
	const_val!(LATVIAN, 0x26);
	const_val!(LITHUANIAN, 0x27);
	const_val!(LOWER_SORBIAN, 0x2e);
	const_val!(LUXEMBOURGISH, 0x6e);
	const_val!(MACEDONIAN, 0x2f);
	const_val!(MALAY, 0x3e);
	const_val!(MALAYALAM, 0x4c);
	const_val!(MALTESE, 0x3a);
	const_val!(MANIPURI, 0x58);
	const_val!(MAORI, 0x81);
	const_val!(MAPUDUNGUN, 0x7a);
	const_val!(MARATHI, 0x4e);
	const_val!(MOHAWK, 0x7c);
	const_val!(MONGOLIAN, 0x50);
	const_val!(NEPALI, 0x61);
	const_val!(NORWEGIAN, 0x14);
	const_val!(OCCITAN, 0x82);
	const_val!(ODIA, 0x48);
	const_val!(ORIYA, 0x48);
	const_val!(PASHTO, 0x63);
	const_val!(PERSIAN, 0x29);
	const_val!(POLISH, 0x15);
	const_val!(PORTUGUESE, 0x16);
	const_val!(PULAR, 0x67);
	const_val!(PUNJABI, 0x46);
	const_val!(QUECHUA, 0x6b);
	const_val!(ROMANIAN, 0x18);
	const_val!(ROMANSH, 0x17);
	const_val!(RUSSIAN, 0x19);
	const_val!(SAKHA, 0x85);
	const_val!(SAMI, 0x3b);
	const_val!(SANSKRIT, 0x4f);
	const_val!(SCOTTISH_GAELIC, 0x91);
	const_val!(SERBIAN, 0x1a);
	const_val!(SERBIAN_NEUTRAL, 0x7c1a);
	const_val!(SINDHI, 0x59);
	const_val!(SINHALESE, 0x5b);
	const_val!(SLOVAK, 0x1b);
	const_val!(SLOVENIAN, 0x24);
	const_val!(SOTHO, 0x6c);
	const_val!(SPANISH, 0x0a);
	const_val!(SWAHILI, 0x41);
	const_val!(SWEDISH, 0x1d);
	const_val!(SYRIAC, 0x5a);
	const_val!(TAJIK, 0x28);
	const_val!(TAMAZIGHT, 0x5f);
	const_val!(TAMIL, 0x49);
	const_val!(TATAR, 0x44);
	const_val!(TELUGU, 0x4a);
	const_val!(THAI, 0x1e);
	const_val!(TIBETAN, 0x51);
	const_val!(TIGRIGNA, 0x73);
	const_val!(TIGRINYA, 0x73);
	const_val!(TSWANA, 0x32);
	const_val!(TURKISH, 0x1f);
	const_val!(TURKMEN, 0x42);
	const_val!(UIGHUR, 0x80);
	const_val!(UKRAINIAN, 0x22);
	const_val!(UPPER_SORBIAN, 0x2e);
	const_val!(URDU, 0x20);
	const_val!(UZBEK, 0x43);
	const_val!(VALENCIAN, 0x03);
	const_val!(VIETNAMESE, 0x2a);
	const_val!(WELSH, 0x52);
	const_val!(WOLOF, 0x88);
	const_val!(XHOSA, 0x34);
	const_val!(YAKUT, 0x85);
	const_val!(YI, 0x78);
	const_val!(YORUBA, 0x6a);
	const_val!(ZULU, 0x35);
}