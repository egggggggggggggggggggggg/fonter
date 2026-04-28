use std::{
    borrow::Borrow,
    ops::{Deref, DerefMut},
};

#[repr(transparent)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Tag {
    inner: [u8; 4],
}
impl Tag {
    pub const fn from_bytes(bytes: [u8; 4]) -> Self {
        Tag { inner: bytes }
    }
}
impl From<[u8; 4]> for Tag {
    fn from(inner: [u8; 4]) -> Self {
        Tag { inner }
    }
}
impl From<u32> for Tag {
    fn from(value: u32) -> Self {
        Self {
            inner: value.to_be_bytes(),
        }
    }
}
impl Borrow<[u8; 4]> for Tag {
    fn borrow(&self) -> &[u8; 4] {
        &self.inner
    }
}
impl AsRef<[u8; 4]> for Tag {
    fn as_ref(&self) -> &[u8; 4] {
        &self.inner
    }
}
impl AsRef<[u8]> for Tag {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}
impl Tag {
    pub const fn is_valid_script(&self) -> bool {
        todo!()
    }
    pub const fn is_valid_feature(&self) -> bool {
        match &self.inner {
            feature::LIGA
            | feature::DFLT
            | feature::CLIG
            | feature::DLIG
            | feature::HNUM
            | feature::VNUM
            | feature::ONUM
            | feature::TNUM
            | feature::NUMR
            | feature::DNOM
            | feature::SWSH
            | feature::SALT
            | feature::SS01
            | feature::SS02
            | feature::SS03
            | feature::SS04
            | feature::SS05
            | feature::SS06
            | feature::SS07
            | feature::SS08
            | feature::SS09
            | feature::SS10
            | feature::CASE
            | feature::C2SC
            | feature::SMCP
            | feature::C2PC
            | feature::PCAP
            | feature::TBLK
            | feature::RALT
            | feature::RCLT
            | feature::ABRV
            | feature::INIT
            | feature::FIN
            | feature::MED
            | feature::PRES
            | feature::RCOM
            | feature::RPOS
            | feature::SWAP
            | feature::SMAR
            | feature::LOCL
            | feature::HIST
            | feature::ARAB
            | feature::GSUB => true,
            _ => false,
        }
    }
    pub fn is_valid_language(&self) -> bool {
        todo!()
    }
}

pub mod script {
    pub const ADLAM: &[u8; 4] = b"adlm";
    pub const AHOM: &[u8; 4] = b"ahom";
    pub const ANATOLIAN_HIEROGLYPHS: &[u8; 4] = b"hluw";
    pub const ARABIC: &[u8; 4] = b"arab";
    pub const ARMENIAN: &[u8; 4] = b"armn";
    pub const AVESTAN: &[u8; 4] = b"avst";
    pub const BALINESE: &[u8; 4] = b"bali";
    pub const BAMUM: &[u8; 4] = b"bamu";
    pub const BASSA_VAH: &[u8; 4] = b"bass";
    pub const BATAK: &[u8; 4] = b"batk";
    pub const BANGLA: &[u8; 4] = b"beng";
    pub const BANGLA_V2: &[u8; 4] = b"bng2";
    pub const BERIA_ERFE: &[u8; 4] = b"berf";
    pub const BHAIKSUKI: &[u8; 4] = b"bhks";
    pub const BOPOMOFO: &[u8; 4] = b"bopo";
    pub const BRAHMI: &[u8; 4] = b"brah";
    pub const BRAILLE: &[u8; 4] = b"brai";
    pub const BUGINESE: &[u8; 4] = b"bugi";
    pub const BUHID: &[u8; 4] = b"buhd";
    pub const BYZANTINE_MUSIC: &[u8; 4] = b"byzm";
    pub const CANADIAN_SYLLABICS: &[u8; 4] = b"cans";
    pub const CARIAN: &[u8; 4] = b"cari";
    pub const CAUCASIAN_ALBANIAN: &[u8; 4] = b"aghb";
    pub const CHAKMA: &[u8; 4] = b"cakm";
    pub const CHAM: &[u8; 4] = b"cham";
    pub const CHEROKEE: &[u8; 4] = b"cher";
    pub const CHORASMIAN: &[u8; 4] = b"chrs";
    pub const CJK_IDEOGRAPHIC: &[u8; 4] = b"hani";
    pub const COPTIC: &[u8; 4] = b"copt";
    pub const CYPRIOT_SYLLABARY: &[u8; 4] = b"cprt";
    pub const CYPRO_MINOAN: &[u8; 4] = b"cpmn";
    pub const CYRILLIC: &[u8; 4] = b"cyrl";
    pub const DEFAULT: &[u8; 4] = b"DFLT";
    pub const DESERET: &[u8; 4] = b"dsrt";
    pub const DEVANAGARI: &[u8; 4] = b"deva";
    pub const DEVANAGARI_V2: &[u8; 4] = b"dev2";
    pub const DIVES_AKURU: &[u8; 4] = b"diak";
    pub const DOGRA: &[u8; 4] = b"dogr";
    pub const DUPLOYAN: &[u8; 4] = b"dupl";
    pub const EGYPTIAN_HIEROGLYPHS: &[u8; 4] = b"egyp";
    pub const ELBASAN: &[u8; 4] = b"elba";
    pub const ELYMAIC: &[u8; 4] = b"elym";
    pub const ETHIOPIC: &[u8; 4] = b"ethi";
    pub const GARAY: &[u8; 4] = b"gara";
    pub const GEORGIAN: &[u8; 4] = b"geor";
    pub const GLAGOLITIC: &[u8; 4] = b"glag";
    pub const GOTHIC: &[u8; 4] = b"goth";
    pub const GRANTHA: &[u8; 4] = b"gran";
    pub const GREEK: &[u8; 4] = b"grek";
    pub const GUJARATI: &[u8; 4] = b"gujr";
    pub const GUJARATI_V2: &[u8; 4] = b"gjr2";
    pub const GUNJALA_GONDI: &[u8; 4] = b"gong";
    pub const GURMUKHI: &[u8; 4] = b"guru";
    pub const GURMUKHI_V2: &[u8; 4] = b"gur2";
    pub const GURUNG_KHEMA: &[u8; 4] = b"gukh";
    pub const HANGUL: &[u8; 4] = b"hang";
    pub const HANGUL_JAMO: &[u8; 4] = b"jamo";
    pub const HANIFI_ROHINGYA: &[u8; 4] = b"rohg";
    pub const HANUNOO: &[u8; 4] = b"hano";
    pub const HATRAN: &[u8; 4] = b"hatr";
    pub const HEBREW: &[u8; 4] = b"hebr";
    pub const KANA: &[u8; 4] = b"kana"; // Hiragana / Katakana
    pub const IMPERIAL_ARAMAIC: &[u8; 4] = b"armi";
    pub const INSCRIPTIONAL_PAHLAVI: &[u8; 4] = b"phli";
    pub const INSCRIPTIONAL_PARTHIAN: &[u8; 4] = b"prti";
    pub const JAVANESE: &[u8; 4] = b"java";
    pub const KAITHI: &[u8; 4] = b"kthi";
    pub const KANNADA: &[u8; 4] = b"knda";
    pub const KANNADA_V2: &[u8; 4] = b"knd2";
    pub const KAWI: &[u8; 4] = b"kawi";
    pub const KAYAH_LI: &[u8; 4] = b"kali";
    pub const KHAROSTHI: &[u8; 4] = b"khar";
    pub const KHITAN_SMALL_SCRIPT: &[u8; 4] = b"kits";
    pub const KHMER: &[u8; 4] = b"khmr";
    pub const KHOJKI: &[u8; 4] = b"khoj";
    pub const KHUDAWADI: &[u8; 4] = b"sind";
    pub const KIRAT_RAI: &[u8; 4] = b"krai";
    pub const LAO: &[u8; 4] = b"lao ";
    pub const LATIN: &[u8; 4] = b"latn";
    pub const LEPCHA: &[u8; 4] = b"lepc";
    pub const LIMBU: &[u8; 4] = b"limb";
    pub const LINEAR_A: &[u8; 4] = b"lina";
    pub const LINEAR_B: &[u8; 4] = b"linb";
    pub const LISU: &[u8; 4] = b"lisu";
    pub const LYCIAN: &[u8; 4] = b"lyci";
    pub const LYDIAN: &[u8; 4] = b"lydi";
    pub const MAHAJANI: &[u8; 4] = b"mahj";
    pub const MAKASAR: &[u8; 4] = b"maka";
    pub const MALAYALAM: &[u8; 4] = b"mlym";
    pub const MALAYALAM_V2: &[u8; 4] = b"mlm2";
    pub const MANDAIC: &[u8; 4] = b"mand";
    pub const MANICHAEAN: &[u8; 4] = b"mani";
    pub const MARCHEN: &[u8; 4] = b"marc";
    pub const MASARAM_GONDI: &[u8; 4] = b"gonm";
    pub const MATH: &[u8; 4] = b"math";
    pub const MEDEFAIDRIN: &[u8; 4] = b"medf";
    pub const MEITEI_MAYEK: &[u8; 4] = b"mtei";
    pub const MENDE_KIKAKUI: &[u8; 4] = b"mend";
    pub const MEROITIC_CURSIVE: &[u8; 4] = b"merc";
    pub const MEROITIC_HIEROGLYPHS: &[u8; 4] = b"mero";
    pub const MIAO: &[u8; 4] = b"plrd";
    pub const MODI: &[u8; 4] = b"modi";
    pub const MONGOLIAN: &[u8; 4] = b"mong";
    pub const MRO: &[u8; 4] = b"mroo";
    pub const MULTANI: &[u8; 4] = b"mult";
    pub const MUSICAL_SYMBOLS: &[u8; 4] = b"musc";
    pub const MYANMAR: &[u8; 4] = b"mymr";
    pub const MYANMAR_V2: &[u8; 4] = b"mym2";
    pub const NABATAEAN: &[u8; 4] = b"nbat";
    pub const NAG_MUNDARI: &[u8; 4] = b"nagm";
    pub const NANDINAGARI: &[u8; 4] = b"nand";
    pub const NEWA: &[u8; 4] = b"newa";
    pub const NEW_TAI_LUE: &[u8; 4] = b"talu";
    pub const NKO: &[u8; 4] = b"nko ";
    pub const NUSHU: &[u8; 4] = b"nshu";
    pub const HMNP: &[u8; 4] = b"hmnp";
    pub const ODIA: &[u8; 4] = b"orya";
    pub const ODIA_V2: &[u8; 4] = b"ory2";
    pub const OGHAM: &[u8; 4] = b"ogam";
    pub const OL_CHIKI: &[u8; 4] = b"olck";
    pub const OL_ONAL: &[u8; 4] = b"onao";
    pub const OLD_ITALIC: &[u8; 4] = b"ital";
    pub const OLD_HUNGARIAN: &[u8; 4] = b"hung";
    pub const OLD_NORTH_ARABIAN: &[u8; 4] = b"narb";
    pub const OLD_PERMIC: &[u8; 4] = b"perm";
    pub const OLD_PERSIAN: &[u8; 4] = b"xpeo";
    pub const OLD_SOGDIAN: &[u8; 4] = b"sogo";
    pub const OLD_SOUTH_ARABIAN: &[u8; 4] = b"sarb";
    pub const OLD_TURKIC: &[u8; 4] = b"orkh";
    pub const OLD_UYGHUR: &[u8; 4] = b"ougr";
    pub const OSAGE: &[u8; 4] = b"osge";
    pub const OSMANYA: &[u8; 4] = b"osma";
    pub const PAHAWH_HMONG: &[u8; 4] = b"hmng";
    pub const PALMYRENE: &[u8; 4] = b"palm";
    pub const PAU_CIN_HAU: &[u8; 4] = b"pauc";
    pub const PHAGS_PA: &[u8; 4] = b"phag";
    pub const PHOENICIAN: &[u8; 4] = b"phnx";
    pub const PSALTER_PAHLAVI: &[u8; 4] = b"phlp";
    pub const REJANG: &[u8; 4] = b"rjng";
    pub const RUNIC: &[u8; 4] = b"runr";
    pub const SAMARITAN: &[u8; 4] = b"samr";
    pub const SAURASHTRA: &[u8; 4] = b"saur";
    pub const SHARADA: &[u8; 4] = b"shrd";
    pub const SHAVIAN: &[u8; 4] = b"shaw";
    pub const SIDDHAM: &[u8; 4] = b"sidd";
    pub const SIDETIC: &[u8; 4] = b"sidt";
    pub const SIGNWRITING: &[u8; 4] = b"sgnw";
    pub const SINHALA: &[u8; 4] = b"sinh";
    pub const SOGDIAN: &[u8; 4] = b"sogd";
    pub const SORA_SOMPENG: &[u8; 4] = b"sora";
    pub const SOYOMBO: &[u8; 4] = b"soyo";
    pub const SUMERO_AKKADIAN: &[u8; 4] = b"xsux";
    pub const SUNDANESE: &[u8; 4] = b"sund";
    pub const SUNUWAR: &[u8; 4] = b"sunu";
    pub const SYLOTI_NAGRI: &[u8; 4] = b"sylo";
    pub const SYRIAC: &[u8; 4] = b"syrc";
    pub const TAGALOG: &[u8; 4] = b"tglg";
    pub const TAGBANWA: &[u8; 4] = b"tagb";
    pub const TAI_LE: &[u8; 4] = b"tale";
    pub const TAI_THAM: &[u8; 4] = b"lana";
    pub const TAI_VIET: &[u8; 4] = b"tavt";
    pub const TAI_YO: &[u8; 4] = b"tayo";
    pub const TAKRI: &[u8; 4] = b"takr";
    pub const TAMIL: &[u8; 4] = b"taml";
    pub const TAMIL_V2: &[u8; 4] = b"tml2";
    pub const TANGSA: &[u8; 4] = b"tnsa";
    pub const TANGUT: &[u8; 4] = b"tang";
    pub const TELUGU: &[u8; 4] = b"telu";
    pub const TELUGU_V2: &[u8; 4] = b"tel2";
    pub const THAANA: &[u8; 4] = b"thaa";
    pub const THAI: &[u8; 4] = b"thai";
    pub const TIBETAN: &[u8; 4] = b"tibt";
    pub const TIFINAGH: &[u8; 4] = b"tfng";
    pub const TIRHUTA: &[u8; 4] = b"tirh";
    pub const TODHRI: &[u8; 4] = b"todr";
    pub const TOLONG_SIKI: &[u8; 4] = b"tols";
    pub const TOTO: &[u8; 4] = b"toto";
    pub const TULU_TIGALARI: &[u8; 4] = b"tutg";
    pub const UGARITIC: &[u8; 4] = b"ugar";
    pub const VAI: &[u8; 4] = b"vai ";
    pub const VITHKUQI: &[u8; 4] = b"vith";
    pub const WANCHO: &[u8; 4] = b"wcho";
    pub const WARANG_CITI: &[u8; 4] = b"wara";
    pub const YEZIDI: &[u8; 4] = b"yezi";
    ///Two spaces inside here. Byte sequence = 0x79 0x69 0x20 0x20
    pub const YI: &[u8; 4] = b"yi  ";
    pub const ZANABAZAR_SQUARE: &[u8; 4] = b"zanb";
}

mod language {
    // Common OpenType language system tags (4 bytes)
    pub const DEFAULT: &[u8; 4] = b"dflt";
    pub const ENGLISH_US: &[u8; 4] = b"ENG ";
    pub const ENGLISH: &[u8; 4] = b"en  "; // conservative placeholder (not standard 4-byte)
    pub const FRENCH: &[u8; 4] = b"fr  ";
    pub const GERMAN: &[u8; 4] = b"de  ";
    pub const SPANISH: &[u8; 4] = b"es  ";
    pub const ITALIAN: &[u8; 4] = b"it  ";
    pub const PORTUGUESE: &[u8; 4] = b"pt  ";
    pub const RUSSIAN: &[u8; 4] = b"ru  ";
    pub const CHINESE_SIMPLIFIED: &[u8; 4] = b"zhcn";
    pub const CHINESE_TRADITIONAL: &[u8; 4] = b"zhtw";
    pub const JAPANESE: &[u8; 4] = b"ja  ";
    pub const KOREAN: &[u8; 4] = b"ko  ";
    pub const ARABIC: &[u8; 4] = b"ar  ";
    pub const HEBREW: &[u8; 4] = b"he  ";
    pub const HINDI: &[u8; 4] = b"hi  ";
    pub const BENGALI: &[u8; 4] = b"bn  ";
    pub const TAMIL: &[u8; 4] = b"ta  ";
    pub const TELUGU: &[u8; 4] = b"te  ";
    pub const PUNJABI: &[u8; 4] = b"pa  ";
    pub const VIETNAMESE: &[u8; 4] = b"vi  ";
    pub const THAI: &[u8; 4] = b"th  ";
    pub const LAO: &[u8; 4] = b"lo  ";
    pub const MYANMAR: &[u8; 4] = b"my  ";
    pub const HUNGARIAN: &[u8; 4] = b"hu  ";
    pub const POLISH: &[u8; 4] = b"pl  ";
    pub const DUTCH: &[u8; 4] = b"nl  ";
    pub const SWEDISH: &[u8; 4] = b"sv  ";
    pub const NORWEGIAN: &[u8; 4] = b"no  ";
    pub const DANISH: &[u8; 4] = b"da  ";
    pub const FINNISH: &[u8; 4] = b"fi  ";
    pub const CZECH: &[u8; 4] = b"cs  ";
    pub const SLOVAK: &[u8; 4] = b"sk  ";
    pub const ROMANIAN: &[u8; 4] = b"ro  ";
    pub const GREEK: &[u8; 4] = b"el  ";
    pub const TURKISH: &[u8; 4] = b"tr  ";
    pub const UKRAINIAN: &[u8; 4] = b"uk  ";
    // Add more language tags as needed
}
macro_rules! script_set {
    ($($name:ident),* $(,)?) => {
        pub const VALID_SCRIPTS: &[[u8; 4]] = &[
            $(*script::$name),*
        ];
    };
}
mod feature {
    // Common OpenType feature tags (4 bytes)
    pub const LIGA: &[u8; 4] = b"liga"; // standard ligatures
    pub const DFLT: &[u8; 4] = b"dflt";
    pub const CLIG: &[u8; 4] = b"clig"; // contextual ligatures
    pub const DLIG: &[u8; 4] = b"dlig"; // discretionary ligatures
    pub const HNUM: &[u8; 4] = b"hnum"; // tabular numbers
    pub const VNUM: &[u8; 4] = b"vnum"; // proportional numbers
    pub const ONUM: &[u8; 4] = b"onum"; // oldstyle figures
    pub const TNUM: &[u8; 4] = b"tnum"; // tabular figures
    pub const NUMR: &[u8; 4] = b"numr"; // numerator
    pub const DNOM: &[u8; 4] = b"dnom"; // denominator
    pub const SWSH: &[u8; 4] = b"swsh"; // swash
    pub const SALT: &[u8; 4] = b"salt"; // stylistic alternate
    pub const SS01: &[u8; 4] = b"ss01"; // stylistic set 1
    pub const SS02: &[u8; 4] = b"ss02";
    pub const SS03: &[u8; 4] = b"ss03";
    pub const SS04: &[u8; 4] = b"ss04";
    pub const SS05: &[u8; 4] = b"ss05";
    pub const SS06: &[u8; 4] = b"ss06";
    pub const SS07: &[u8; 4] = b"ss07";
    pub const SS08: &[u8; 4] = b"ss08";
    pub const SS09: &[u8; 4] = b"ss09";
    pub const SS10: &[u8; 4] = b"ss10";
    pub const CASE: &[u8; 4] = b"case"; // case-sensitive forms
    pub const C2SC: &[u8; 4] = b"c2sc"; // small caps from caps
    pub const SMCP: &[u8; 4] = b"smcp"; // small caps
    pub const C2PC: &[u8; 4] = b"c2pc"; // petite caps from caps
    pub const PCAP: &[u8; 4] = b"pcap"; // petite caps
    pub const TBLK: &[u8; 4] = b"tblk"; // traditional baseline
    pub const RALT: &[u8; 4] = b"ralt"; // required alternate
    pub const RCLT: &[u8; 4] = b"rclt"; // required contextual alternates
    pub const ABRV: &[u8; 4] = b"abrv"; // abbreviation
    pub const INIT: &[u8; 4] = b"init"; // initial forms
    pub const FIN: &[u8; 4] = b"fina"; // final forms
    pub const MED: &[u8; 4] = b"medi"; // medial forms
    pub const PRES: &[u8; 4] = b"pres"; // presentation forms
    pub const RCOM: &[u8; 4] = b"rcom"; // required composition
    pub const RPOS: &[u8; 4] = b"rpos";
    pub const SWAP: &[u8; 4] = b"swap";
    pub const SMAR: &[u8; 4] = b"smar";
    pub const LOCL: &[u8; 4] = b"locl"; // localized forms
    pub const HIST: &[u8; 4] = b"hist"; // historical forms
    pub const ARAB: &[u8; 4] = b"arab"; // Arabic features group (placeholder)
    pub const GSUB: &[u8; 4] = b"GSUB"; // table name (not a feature tag but sometimes used)
    // Add more feature tags as needed
}
