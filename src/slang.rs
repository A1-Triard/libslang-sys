use core::mem::{size_of};
use libc::{FILE, c_void, c_int, c_char, c_uint, c_ulong, c_uchar, c_short, c_long, c_ushort, c_float, c_double};

include!(concat!(env!("OUT_DIR"), "/version.rs"));

pub type VOID_STAR = *mut c_void;
pub type FVOID_STAR = Option<unsafe extern "C" fn() -> c_int>;
pub type SLstrlen_Type = c_uint;
pub type SLindex_Type = c_int;
pub type SLuindex_Type = c_uint;
pub type SLstr_Type = c_char;
pub type SLstr_Hash_Type = c_ulong;
pub type SLwchar_Type = u32;
pub type SLuchar_Type = c_uchar;
pub type SLtype = c_uint;
pub type SLclass_Type = c_int;
pub type SLang_MMT_Type = c_int;
pub type SLang_Ref_Type = c_int;
pub type SLtt_Char_Type = c_ulong;
pub type SLsmg_Color_Type = c_ushort;

pub const SLANG_SIZEOF_SHORT: usize = size_of::<c_short>();
pub const SLANG_SIZEOF_INT: usize = size_of::<c_int>();
pub const SLANG_SIZEOF_LONG: usize = size_of::<c_long>();
pub const SLANG_SIZEOF_FLOAT: usize = size_of::<c_float>();
pub const SLANG_SIZEOF_DOUBLE: usize = size_of::<c_double>();

pub const SLUTF8_MAX_MBLEN: u8 = 6;

pub const SLWCWIDTH_SINGLE_WIDTH: c_int = 1;
pub const SLWCWIDTH_CJK_LEGACY: c_int = 2;

pub const SLANG_LVARIABLE: c_uchar = 1;
pub const SLANG_GVARIABLE: c_uchar = 2;
pub const SLANG_IVARIABLE: c_uchar = 3;
pub const SLANG_RVARIABLE: c_uchar = 4;
pub const SLANG_INTRINSIC: c_uchar = 5;
pub const SLANG_FUNCTION: c_uchar = 6;
pub const SLANG_MATH_UNARY: c_uchar = 7;
pub const SLANG_APP_UNARY: c_uchar = 8;
pub const SLANG_ARITH_UNARY: c_uchar = 9;
pub const SLANG_ARITH_BINARY: c_uchar = 10;
pub const SLANG_ICONSTANT: c_uchar = 11;
pub const SLANG_DCONSTANT: c_uchar = 12;
pub const SLANG_FCONSTANT: c_uchar = 13;
pub const SLANG_LLCONSTANT: c_uchar = 14;
pub const SLANG_PVARIABLE: c_uchar = 15;
pub const SLANG_PFUNCTION: c_uchar = 16;
pub const SLANG_HCONSTANT: c_uchar = 17;
pub const SLANG_LCONSTANT: c_uchar = 18;

pub const SLANG_MAX_INTRIN_ARGS: usize = 7;

pub const SLANG_LOAD_FILE_VERBOSE: c_int = 1;
pub const SLANG_LOAD_MODULE_VERBOSE: c_int = 2;

pub const SLANG_CLASS_TYPE_MMT: SLclass_Type = 0;
pub const SLANG_CLASS_TYPE_SCALAR: SLclass_Type = 1;
pub const SLANG_CLASS_TYPE_VECTOR: SLclass_Type = 2;
pub const SLANG_CLASS_TYPE_PTR: SLclass_Type = 3;

pub const SLMATH_SIN: c_int = 1;
pub const SLMATH_COS: c_int = 2;
pub const SLMATH_TAN: c_int = 3;
pub const SLMATH_ATAN: c_int = 4;
pub const SLMATH_ASIN: c_int = 5;
pub const SLMATH_ACOS: c_int = 6;
pub const SLMATH_EXP: c_int = 7;
pub const SLMATH_LOG: c_int = 8;
pub const SLMATH_SQRT: c_int = 9;
pub const SLMATH_LOG10: c_int = 10;
pub const SLMATH_REAL: c_int = 11;
pub const SLMATH_IMAG: c_int = 12;
pub const SLMATH_SINH: c_int = 13;
pub const SLMATH_COSH: c_int = 14;
pub const SLMATH_TANH: c_int = 15;
pub const SLMATH_ATANH: c_int = 16;
pub const SLMATH_ASINH: c_int = 17;
pub const SLMATH_ACOSH: c_int = 18;
pub const SLMATH_TODOUBLE: c_int = 19;
pub const SLMATH_CONJ: c_int = 20;
pub const SLMATH_ISINF: c_int = 21;
pub const SLMATH_ISNAN: c_int = 22;
pub const SLMATH_FLOOR: c_int = 23;
pub const SLMATH_CEIL: c_int = 24;
pub const SLMATH_ROUND: c_int = 25;
pub const SLMATH_EXPM1: c_int = 26;
pub const SLMATH_LOG1P: c_int = 27;

pub const SLARRAY_MAX_DIMS: usize = 7;

pub const SLARR_DATA_VALUE_IS_READ_ONLY: c_uint = 1;
pub const SLARR_DATA_VALUE_IS_POINTER: c_uint = 2;
pub const SLARR_DATA_VALUE_IS_RANGE: c_uint = 4;
pub const SLARR_DATA_VALUE_IS_INTRINSIC: c_uint = 8;
pub const SLARR_DERIVED_FROM_SCALAR: c_uint = 256;

pub const SL_TB_NONE: c_int = 0;
pub const SL_TB_FULL: c_int = 1;
pub const SL_TB_OMIT_LOCALS: c_int = 2;
pub const SL_TB_PARTIAL: c_int = 4;

pub const SLANG_GETKEY_ERROR: c_uint = 65535;

pub const SLANG_MAX_KEYMAP_KEY_SEQ: usize = 14;

pub const SLKEY_F_INTERPRET: c_uchar = 1;
pub const SLKEY_F_INTRINSIC: c_uchar = 2;
pub const SLKEY_F_KEYSYM: c_uchar = 3;
pub const SLKEY_F_SLANG: c_uchar = 4;

pub const SL_RLINE_NO_ECHO: c_uint = 1;
pub const SL_RLINE_USE_ANSI: c_uint = 2;
pub const SL_RLINE_BLINK_MATCH: c_uint = 4;
pub const SL_RLINE_UTF8_MODE: c_uint = 8;
pub const SL_RLINE_USE_MULTILINE: c_uint = 16;

pub const SLTT_BOLD_MASK: SLtt_Char_Type = 16777216;
pub const SLTT_BLINK_MASK: SLtt_Char_Type = 33554432;
pub const SLTT_ULINE_MASK: SLtt_Char_Type = 67108864;
pub const SLTT_REV_MASK: SLtt_Char_Type = 134217728;
pub const SLTT_ALTC_MASK: SLtt_Char_Type = 268435456;
pub const SLTT_ITALIC_MASK: SLtt_Char_Type = 536870912;

pub const SLSMG_MAX_COLORS: SLsmg_Color_Type = 32766;
pub const SLSMG_COLOR_MASK: SLsmg_Color_Type = 32767;
pub const SLSMG_ACS_MASK: SLsmg_Color_Type = 32768;

pub const SLSMG_MAX_CHARS_PER_CELL: usize = 5;

pub const SLPREP_BLANK_LINES_OK: c_uint = 1;
pub const SLPREP_COMMENT_LINES_OK: c_uint = 2;

pub const SLSMG_NEWLINE_IGNORED: c_int = 0;
pub const SLSMG_NEWLINE_MOVES: c_int = 1;
pub const SLSMG_NEWLINE_SCROLLS: c_int = 2;
pub const SLSMG_NEWLINE_PRINTABLE: c_int = 3;

pub const SLSMG_COLOR_BLACK: SLsmg_Color_Type = 0;
pub const SLSMG_COLOR_RED: SLsmg_Color_Type = 1;
pub const SLSMG_COLOR_GREEN: SLsmg_Color_Type = 2;
pub const SLSMG_COLOR_BROWN: SLsmg_Color_Type = 3;
pub const SLSMG_COLOR_BLUE: SLsmg_Color_Type = 4;
pub const SLSMG_COLOR_MAGENTA: SLsmg_Color_Type = 5;
pub const SLSMG_COLOR_CYAN: SLsmg_Color_Type = 6;
pub const SLSMG_COLOR_LGRAY: SLsmg_Color_Type = 7;
pub const SLSMG_COLOR_GRAY: SLsmg_Color_Type = 8;
pub const SLSMG_COLOR_BRIGHT_RED: SLsmg_Color_Type = 9;
pub const SLSMG_COLOR_BRIGHT_GREEN: SLsmg_Color_Type = 10;
pub const SLSMG_COLOR_BRIGHT_BROWN: SLsmg_Color_Type = 11;
pub const SLSMG_COLOR_BRIGHT_BLUE: SLsmg_Color_Type = 12;
pub const SLSMG_COLOR_BRIGHT_MAGENTA: SLsmg_Color_Type = 13;
pub const SLSMG_COLOR_BRIGHT_CYAN: SLsmg_Color_Type = 14;
pub const SLSMG_COLOR_BRIGHT_WHITE: SLsmg_Color_Type = 15;

pub const SL_KEY_ERR: c_int = 65535;
pub const SL_KEY_UP: c_int = 257;
pub const SL_KEY_DOWN: c_int = 258;
pub const SL_KEY_LEFT: c_int = 259;
pub const SL_KEY_RIGHT: c_int = 260;
pub const SL_KEY_PPAGE: c_int = 261;
pub const SL_KEY_NPAGE: c_int = 262;
pub const SL_KEY_HOME: c_int = 263;
pub const SL_KEY_END: c_int = 264;
pub const SL_KEY_A1: c_int = 265;
pub const SL_KEY_A3: c_int = 266;
pub const SL_KEY_B2: c_int = 267;
pub const SL_KEY_C1: c_int = 268;
pub const SL_KEY_C3: c_int = 269;
pub const SL_KEY_REDO: c_int = 270;
pub const SL_KEY_UNDO: c_int = 271;
pub const SL_KEY_BACKSPACE: c_int = 272;
pub const SL_KEY_ENTER: c_int = 273;
pub const SL_KEY_IC: c_int = 274;
pub const SL_KEY_DELETE: c_int = 275;
pub const SL_KEY_F0: c_int = 512;
pub const fn SL_KEY_F(n: c_int) -> c_int { SL_KEY_F0 + n }

pub const SL_FE_DIVBYZERO: c_uint = 1;
pub const SL_FE_INVALID: c_uint = 2;
pub const SL_FE_OVERFLOW: c_uint = 4;
pub const SL_FE_UNDERFLOW: c_uint = 8;
pub const SL_FE_INEXACT: c_uint = 16;
pub const SL_FE_ALLEXCEPT: c_uint = 31;

pub const SLANG_UNDEFINED_TYPE: SLtype = 0;
pub const SLANG_VOID_TYPE: SLtype = 1;
pub const SLANG_NULL_TYPE: SLtype = 2;
pub const SLANG_ANY_TYPE: SLtype = 3;
pub const SLANG_DATATYPE_TYPE: SLtype = 4;
pub const SLANG_REF_TYPE: SLtype = 5;
pub const SLANG_STRING_TYPE: SLtype = 6;
pub const SLANG_BSTRING_TYPE: SLtype = 7;
pub const SLANG_FILE_PTR_TYPE: SLtype = 8;
pub const SLANG_FILE_FD_TYPE: SLtype = 9;
pub const SLANG_MD5_TYPE: SLtype = 10;
pub const SLANG_INTP_TYPE: SLtype = 15;
pub const SLANG_CHAR_TYPE: SLtype = 16;
pub const SLANG_UCHAR_TYPE: SLtype = 17;
pub const SLANG_SHORT_TYPE: SLtype = 18;
pub const SLANG_USHORT_TYPE: SLtype = 19;
pub const SLANG_INT_TYPE: SLtype = 20;
pub const SLANG_UINT_TYPE: SLtype = 21;
pub const SLANG_LONG_TYPE: SLtype = 22;
pub const SLANG_ULONG_TYPE: SLtype = 23;
pub const SLANG_LLONG_TYPE: SLtype = 24;
pub const SLANG_ULLONG_TYPE: SLtype = 25;
pub const SLANG_FLOAT_TYPE: SLtype = 26;
pub const SLANG_DOUBLE_TYPE: SLtype = 27;
pub const SLANG_LDOUBLE_TYPE: SLtype = 28;
pub const SLANG_COMPLEX_TYPE: SLtype = 32;
pub const SLANG_ISTRUCT_TYPE: SLtype = 42;
pub const SLANG_STRUCT_TYPE: SLtype = 43;
pub const SLANG_ASSOC_TYPE: SLtype = 44;
pub const SLANG_ARRAY_TYPE: SLtype = 45;
pub const SLANG_LIST_TYPE: SLtype = 46;
pub const SLANG_MIN_UNUSED_TYPE: SLtype = 48;

pub const VOID_TYPE: SLtype = SLANG_VOID_TYPE;
pub const INT_TYPE: SLtype = SLANG_INT_TYPE;
pub const INTP_TYPE: SLtype = SLANG_INTP_TYPE;
pub const FLOAT_TYPE: SLtype = SLANG_FLOAT_TYPE;
pub const ARRAY_TYPE: SLtype = SLANG_ARRAY_TYPE;
pub const CHAR_TYPE: SLtype = SLANG_CHAR_TYPE;
pub const STRING_TYPE: SLtype = SLANG_STRING_TYPE;

pub const SLANG_BINARY_OP_MIN: c_uint = 1;
pub const SLANG_PLUS: c_uint = 1;
pub const SLANG_MINUS: c_uint = 2;
pub const SLANG_TIMES: c_uint = 3;
pub const SLANG_DIVIDE: c_uint = 4;
pub const SLANG_EQ: c_uint = 5;
pub const SLANG_NE: c_uint = 6;
pub const SLANG_GT: c_uint = 7;
pub const SLANG_GE: c_uint = 8;
pub const SLANG_LT: c_uint = 9;
pub const SLANG_LE: c_uint = 10;
pub const SLANG_POW: c_uint = 11;
pub const SLANG_OR: c_uint = 12;
pub const SLANG_AND: c_uint = 13;
pub const SLANG_BAND: c_uint = 14;
pub const SLANG_BOR: c_uint = 15;
pub const SLANG_BXOR: c_uint = 16;
pub const SLANG_SHL: c_uint = 17;
pub const SLANG_SHR: c_uint = 18;
pub const SLANG_MOD: c_uint = 19;
pub const SLANG_BINARY_OP_MAX: c_uint = 19;
pub const SLANG_UNARY_OP_MIN: c_uint = 32;
pub const SLANG_PLUSPLUS: c_uint = 32;
pub const SLANG_MINUSMINUS: c_uint = 33;
pub const SLANG_CHS: c_uint = 34;
pub const SLANG_NOT: c_uint = 35;
pub const SLANG_BNOT: c_uint = 36;
pub const SLANG_ABS: c_uint = 37;
pub const SLANG_SIGN: c_uint = 38;
pub const SLANG_SQR: c_uint = 39;
pub const SLANG_MUL2: c_uint = 40;
pub const SLANG_ISPOS: c_uint = 41;
pub const SLANG_ISNEG: c_uint = 42;
pub const SLANG_ISNONNEG: c_uint = 43;
pub const SLANG_UNARY_OP_MAX: c_uint = 43;

pub const SLREGEXP_CASELESS: c_uint = 1;
pub const SLREGEXP_UTF8: c_uint = 16;

pub const SLREGEXP_HINT_BOL: c_uint = 1;
pub const SLREGEXP_HINT_OSEARCH: c_uint = 2;

pub const SLSEARCH_CASELESS: c_int = 1;
pub const SLSEARCH_UTF8: c_int = 2;

extern "C" {
    pub fn SLdebug_malloc(arg1: c_ulong) -> *mut c_char;
}
extern "C" {
    pub fn SLdebug_calloc(
        arg1: c_ulong,
        arg2: c_ulong,
    ) -> *mut c_char;
}
extern "C" {
    pub fn SLdebug_realloc(
        arg1: *mut c_char,
        arg2: c_ulong,
    ) -> *mut c_char;
}
extern "C" {
    pub fn SLdebug_free(arg1: *mut c_char);
}
extern "C" {
    pub fn SLmalloc_dump_statistics();
}
extern "C" {
    pub fn SLstrcpy(
        arg1: *mut c_char,
        arg2: *mut c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn SLstrcmp(
        arg1: *mut c_char,
        arg2: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLstrncpy(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn SLmemset(
        arg1: *mut c_char,
        arg2: c_char,
        arg3: c_int,
    );
}
extern "C" {
    pub fn SLmemchr(
        arg1: *mut c_char,
        arg2: c_char,
        arg3: c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn SLmemcpy(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn SLmemcmp(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_pop_strlen_type(arg1: *mut SLstrlen_Type) -> c_int;
}
extern "C" {
    pub fn SLang_push_strlen_type(arg1: SLstrlen_Type) -> c_int;
}
extern "C" {
    pub fn SLang_create_nslstring(
        arg1: *mut c_char,
        arg2: SLstrlen_Type,
    ) -> *mut SLstr_Type;
}
extern "C" {
    pub fn SLang_create_slstring(arg1: *mut c_char) -> *mut SLstr_Type;
}
extern "C" {
    pub fn SLang_free_slstring(arg1: *const SLstr_Type);
}
extern "C" {
    pub fn SLang_pop_slstring(arg1: *mut *mut SLstr_Type) -> c_int;
}
extern "C" {
    pub fn SLang_concat_slstrings(a: *mut SLstr_Type, b: *mut SLstr_Type) -> *mut SLstr_Type;
}
extern "C" {
    pub fn SLcompute_string_hash(arg1: *const SLstr_Type) -> SLstr_Hash_Type;
}
extern "C" {
    pub fn SLutf8_enable(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn SLutf8_is_utf8_mode() -> c_int;
}
extern "C" {
    pub fn SLtt_utf8_enable(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn SLtt_is_utf8_mode() -> c_int;
}
extern "C" {
    pub fn SLsmg_utf8_enable(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn SLsmg_is_utf8_mode() -> c_int;
}
extern "C" {
    pub fn SLinterp_utf8_enable(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn SLinterp_is_utf8_mode() -> c_int;
}
extern "C" {
    pub fn SLwchar_toupper(arg1: SLwchar_Type) -> SLwchar_Type;
}
extern "C" {
    pub fn SLwchar_tolower(arg1: SLwchar_Type) -> SLwchar_Type;
}
extern "C" {
    pub fn SLwchar_wcwidth(arg1: SLwchar_Type) -> c_int;
}
extern "C" {
    pub fn SLwchar_isalnum(arg1: SLwchar_Type) -> c_int;
}
extern "C" {
    pub fn SLwchar_isalpha(arg1: SLwchar_Type) -> c_int;
}
extern "C" {
    pub fn SLwchar_isblank(arg1: SLwchar_Type) -> c_int;
}
extern "C" {
    pub fn SLwchar_iscntrl(arg1: SLwchar_Type) -> c_int;
}
extern "C" {
    pub fn SLwchar_isdigit(arg1: SLwchar_Type) -> c_int;
}
extern "C" {
    pub fn SLwchar_isgraph(arg1: SLwchar_Type) -> c_int;
}
extern "C" {
    pub fn SLwchar_islower(arg1: SLwchar_Type) -> c_int;
}
extern "C" {
    pub fn SLwchar_isprint(arg1: SLwchar_Type) -> c_int;
}
extern "C" {
    pub fn SLwchar_ispunct(arg1: SLwchar_Type) -> c_int;
}
extern "C" {
    pub fn SLwchar_isspace(arg1: SLwchar_Type) -> c_int;
}
extern "C" {
    pub fn SLwchar_isupper(arg1: SLwchar_Type) -> c_int;
}
extern "C" {
    pub fn SLwchar_isxdigit(arg1: SLwchar_Type) -> c_int;
}
extern "C" {
    pub fn SLwchar_set_wcwidth_flags(flags: c_int) -> c_int;
}
extern "C" {
    pub fn SLutf8_skip_char(u: *mut SLuchar_Type, umax: *mut SLuchar_Type) -> *mut SLuchar_Type;
}
extern "C" {
    pub fn SLutf8_bskip_char(umin: *mut SLuchar_Type, u: *mut SLuchar_Type) -> *mut SLuchar_Type;
}
extern "C" {
    pub fn SLutf8_skip_chars(
        u: *mut SLuchar_Type,
        umax: *mut SLuchar_Type,
        num: SLstrlen_Type,
        dnum: *mut SLstrlen_Type,
        ignore_combining: c_int,
    ) -> *mut SLuchar_Type;
}
extern "C" {
    pub fn SLutf8_bskip_chars(
        umin: *mut SLuchar_Type,
        u: *mut SLuchar_Type,
        num: SLstrlen_Type,
        dnum: *mut SLstrlen_Type,
        ignore_combining: c_int,
    ) -> *mut SLuchar_Type;
}
extern "C" {
    pub fn SLutf8_strup(u: *mut SLuchar_Type, umax: *mut SLuchar_Type) -> *mut SLuchar_Type;
}
extern "C" {
    pub fn SLutf8_strlo(u: *mut SLuchar_Type, umax: *mut SLuchar_Type) -> *mut SLuchar_Type;
}
extern "C" {
    pub fn SLutf8_subst_wchar(
        u: *mut SLuchar_Type,
        umax: *mut SLuchar_Type,
        wch: SLwchar_Type,
        pos: SLstrlen_Type,
        ignore_combining: c_int,
    ) -> *mut SLstr_Type;
}
extern "C" {
    pub fn SLutf8_strlen(
        s: *mut SLuchar_Type,
        ignore_combining: c_int,
    ) -> SLstrlen_Type;
}
extern "C" {
    pub fn SLutf8_decode(
        u: *mut SLuchar_Type,
        umax: *mut SLuchar_Type,
        w: *mut SLwchar_Type,
        nconsumedp: *mut SLstrlen_Type,
    ) -> *mut SLuchar_Type;
}
extern "C" {
    pub fn SLutf8_encode(
        w: SLwchar_Type,
        u: *mut SLuchar_Type,
        ulen: SLstrlen_Type,
    ) -> *mut SLuchar_Type;
}
extern "C" {
    pub fn SLutf8_compare(
        a: *mut SLuchar_Type,
        amax: *mut SLuchar_Type,
        b: *mut SLuchar_Type,
        bmax: *mut SLuchar_Type,
        nchars: SLstrlen_Type,
        case_sensitive: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLutf8_extract_utf8_char(
        u: *mut SLuchar_Type,
        umax: *mut SLuchar_Type,
        buf: *mut SLuchar_Type,
    ) -> *mut SLuchar_Type;
}
extern "C" {
    pub fn SLutf8_encode_null_terminate(
        w: SLwchar_Type,
        buf: *mut SLuchar_Type,
    ) -> *mut SLuchar_Type;
}
extern {
    pub type SLwchar_Lut_Type;
}
extern "C" {
    pub fn SLwchar_create_lut(num_entries: c_uint) -> *mut SLwchar_Lut_Type;
}
extern "C" {
    pub fn SLwchar_add_range_to_lut(
        r: *mut SLwchar_Lut_Type,
        a: SLwchar_Type,
        b: SLwchar_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLwchar_skip_range(
        r: *mut SLwchar_Lut_Type,
        p: *mut SLuchar_Type,
        pmax: *mut SLuchar_Type,
        ignore_combining: c_int,
        invert: c_int,
    ) -> *mut SLuchar_Type;
}
extern "C" {
    pub fn SLwchar_strtolut(
        u: *mut SLuchar_Type,
        allow_range: c_int,
        allow_charclass: c_int,
    ) -> *mut SLwchar_Lut_Type;
}
extern "C" {
    pub fn SLwchar_free_lut(r: *mut SLwchar_Lut_Type);
}
extern "C" {
    pub fn SLwchar_bskip_range(
        r: *mut SLwchar_Lut_Type,
        pmin: *mut SLuchar_Type,
        p: *mut SLuchar_Type,
        ignore_combining: c_int,
        invert: c_int,
    ) -> *mut SLuchar_Type;
}
extern "C" {
    pub fn SLwchar_in_lut(r: *mut SLwchar_Lut_Type, wch: SLwchar_Type) -> c_int;
}
extern {
    pub type SLwchar_Map_Type;
}
extern "C" {
    pub fn SLwchar_free_char_map(map: *mut SLwchar_Map_Type);
}
extern "C" {
    pub fn SLwchar_allocate_char_map(
        from: *mut SLuchar_Type,
        to: *mut SLuchar_Type,
    ) -> *mut SLwchar_Map_Type;
}
extern "C" {
    pub fn SLwchar_apply_char_map(
        map: *mut SLwchar_Map_Type,
        input: *mut SLwchar_Type,
        output: *mut SLwchar_Type,
        num: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLuchar_apply_char_map(
        map: *mut SLwchar_Map_Type,
        str_: *mut SLuchar_Type,
    ) -> *mut SLuchar_Type;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLang_Name_Type {
    pub name: *mut c_char,
    pub next: *mut SLang_Name_Type,
    pub name_type: c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLang_Intrin_Fun_Type {
    pub name: *mut c_char,
    pub next: *mut SLang_Name_Type,
    pub name_type: c_char,
    pub i_fun: FVOID_STAR,
    pub arg_types: [SLtype; SLANG_MAX_INTRIN_ARGS],
    pub num_args: c_uchar,
    pub return_type: SLtype,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLang_Intrin_Var_Type {
    pub name: *mut c_char,
    pub next: *mut SLang_Name_Type,
    pub name_type: c_char,
    pub addr: VOID_STAR,
    pub type_: SLtype,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLang_App_Unary_Type {
    pub name: *mut c_char,
    pub next: *mut SLang_Name_Type,
    pub name_type: c_char,
    pub unary_op: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLang_Math_Unary_Type {
    pub name: *mut c_char,
    pub next: *mut SLang_Name_Type,
    pub name_type: c_char,
    pub unary_op: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLang_HConstant_Type {
    pub name: *mut c_char,
    pub next: *mut SLang_Name_Type,
    pub name_type: c_char,
    pub data_type: SLtype,
    pub value: c_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLang_IConstant_Type {
    pub name: *mut c_char,
    pub next: *mut SLang_Name_Type,
    pub name_type: c_char,
    pub data_type: SLtype,
    pub value: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLang_LConstant_Type {
    pub name: *mut c_char,
    pub next: *mut SLang_Name_Type,
    pub name_type: c_char,
    pub data_type: SLtype,
    pub value: c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLang_DConstant_Type {
    pub name: *mut c_char,
    pub next: *mut SLang_Name_Type,
    pub name_type: c_char,
    pub d: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLang_FConstant_Type {
    pub name: *mut c_char,
    pub next: *mut SLang_Name_Type,
    pub name_type: c_char,
    pub f: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLang_IStruct_Field_Type {
    pub field_name: *mut c_char,
    pub offset: c_uint,
    pub type_: SLtype,
    pub read_only: c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLang_CStruct_Field_Type {
    pub field_name: *mut c_char,
    pub offset: c_uint,
    pub type_: SLtype,
    pub read_only: c_uchar,
}
extern "C" {
    pub fn SLadd_intrin_fun_table(
        arg1: *mut SLang_Intrin_Fun_Type,
        arg2: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLadd_intrin_var_table(
        arg1: *mut SLang_Intrin_Var_Type,
        arg2: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLadd_app_unary_table(
        arg1: *mut SLang_App_Unary_Type,
        arg2: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLadd_math_unary_table(
        arg1: *mut SLang_Math_Unary_Type,
        arg2: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLadd_iconstant_table(
        arg1: *mut SLang_IConstant_Type,
        arg2: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLadd_lconstant_table(
        arg1: *mut SLang_LConstant_Type,
        pp: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLadd_dconstant_table(
        arg1: *mut SLang_DConstant_Type,
        arg2: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLadd_fconstant_table(
        arg1: *mut SLang_FConstant_Type,
        arg2: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLadd_istruct_table(
        arg1: *mut SLang_IStruct_Field_Type,
        arg2: VOID_STAR,
        arg3: *mut c_char,
    ) -> c_int;
}

extern {
    pub type SLang_NameSpace_Type;
}

extern "C" {
    pub fn SLns_add_intrin_fun_table(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut SLang_Intrin_Fun_Type,
        arg3: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_add_intrin_var_table(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut SLang_Intrin_Var_Type,
        arg3: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_add_app_unary_table(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut SLang_App_Unary_Type,
        arg3: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_add_math_unary_table(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut SLang_Math_Unary_Type,
        arg3: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_add_hconstant_table(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut SLang_HConstant_Type,
        arg3: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_add_iconstant_table(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut SLang_IConstant_Type,
        arg3: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_add_lconstant_table(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut SLang_LConstant_Type,
        arg3: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_add_dconstant_table(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut SLang_DConstant_Type,
        arg3: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_add_istruct_table(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut SLang_IStruct_Field_Type,
        arg3: VOID_STAR,
        arg4: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_add_hconstant(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut c_char,
        arg3: SLtype,
        arg4: c_short,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_add_iconstant(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut c_char,
        arg3: SLtype,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_add_lconstant(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut c_char,
        arg3: SLtype,
        arg4: c_long,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_add_fconstant(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut c_char,
        arg3: f32,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_add_dconstant(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut c_char,
        arg3: f64,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_create_namespace(arg1: *mut c_char) -> *mut SLang_NameSpace_Type;
}
extern "C" {
    pub fn SLns_delete_namespace(arg1: *mut SLang_NameSpace_Type);
}
extern "C" {
    pub fn SLns_load_file(
        arg1: *mut c_char,
        arg2: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_load_string(
        arg1: *mut c_char,
        arg2: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub static mut SLns_Load_File_Hook: Option<
        unsafe extern "C" fn(
            arg1: *mut c_char,
            arg2: *mut c_char,
        ) -> c_int,
    >;
}
extern "C" {
    pub fn SLang_load_file_verbose(arg1: c_int) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLang_Load_Type {
    pub type_: c_int,
    pub client_data: VOID_STAR,
    pub auto_declare_globals: c_int,
    pub read: Option<
        unsafe extern "C" fn(arg1: *mut SLang_Load_Type) -> *mut c_char,
    >,
    pub line_num: c_uint,
    pub parse_level: c_int,
    pub name: *mut c_char,
    pub namespace_name: *mut c_char,
    pub reserved: [c_ulong; 3usize],
}
extern "C" {
    pub fn SLallocate_load_type(arg1: *mut c_char) -> *mut SLang_Load_Type;
}
extern "C" {
    pub fn SLdeallocate_load_type(arg1: *mut SLang_Load_Type);
}
extern "C" {
    pub fn SLns_allocate_load_type(
        arg1: *mut c_char,
        arg2: *mut c_char,
    ) -> *mut SLang_Load_Type;
}
extern "C" {
    pub fn SLang_load_object(arg1: *mut SLang_Load_Type) -> c_int;
}
extern "C" {
    pub static mut SLang_Load_File_Hook: Option<
        unsafe extern "C" fn(arg1: *mut c_char) -> c_int,
    >;
}
extern "C" {
    pub static mut SLang_Auto_Declare_Var_Hook: Option<
        unsafe extern "C" fn(arg1: *mut c_char) -> c_int,
    >;
}
extern "C" {
    pub fn SLang_generate_debug_info(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn SLang_pop_array_index(arg1: *mut SLindex_Type) -> c_int;
}
extern "C" {
    pub fn SLang_push_array_index(arg1: SLindex_Type) -> c_int;
}
extern {
    pub type SLang_Struct_Type;
}
extern "C" {
    pub fn SLang_free_struct(arg1: *mut SLang_Struct_Type);
}
extern "C" {
    pub fn SLang_push_struct(arg1: *mut SLang_Struct_Type) -> c_int;
}
extern "C" {
    pub fn SLang_pop_struct(arg1: *mut *mut SLang_Struct_Type) -> c_int;
}
extern "C" {
    pub fn SLang_create_struct(
        field_names: *mut *mut c_char,
        nfields: c_uint,
    ) -> *mut SLang_Struct_Type;
}
extern "C" {
    pub fn SLang_push_struct_field(
        s: *mut SLang_Struct_Type,
        name: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_pop_struct_field(
        s: *mut SLang_Struct_Type,
        name: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_pop_struct_fields(
        s: *mut SLang_Struct_Type,
        n: c_int,
    ) -> c_int;
}
extern {
    pub type SLang_Assoc_Array_Type;
}
extern "C" {
    pub fn SLang_create_assoc(
        type_: SLtype,
        has_default: c_int,
    ) -> *mut SLang_Assoc_Array_Type;
}
extern "C" {
    pub fn SLang_free_assoc(arg1: *mut SLang_Assoc_Array_Type);
}
extern "C" {
    pub fn SLang_push_assoc(
        arg1: *mut SLang_Assoc_Array_Type,
        free_flag: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_pop_assoc(arg1: *mut *mut SLang_Assoc_Array_Type) -> c_int;
}
extern "C" {
    pub fn SLang_assoc_get(
        arg1: *mut SLang_Assoc_Array_Type,
        arg2: *mut SLstr_Type,
        arg3: *mut SLtype,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_assoc_put(
        arg1: *mut SLang_Assoc_Array_Type,
        arg2: *mut SLstr_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_assoc_key_exists(
        arg1: *mut SLang_Assoc_Array_Type,
        arg2: *mut SLstr_Type,
    ) -> c_int;
}
extern {
    pub type SLang_List_Type;
}
extern "C" {
    pub fn SLang_create_list(arg1: c_int) -> *mut SLang_List_Type;
}
extern "C" {
    pub fn SLang_list_append(
        arg1: *mut SLang_List_Type,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_list_insert(
        arg1: *mut SLang_List_Type,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_push_list(
        arg1: *mut SLang_List_Type,
        free_list: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_pop_list(arg1: *mut *mut SLang_List_Type) -> c_int;
}
extern "C" {
    pub fn SLang_free_list(arg1: *mut SLang_List_Type);
}
extern {
    pub type SLang_Foreach_Context_Type;
    pub type SLang_Class_Type;
}
extern "C" {
    pub fn SLclass_push_double_obj(arg1: SLtype, arg2: f64) -> c_int;
}
extern "C" {
    pub fn SLclass_push_float_obj(arg1: SLtype, arg2: f32) -> c_int;
}
extern "C" {
    pub fn SLclass_push_long_obj(
        arg1: SLtype,
        arg2: c_long,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_push_int_obj(arg1: SLtype, arg2: c_int)
        -> c_int;
}
extern "C" {
    pub fn SLclass_push_short_obj(
        arg1: SLtype,
        arg2: c_short,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_push_char_obj(
        arg1: SLtype,
        arg2: c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_push_ptr_obj(arg1: SLtype, arg2: VOID_STAR) -> c_int;
}
extern "C" {
    pub fn SLclass_pop_double_obj(arg1: SLtype, arg2: *mut f64) -> c_int;
}
extern "C" {
    pub fn SLclass_pop_float_obj(arg1: SLtype, arg2: *mut f32) -> c_int;
}
extern "C" {
    pub fn SLclass_pop_long_obj(
        arg1: SLtype,
        arg2: *mut c_long,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_pop_int_obj(
        arg1: SLtype,
        arg2: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_pop_short_obj(
        arg1: SLtype,
        arg2: *mut c_short,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_pop_char_obj(
        arg1: SLtype,
        arg2: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_pop_ptr_obj(arg1: SLtype, arg2: *mut VOID_STAR) -> c_int;
}
extern "C" {
    pub fn SLclass_allocate_class(arg1: *mut c_char) -> *mut SLang_Class_Type;
}
extern "C" {
    pub fn SLclass_get_class_id(cl: *mut SLang_Class_Type) -> c_int;
}
extern "C" {
    pub fn SLclass_create_synonym(
        arg1: *mut c_char,
        arg2: SLtype,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_is_class_defined(arg1: SLtype) -> c_int;
}
extern "C" {
    pub fn SLclass_dup_object(
        type_: SLtype,
        from: VOID_STAR,
        to: VOID_STAR,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_register_class(
        arg1: *mut SLang_Class_Type,
        arg2: SLtype,
        arg3: c_uint,
        arg4: SLclass_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_string_function(
        arg1: *mut SLang_Class_Type,
        arg2: Option<
            unsafe extern "C" fn(arg1: SLtype, arg2: VOID_STAR) -> *mut c_char,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_destroy_function(
        arg1: *mut SLang_Class_Type,
        arg2: Option<unsafe extern "C" fn(arg1: SLtype, arg2: VOID_STAR)>,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_push_function(
        arg1: *mut SLang_Class_Type,
        arg2: Option<
            unsafe extern "C" fn(arg1: SLtype, arg2: VOID_STAR) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_apush_function(
        arg1: *mut SLang_Class_Type,
        arg2: Option<
            unsafe extern "C" fn(arg1: SLtype, arg2: VOID_STAR) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_pop_function(
        arg1: *mut SLang_Class_Type,
        arg2: Option<
            unsafe extern "C" fn(arg1: SLtype, arg2: VOID_STAR) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_aget_function(
        arg1: *mut SLang_Class_Type,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: SLtype,
                arg2: c_uint,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_aput_function(
        arg1: *mut SLang_Class_Type,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: SLtype,
                arg2: c_uint,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_anew_function(
        arg1: *mut SLang_Class_Type,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: SLtype,
                arg2: c_uint,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_sget_function(
        arg1: *mut SLang_Class_Type,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: SLtype,
                arg2: *mut c_char,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_sput_function(
        arg1: *mut SLang_Class_Type,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: SLtype,
                arg2: *mut c_char,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_acopy_function(
        arg1: *mut SLang_Class_Type,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: SLtype,
                arg2: VOID_STAR,
                arg3: VOID_STAR,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_deref_function(
        arg1: *mut SLang_Class_Type,
        arg2: Option<
            unsafe extern "C" fn(arg1: SLtype, arg2: VOID_STAR) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_eqs_function(
        arg1: *mut SLang_Class_Type,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: SLtype,
                arg2: VOID_STAR,
                arg3: SLtype,
                arg4: VOID_STAR,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_length_function(
        arg1: *mut SLang_Class_Type,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: SLtype,
                arg2: VOID_STAR,
                arg3: *mut SLuindex_Type,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_is_container(
        arg1: *mut SLang_Class_Type,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_foreach_functions(
        arg1: *mut SLang_Class_Type,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: SLtype,
                arg2: c_uint,
            ) -> *mut SLang_Foreach_Context_Type,
        >,
        arg3: Option<
            unsafe extern "C" fn(
                arg1: SLtype,
                arg2: *mut SLang_Foreach_Context_Type,
            ) -> c_int,
        >,
        arg4: Option<
            unsafe extern "C" fn(arg1: SLtype, arg2: *mut SLang_Foreach_Context_Type),
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_set_aelem_init_function(
        cl: *mut SLang_Class_Type,
        f: Option<
            unsafe extern "C" fn(arg1: SLtype, arg2: VOID_STAR) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_typecast(
        arg1: SLtype,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_add_unary_op(
        arg1: SLtype,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: c_int,
                arg2: SLtype,
                arg3: VOID_STAR,
                arg4: SLuindex_Type,
                arg5: VOID_STAR,
            ) -> c_int,
        >,
        arg3: Option<
            unsafe extern "C" fn(
                arg1: c_int,
                arg2: SLtype,
                arg3: *mut SLtype,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_add_app_unary_op(
        arg1: SLtype,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: c_int,
                arg2: SLtype,
                arg3: VOID_STAR,
                arg4: SLuindex_Type,
                arg5: VOID_STAR,
            ) -> c_int,
        >,
        arg3: Option<
            unsafe extern "C" fn(
                arg1: c_int,
                arg2: SLtype,
                arg3: *mut SLtype,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_add_binary_op(
        arg1: SLtype,
        arg2: SLtype,
        arg3: Option<
            unsafe extern "C" fn(
                arg1: c_int,
                arg2: SLtype,
                arg3: VOID_STAR,
                arg4: SLuindex_Type,
                arg5: SLtype,
                arg6: VOID_STAR,
                arg7: SLuindex_Type,
                arg8: VOID_STAR,
            ) -> c_int,
        >,
        arg4: Option<
            unsafe extern "C" fn(
                arg1: c_int,
                arg2: SLtype,
                arg3: SLtype,
                arg4: *mut SLtype,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_add_math_op(
        arg1: SLtype,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: c_int,
                arg2: SLtype,
                arg3: VOID_STAR,
                arg4: SLuindex_Type,
                arg5: VOID_STAR,
            ) -> c_int,
        >,
        arg3: Option<
            unsafe extern "C" fn(
                arg1: c_int,
                arg2: SLtype,
                arg3: *mut SLtype,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_add_typecast(
        arg1: SLtype,
        arg2: SLtype,
        arg3: Option<
            unsafe extern "C" fn(
                arg1: SLtype,
                arg2: VOID_STAR,
                arg3: SLuindex_Type,
                arg4: SLtype,
                arg5: VOID_STAR,
            ) -> c_int,
        >,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_get_datatype_name(arg1: SLtype) -> *mut c_char;
}
extern "C" {
    pub fn SLcomplex_abs(arg1: *mut f64) -> f64;
}
extern "C" {
    pub fn SLcomplex_times(arg1: *mut f64, arg2: *mut f64, arg3: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_divide(arg1: *mut f64, arg2: *mut f64, arg3: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_sin(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_cos(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_tan(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_asin(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_acos(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_atan(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_exp(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_log(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_log10(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_sqrt(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_sinh(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_cosh(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_tanh(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_pow(arg1: *mut f64, arg2: *mut f64, arg3: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLmath_hypot(x: f64, y: f64) -> f64;
}
extern "C" {
    pub fn SLcomplex_asinh(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_acosh(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLcomplex_atanh(arg1: *mut f64, arg2: *mut f64) -> *mut f64;
}
extern "C" {
    pub fn SLang_free_mmt(arg1: *mut SLang_MMT_Type);
}
extern "C" {
    pub fn SLang_object_from_mmt(arg1: *mut SLang_MMT_Type) -> VOID_STAR;
}
extern "C" {
    pub fn SLang_create_mmt(arg1: SLtype, arg2: VOID_STAR) -> *mut SLang_MMT_Type;
}
extern "C" {
    pub fn SLang_push_mmt(arg1: *mut SLang_MMT_Type) -> c_int;
}
extern "C" {
    pub fn SLang_pop_mmt(arg1: SLtype) -> *mut SLang_MMT_Type;
}
extern "C" {
    pub fn SLang_inc_mmt(arg1: *mut SLang_MMT_Type);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLang_Array_Type {
    pub data_type: SLtype,
    pub sizeof_type: c_uint,
    pub data: VOID_STAR,
    pub num_elements: SLuindex_Type,
    pub num_dims: c_uint,
    pub dims: [SLindex_Type; SLARRAY_MAX_DIMS],
    pub index_fun: Option<
        unsafe extern "C" fn(arg1: *mut SLang_Array_Type, arg2: *mut SLindex_Type) -> VOID_STAR,
    >,
    pub flags: c_uint,
    pub cl: *mut SLang_Class_Type,
    pub num_refs: c_uint,
    pub free_fun: Option<unsafe extern "C" fn(arg1: *mut SLang_Array_Type)>,
    pub client_data: VOID_STAR,
}
extern "C" {
    pub fn _pSLarray_convert_to_array(
        cd: VOID_STAR,
        get_type: Option<
            unsafe extern "C" fn(
                arg1: VOID_STAR,
                arg2: SLuindex_Type,
                arg3: *mut SLtype,
            ) -> c_int,
        >,
        push: Option<
            unsafe extern "C" fn(arg1: VOID_STAR, arg2: SLuindex_Type) -> c_int,
        >,
        num_objects: SLuindex_Type,
        type_: SLtype,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_pop_array_of_type(
        atp: *mut *mut SLang_Array_Type,
        type_: SLtype,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_pop_array(
        atp: *mut *mut SLang_Array_Type,
        convert_scalar: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_push_array(
        at: *mut SLang_Array_Type,
        do_free: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_free_array(at: *mut SLang_Array_Type);
}
extern "C" {
    pub fn SLang_create_array(
        arg1: SLtype,
        arg2: c_int,
        arg3: VOID_STAR,
        arg4: *mut SLindex_Type,
        arg5: c_uint,
    ) -> *mut SLang_Array_Type;
}
extern "C" {
    pub fn SLang_create_array1(
        arg1: SLtype,
        arg2: c_int,
        arg3: VOID_STAR,
        arg4: *mut SLindex_Type,
        arg5: c_uint,
        arg6: c_int,
    ) -> *mut SLang_Array_Type;
}
extern "C" {
    pub fn SLang_duplicate_array(arg1: *mut SLang_Array_Type) -> *mut SLang_Array_Type;
}
extern "C" {
    pub fn SLang_get_array_element(
        arg1: *mut SLang_Array_Type,
        arg2: *mut SLindex_Type,
        arg3: VOID_STAR,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_set_array_element(
        arg1: *mut SLang_Array_Type,
        arg2: *mut SLindex_Type,
        arg3: VOID_STAR,
    ) -> c_int;
}
pub type SLarray_Contract_Fun_Type = Option<
    unsafe extern "C" fn(
        xp: VOID_STAR,
        increment: c_uint,
        num: c_uint,
        yp: VOID_STAR,
    ) -> c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLarray_Contract_Type {
    pub from_type: SLtype,
    pub typecast_to_type: SLtype,
    pub result_type: SLtype,
    pub f: SLarray_Contract_Fun_Type,
}
extern "C" {
    pub fn SLarray_contract_array(arg1: *const SLarray_Contract_Type) -> c_int;
}
pub type SLarray_Map_Fun_Type = Option<
    unsafe extern "C" fn(
        xtype: SLtype,
        xp: VOID_STAR,
        increment: c_uint,
        num: c_uint,
        ytype: SLtype,
        yp: VOID_STAR,
        clientdata: VOID_STAR,
    ) -> c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLarray_Map_Type {
    pub from_type: SLtype,
    pub typecast_to_type: SLtype,
    pub result_type: SLtype,
    pub f: SLarray_Map_Fun_Type,
}
extern "C" {
    pub fn SLarray_map_array_1(
        arg1: *const SLarray_Map_Type,
        use_this_dim: *mut c_int,
        clientdata: VOID_STAR,
    ) -> c_int;
}
extern "C" {
    pub fn SLarray_map_array(arg1: *const SLarray_Map_Type) -> c_int;
}
extern "C" {
    pub fn SLerr_throw(
        err: c_int,
        msg: *mut c_char,
        obj_type: SLtype,
        objptr: VOID_STAR,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_verror(arg1: c_int, arg2: *mut c_char, ...);
}
/*
extern "C" {
    pub fn SLang_verror_va(
        errcode: c_int,
        fmt: *mut c_char,
        va: *mut __va_list_tag,
    );
}
*/
extern "C" {
    pub fn SLang_get_error() -> c_int;
}
extern "C" {
    pub fn SLang_set_error(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn SLerr_strerror(errcode: c_int) -> *mut c_char;
}
extern "C" {
    pub fn SLerr_new_exception(
        baseclass: c_int,
        name: *mut c_char,
        descript: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLerr_exception_eqs(
        arg1: c_int,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub static mut SL_Any_Error: c_int;
}
extern "C" {
    pub static mut SL_OS_Error: c_int;
}
extern "C" {
    pub static mut SL_Malloc_Error: c_int;
}
extern "C" {
    pub static mut SL_IO_Error: c_int;
}
extern "C" {
    pub static mut SL_Write_Error: c_int;
}
extern "C" {
    pub static mut SL_Read_Error: c_int;
}
extern "C" {
    pub static mut SL_Open_Error: c_int;
}
extern "C" {
    pub static mut SL_RunTime_Error: c_int;
}
extern "C" {
    pub static mut SL_InvalidParm_Error: c_int;
}
extern "C" {
    pub static mut SL_TypeMismatch_Error: c_int;
}
extern "C" {
    pub static mut SL_UserBreak_Error: c_int;
}
extern "C" {
    pub static mut SL_Stack_Error: c_int;
}
extern "C" {
    pub static mut SL_StackOverflow_Error: c_int;
}
extern "C" {
    pub static mut SL_StackUnderflow_Error: c_int;
}
extern "C" {
    pub static mut SL_ReadOnly_Error: c_int;
}
extern "C" {
    pub static mut SL_VariableUninitialized_Error: c_int;
}
extern "C" {
    pub static mut SL_NumArgs_Error: c_int;
}
extern "C" {
    pub static mut SL_Index_Error: c_int;
}
extern "C" {
    pub static mut SL_Parse_Error: c_int;
}
extern "C" {
    pub static mut SL_Syntax_Error: c_int;
}
extern "C" {
    pub static mut SL_DuplicateDefinition_Error: c_int;
}
extern "C" {
    pub static mut SL_UndefinedName_Error: c_int;
}
extern "C" {
    pub static mut SL_Usage_Error: c_int;
}
extern "C" {
    pub static mut SL_Application_Error: c_int;
}
extern "C" {
    pub static mut SL_Internal_Error: c_int;
}
extern "C" {
    pub static mut SL_NotImplemented_Error: c_int;
}
extern "C" {
    pub static mut SL_LimitExceeded_Error: c_int;
}
extern "C" {
    pub static mut SL_Forbidden_Error: c_int;
}
extern "C" {
    pub static mut SL_Math_Error: c_int;
}
extern "C" {
    pub static mut SL_DivideByZero_Error: c_int;
}
extern "C" {
    pub static mut SL_ArithOverflow_Error: c_int;
}
extern "C" {
    pub static mut SL_ArithUnderflow_Error: c_int;
}
extern "C" {
    pub static mut SL_Domain_Error: c_int;
}
extern "C" {
    pub static mut SL_Data_Error: c_int;
}
extern "C" {
    pub static mut SL_Unicode_Error: c_int;
}
extern "C" {
    pub static mut SL_InvalidUTF8_Error: c_int;
}
extern "C" {
    pub static mut SL_Namespace_Error: c_int;
}
extern "C" {
    pub static mut SL_Unknown_Error: c_int;
}
extern "C" {
    pub static mut SL_Import_Error: c_int;
}
extern "C" {
    pub static mut SLang_Traceback: c_int;
}
extern "C" {
    pub static mut SLang_User_Prompt: *mut c_char;
}
extern "C" {
    pub static mut SLang_Version: c_int;
}
extern "C" {
    pub static mut SLang_Version_String: *mut c_char;
}
extern "C" {
    pub static mut SLang_Doc_Dir: *mut c_char;
}
/*
extern "C" {
    pub static mut SLang_VMessage_Hook: Option<
        unsafe extern "C" fn(arg1: *mut c_char, arg2: *mut __va_list_tag),
    >;
}
*/
extern "C" {
    pub fn SLang_vmessage(arg1: *mut c_char, ...);
}
extern "C" {
    pub static mut SLang_Error_Hook:
        Option<unsafe extern "C" fn(arg1: *mut c_char)>;
}
/*
extern "C" {
    pub static mut SLang_Exit_Error_Hook: Option<
        unsafe extern "C" fn(arg1: *mut c_char, arg2: *mut __va_list_tag),
    >;
}
*/
extern "C" {
    pub fn SLang_exit_error(arg1: *mut c_char, ...);
}
extern "C" {
    pub static mut SLang_Dump_Routine:
        Option<unsafe extern "C" fn(arg1: *mut c_char)>;
}
extern "C" {
    pub static mut SLang_Interrupt: Option<unsafe extern "C" fn()>;
}
extern "C" {
    pub static mut SLang_User_Clear_Error: Option<unsafe extern "C" fn()>;
}
extern "C" {
    pub static mut SLang_Enter_Function:
        Option<unsafe extern "C" fn(arg1: *mut c_char)>;
}
extern "C" {
    pub static mut SLang_Exit_Function:
        Option<unsafe extern "C" fn(arg1: *mut c_char)>;
}
extern "C" {
    pub static mut SLang_Num_Function_Args: c_int;
}
extern "C" {
    pub fn SLang_handle_interrupt() -> c_int;
}
extern "C" {
    pub fn SLang_add_interrupt_hook(
        arg1: Option<unsafe extern "C" fn(arg1: VOID_STAR) -> c_int>,
        arg2: VOID_STAR,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_remove_interrupt_hook(
        arg1: Option<unsafe extern "C" fn(arg1: VOID_STAR) -> c_int>,
        arg2: VOID_STAR,
    );
}
extern "C" {
    pub fn SLang_init_all() -> c_int;
}
extern "C" {
    pub fn SLang_init_slang() -> c_int;
}
extern "C" {
    pub fn SLang_init_posix_process() -> c_int;
}
extern "C" {
    pub fn SLang_init_stdio() -> c_int;
}
extern "C" {
    pub fn SLang_init_posix_dir() -> c_int;
}
extern "C" {
    pub fn SLang_init_ospath() -> c_int;
}
extern "C" {
    pub fn SLang_init_slmath() -> c_int;
}
extern "C" {
    pub fn SLang_init_slfile() -> c_int;
}
extern "C" {
    pub fn SLang_init_slunix() -> c_int;
}
extern "C" {
    pub fn SLang_init_slassoc() -> c_int;
}
extern "C" {
    pub fn SLang_init_array() -> c_int;
}
extern "C" {
    pub fn SLang_init_array_extra() -> c_int;
}
extern "C" {
    pub fn SLang_init_signal() -> c_int;
}
extern "C" {
    pub fn SLang_init_import() -> c_int;
}
extern "C" {
    pub fn SLang_load_file(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLang_restart(arg1: c_int);
}
extern "C" {
    pub fn SLang_byte_compile_file(
        arg1: *mut c_char,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_autoload(
        arg1: *mut c_char,
        arg2: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_load_string(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLstack_depth() -> c_int;
}
extern "C" {
    pub fn SLdo_pop() -> c_int;
}
extern "C" {
    pub fn SLdo_pop_n(arg1: c_uint) -> c_int;
}
extern "C" {
    pub fn SLang_push_char(arg1: c_char) -> c_int;
}
extern "C" {
    pub fn SLang_push_uchar(arg1: c_uchar) -> c_int;
}
extern "C" {
    pub fn SLang_pop_char(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLang_pop_uchar(arg1: *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn SLang_push_int(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn SLang_push_uint(arg1: c_uint) -> c_int;
}
extern "C" {
    pub fn SLang_pop_int(arg1: *mut c_int) -> c_int;
}
extern "C" {
    pub fn SLang_pop_uint(arg1: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn SLang_pop_short(arg1: *mut c_short) -> c_int;
}
extern "C" {
    pub fn SLang_pop_ushort(arg1: *mut c_ushort) -> c_int;
}
extern "C" {
    pub fn SLang_push_short(arg1: c_short) -> c_int;
}
extern "C" {
    pub fn SLang_push_ushort(arg1: c_ushort) -> c_int;
}
extern "C" {
    pub fn SLang_pop_long(arg1: *mut c_long) -> c_int;
}
extern "C" {
    pub fn SLang_pop_ulong(arg1: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn SLang_push_long(arg1: c_long) -> c_int;
}
extern "C" {
    pub fn SLang_push_ulong(arg1: c_ulong) -> c_int;
}
extern "C" {
    pub fn SLang_pop_float(arg1: *mut f32) -> c_int;
}
extern "C" {
    pub fn SLang_push_float(arg1: f32) -> c_int;
}
extern "C" {
    pub fn SLang_pop_double(arg1: *mut f64) -> c_int;
}
extern "C" {
    pub fn SLang_push_double(arg1: f64) -> c_int;
}
extern "C" {
    pub fn SLang_push_complex(arg1: f64, arg2: f64) -> c_int;
}
extern "C" {
    pub fn SLang_pop_complex(arg1: *mut f64, arg2: *mut f64) -> c_int;
}
extern "C" {
    pub fn SLang_push_datatype(arg1: SLtype) -> c_int;
}
extern "C" {
    pub fn SLang_pop_datatype(arg1: *mut SLtype) -> c_int;
}
extern "C" {
    pub fn SLang_push_malloced_string(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLang_push_string(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLpop_string(arg1: *mut *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLang_push_null() -> c_int;
}
extern "C" {
    pub fn SLang_pop_null() -> c_int;
}
extern "C" {
    pub fn SLang_push_value(type_: SLtype, arg1: VOID_STAR) -> c_int;
}
extern "C" {
    pub fn SLang_pop_value(type_: SLtype, arg1: VOID_STAR) -> c_int;
}
extern "C" {
    pub fn SLang_free_value(type_: SLtype, arg1: VOID_STAR);
}
extern {
    pub type SLang_Any_Type;
}
extern "C" {
    pub fn SLang_pop_anytype(arg1: *mut *mut SLang_Any_Type) -> c_int;
}
extern "C" {
    pub fn SLang_push_anytype(arg1: *mut SLang_Any_Type) -> c_int;
}
extern "C" {
    pub fn SLang_free_anytype(arg1: *mut SLang_Any_Type);
}
extern "C" {
    pub fn SLang_pop_ref(arg1: *mut *mut SLang_Ref_Type) -> c_int;
}
extern "C" {
    pub fn SLang_free_ref(arg1: *mut SLang_Ref_Type);
}
extern "C" {
    pub fn SLang_assign_to_ref(
        arg1: *mut SLang_Ref_Type,
        arg2: SLtype,
        arg3: VOID_STAR,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_assign_nametype_to_ref(
        arg1: *mut SLang_Ref_Type,
        arg2: *mut SLang_Name_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_pop_function() -> *mut SLang_Name_Type;
}
extern "C" {
    pub fn SLang_push_function(arg1: *mut SLang_Name_Type) -> c_int;
}
extern "C" {
    pub fn SLang_get_fun_from_ref(arg1: *mut SLang_Ref_Type) -> *mut SLang_Name_Type;
}
extern "C" {
    pub fn SLang_free_function(f: *mut SLang_Name_Type);
}
extern "C" {
    pub fn SLang_copy_function(arg1: *mut SLang_Name_Type) -> *mut SLang_Name_Type;
}
extern "C" {
    pub fn SLang_push_cstruct(
        arg1: VOID_STAR,
        arg2: *mut SLang_CStruct_Field_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_pop_cstruct(
        arg1: VOID_STAR,
        arg2: *mut SLang_CStruct_Field_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_free_cstruct(arg1: VOID_STAR, arg2: *mut SLang_CStruct_Field_Type);
}
extern "C" {
    pub fn SLang_assign_cstruct_to_ref(
        arg1: *mut SLang_Ref_Type,
        arg2: VOID_STAR,
        arg3: *mut SLang_CStruct_Field_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_is_defined(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLang_run_hooks(
        arg1: *mut c_char,
        arg2: c_uint,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn SLexecute_function(arg1: *mut SLang_Name_Type) -> c_int;
}
extern "C" {
    pub fn SLang_execute_function(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLang_end_arg_list() -> c_int;
}
extern "C" {
    pub fn SLang_start_arg_list() -> c_int;
}
extern "C" {
    pub fn SLang_add_intrinsic_array(
        arg1: *mut c_char,
        arg2: SLtype,
        arg3: c_int,
        arg4: VOID_STAR,
        arg5: c_uint,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn SLextract_list_element(
        arg1: *mut c_char,
        arg2: c_uint,
        arg3: c_char,
        arg4: *mut c_char,
        arg5: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLexpand_escaped_string(
        dest: *mut c_char,
        src: *mut c_char,
        src_max: *mut c_char,
        utf8_encode: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_get_function(arg1: *mut c_char) -> *mut SLang_Name_Type;
}
extern "C" {
    pub fn SLang_release_function(arg1: *mut SLang_Name_Type);
}
extern "C" {
    pub fn SLreverse_stack(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn SLroll_stack(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn SLstack_exch(
        arg1: c_uint,
        arg2: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLdup_n(n: c_int) -> c_int;
}
extern "C" {
    pub fn SLang_peek_at_stack1() -> c_int;
}
extern "C" {
    pub fn SLang_peek_at_stack() -> c_int;
}
extern "C" {
    pub fn SLang_peek_at_stack_n(n: c_uint) -> c_int;
}
extern "C" {
    pub fn SLang_peek_at_stack1_n(n: c_uint) -> c_int;
}
extern "C" {
    pub fn SLmake_lut(
        arg1: *mut c_uchar,
        arg2: *mut c_uchar,
        arg3: c_uchar,
    );
}
extern "C" {
    pub fn SLang_guess_type(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLstruct_create_struct(
        arg1: c_uint,
        arg2: *mut *mut c_char,
        arg3: *mut SLtype,
        arg4: *mut VOID_STAR,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_add_cleanup_function(
        arg1: Option<unsafe extern "C" fn()>,
    ) -> c_int;
}
extern "C" {
    pub fn SLmake_string(arg1: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn SLmake_nstring(
        arg1: *mut c_char,
        arg2: SLstrlen_Type,
    ) -> *mut c_char;
}
extern {
    pub type SLang_BString_Type;
}
extern "C" {
    pub fn SLbstring_get_pointer(
        arg1: *mut SLang_BString_Type,
        arg2: *mut SLstrlen_Type,
    ) -> *mut c_uchar;
}
extern "C" {
    pub fn SLbstring_dup(arg1: *mut SLang_BString_Type) -> *mut SLang_BString_Type;
}
extern "C" {
    pub fn SLbstring_create(
        arg1: *mut c_uchar,
        arg2: SLstrlen_Type,
    ) -> *mut SLang_BString_Type;
}
extern "C" {
    pub fn SLbstring_create_malloced(
        s: *mut c_uchar,
        len: SLstrlen_Type,
        free_on_error: c_int,
    ) -> *mut SLang_BString_Type;
}
extern "C" {
    pub fn SLbstring_create_slstring(arg1: *mut c_char) -> *mut SLang_BString_Type;
}
extern "C" {
    pub fn SLbstring_free(arg1: *mut SLang_BString_Type);
}
extern "C" {
    pub fn SLang_pop_bstring(arg1: *mut *mut SLang_BString_Type) -> c_int;
}
extern "C" {
    pub fn SLang_push_bstring(arg1: *mut SLang_BString_Type) -> c_int;
}
extern "C" {
    pub fn SLmalloc(arg1: SLstrlen_Type) -> *mut c_char;
}
extern "C" {
    pub fn SLcalloc(arg1: SLstrlen_Type, arg2: SLstrlen_Type) -> *mut c_char;
}
extern "C" {
    pub fn SLfree(arg1: *mut c_char);
}
extern "C" {
    pub fn SLrealloc(
        arg1: *mut c_char,
        arg2: SLstrlen_Type,
    ) -> *mut c_char;
}
extern "C" {
    pub fn SLcurrent_time_string() -> *mut c_char;
}
extern "C" {
    pub fn SLatoi(arg1: *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn SLatol(arg1: *mut c_uchar) -> c_long;
}
extern "C" {
    pub fn SLatoul(arg1: *mut c_uchar) -> c_ulong;
}
extern "C" {
    pub fn SLang_pop_fileptr(
        arg1: *mut *mut SLang_MMT_Type,
        arg2: *mut *mut FILE,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_get_name_from_fileptr(arg1: *mut SLang_MMT_Type) -> *mut c_char;
}
extern "C" {
    pub fn SLang_get_fileptr(
        arg1: *mut SLang_MMT_Type,
        arg2: *mut *mut FILE,
    ) -> c_int;
}
extern {
    pub type SLFile_FD_Type;
}
extern "C" {
    pub fn SLfile_create_fd(
        arg1: *mut c_char,
        arg2: c_int,
    ) -> *mut SLFile_FD_Type;
}
extern "C" {
    pub fn SLfile_free_fd(arg1: *mut SLFile_FD_Type);
}
extern "C" {
    pub fn SLfile_push_fd(arg1: *mut SLFile_FD_Type) -> c_int;
}
extern "C" {
    pub fn SLfile_pop_fd(arg1: *mut *mut SLFile_FD_Type) -> c_int;
}
extern "C" {
    pub fn SLfile_get_fd(
        arg1: *mut SLFile_FD_Type,
        arg2: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLfile_dup_fd(f0: *mut SLFile_FD_Type) -> *mut SLFile_FD_Type;
}
extern "C" {
    pub fn SLang_init_posix_io() -> c_int;
}
extern "C" {
    pub fn SLfile_set_getfd_method(
        f: *mut SLFile_FD_Type,
        func: Option<
            unsafe extern "C" fn(
                arg1: VOID_STAR,
                arg2: *mut c_int,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLfile_set_close_method(
        f: *mut SLFile_FD_Type,
        func: Option<unsafe extern "C" fn(arg1: VOID_STAR) -> c_int>,
    ) -> c_int;
}
extern "C" {
    pub fn SLfile_set_read_method(
        f: *mut SLFile_FD_Type,
        func: Option<
            unsafe extern "C" fn(
                arg1: VOID_STAR,
                arg2: *mut c_char,
                arg3: c_uint,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLfile_set_write_method(
        f: *mut SLFile_FD_Type,
        func: Option<
            unsafe extern "C" fn(
                arg1: VOID_STAR,
                arg2: *mut c_char,
                arg3: c_uint,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLfile_set_dup_method(
        f: *mut SLFile_FD_Type,
        func: Option<unsafe extern "C" fn(arg1: VOID_STAR) -> *mut SLFile_FD_Type>,
    ) -> c_int;
}
extern "C" {
    pub fn SLfile_create_clientdata_id(id: *mut c_int) -> c_int;
}
extern "C" {
    pub fn SLfile_set_clientdata(
        f: *mut SLFile_FD_Type,
        free_func: Option<unsafe extern "C" fn(arg1: VOID_STAR)>,
        cd: VOID_STAR,
        id: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLfile_get_clientdata(
        f: *mut SLFile_FD_Type,
        id: c_int,
        cdp: *mut VOID_STAR,
    ) -> c_int;
}
pub type SLang_To_Double_Fun_Type =
    Option<unsafe extern "C" fn(arg1: VOID_STAR) -> f64>;
extern "C" {
    pub fn SLarith_get_to_double_fun(
        arg1: SLtype,
        arg2: *mut c_uint,
    ) -> SLang_To_Double_Fun_Type;
}
extern "C" {
    pub fn SLang_set_argc_argv(
        arg1: c_int,
        arg2: *mut *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_qualifier_exists(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn SLang_get_int_qualifier(
        name: *const c_char,
        val: *mut c_int,
        defval: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_get_long_qualifier(
        name: *const c_char,
        val: *mut c_long,
        defval: c_long,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_get_double_qualifier(
        name: *const c_char,
        val: *mut f64,
        defval: f64,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_get_string_qualifier(
        name: *const c_char,
        val: *mut *mut c_char,
        defval: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub static mut SLang_TT_Baud_Rate: c_int;
}
extern "C" {
    pub static mut SLang_TT_Read_FD: c_int;
}
extern "C" {
    pub fn SLang_init_tty(
        arg1: c_int,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_reset_tty();
}
extern "C" {
    pub fn SLtty_set_suspend_state(arg1: c_int);
}
extern "C" {
    pub static mut SLang_getkey_intr_hook:
        Option<unsafe extern "C" fn() -> c_int>;
}
extern "C" {
    pub fn SLang_getkey() -> c_uint;
}
extern "C" {
    pub fn SLang_ungetkey_string(
        arg1: *mut c_uchar,
        arg2: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_buffer_keystring(
        arg1: *mut c_uchar,
        arg2: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_ungetkey(arg1: c_uchar) -> c_int;
}
extern "C" {
    pub fn SLang_flush_input();
}
extern "C" {
    pub fn SLang_input_pending(arg1: c_int) -> c_int;
}
extern "C" {
    pub static mut SLang_Abort_Char: c_int;
}
extern "C" {
    pub static mut SLang_Ignore_User_Abort: c_int;
}
extern "C" {
    pub fn SLang_set_abort_signal(
        arg1: Option<unsafe extern "C" fn(arg1: c_int)>,
    ) -> c_int;
}
extern "C" {
    pub static mut SLang_Input_Buffer_Len: c_uint;
}
extern "C" {
    pub static mut SLKeyBoard_Quit: c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLKeymap_Function_Type {
    pub name: *mut c_char,
    pub f: Option<unsafe extern "C" fn() -> c_int>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SLang_Key_Type {
    pub next: *mut SLang_Key_Type,
    pub f: SLang_Key_Type__bindgen_ty_1,
    pub type_: c_uchar,
    pub str_: [c_uchar; SLANG_MAX_KEYMAP_KEY_SEQ + 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SLang_Key_Type__bindgen_ty_1 {
    pub s: *mut c_char,
    pub f: FVOID_STAR,
    pub keysym: c_uint,
    pub slang_fun: *mut SLang_Name_Type,
    _bindgen_union_align: u64,
}
extern "C" {
    pub fn SLkm_set_free_method(
        arg1: c_int,
        arg2: Option<
            unsafe extern "C" fn(arg1: c_int, arg2: VOID_STAR),
        >,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLkeymap_Type {
    pub name: *mut c_char,
    pub keymap: *mut SLang_Key_Type,
    pub functions: *mut SLKeymap_Function_Type,
    pub next: *mut SLkeymap_Type,
}
extern "C" {
    pub static mut SLKeyMap_List_Root: *mut SLkeymap_Type;
}
pub type SLKeyMap_List_Type = SLkeymap_Type;
extern "C" {
    pub fn SLang_process_keystring(
        arg1: *mut c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn SLkm_define_key(
        arg1: *mut c_char,
        arg2: FVOID_STAR,
        arg3: *mut SLkeymap_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_define_key(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut SLkeymap_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLkm_define_keysym(
        arg1: *mut c_char,
        arg2: c_uint,
        arg3: *mut SLkeymap_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLkm_define_slkey(
        keysequence: *mut c_char,
        func: *mut SLang_Name_Type,
        arg1: *mut SLkeymap_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_undefine_key(arg1: *mut c_char, arg2: *mut SLkeymap_Type);
}
extern "C" {
    pub fn SLang_create_keymap(
        arg1: *mut c_char,
        arg2: *mut SLkeymap_Type,
    ) -> *mut SLkeymap_Type;
}
extern "C" {
    pub fn SLang_make_keystring(arg1: *mut c_uchar) -> *mut c_char;
}
extern "C" {
    pub fn SLang_do_key(
        arg1: *mut SLkeymap_Type,
        arg2: Option<unsafe extern "C" fn() -> c_int>,
    ) -> *mut SLang_Key_Type;
}
extern "C" {
    pub fn SLang_find_key_function(
        arg1: *mut c_char,
        arg2: *mut SLkeymap_Type,
    ) -> FVOID_STAR;
}
extern "C" {
    pub fn SLang_find_keymap(arg1: *mut c_char) -> *mut SLkeymap_Type;
}
extern "C" {
    pub static mut SLang_Last_Key_Char: c_int;
}
extern "C" {
    pub static mut SLang_Key_TimeOut_Flag: c_int;
}
extern {
    pub type SLrline_Type;
}
extern "C" {
    pub fn SLrline_open(
        width: c_uint,
        flags: c_uint,
    ) -> *mut SLrline_Type;
}
extern "C" {
    pub fn SLrline_open2(
        arg1: *mut c_char,
        width: c_uint,
        flags: c_uint,
    ) -> *mut SLrline_Type;
}
extern "C" {
    pub fn SLrline_close(arg1: *mut SLrline_Type);
}
extern "C" {
    pub fn SLrline_read_line(
        arg1: *mut SLrline_Type,
        prompt: *mut c_char,
        lenp: *mut c_uint,
    ) -> *mut c_char;
}
extern "C" {
    pub fn SLrline_bol(arg1: *mut SLrline_Type) -> c_int;
}
extern "C" {
    pub fn SLrline_eol(arg1: *mut SLrline_Type) -> c_int;
}
extern "C" {
    pub fn SLrline_del(
        arg1: *mut SLrline_Type,
        len: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_ins(
        arg1: *mut SLrline_Type,
        s: *mut c_char,
        len: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_move(
        arg1: *mut SLrline_Type,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_set_echo(
        arg1: *mut SLrline_Type,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_set_tab(
        arg1: *mut SLrline_Type,
        tabwidth: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_set_point(
        arg1: *mut SLrline_Type,
        arg2: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_set_line(
        arg1: *mut SLrline_Type,
        arg2: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_set_hscroll(
        arg1: *mut SLrline_Type,
        arg2: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_set_display_width(
        arg1: *mut SLrline_Type,
        arg2: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_get_echo(
        arg1: *mut SLrline_Type,
        arg2: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_get_tab(
        arg1: *mut SLrline_Type,
        arg2: *mut c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_get_point(
        arg1: *mut SLrline_Type,
        arg2: *mut c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_get_line(arg1: *mut SLrline_Type) -> *mut c_char;
}
extern "C" {
    pub fn SLrline_get_hscroll(
        arg1: *mut SLrline_Type,
        arg2: *mut c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_get_display_width(
        arg1: *mut SLrline_Type,
        arg2: *mut c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_set_update_hook(
        arg1: *mut SLrline_Type,
        arg2: Option<
            unsafe extern "C" fn(
                rli: *mut SLrline_Type,
                prompt: *mut c_char,
                buf: *mut c_char,
                len: c_uint,
                point: c_uint,
                client_data: VOID_STAR,
            ),
        >,
        client_data: VOID_STAR,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_set_free_update_cb(
        arg1: *mut SLrline_Type,
        arg2: Option<unsafe extern "C" fn(arg1: *mut SLrline_Type, arg2: VOID_STAR)>,
    );
}
extern "C" {
    pub fn SLrline_set_update_clear_cb(
        arg1: *mut SLrline_Type,
        arg2: Option<unsafe extern "C" fn(arg1: *mut SLrline_Type, arg2: VOID_STAR)>,
    );
}
extern "C" {
    pub fn SLrline_set_update_preread_cb(
        arg1: *mut SLrline_Type,
        arg2: Option<unsafe extern "C" fn(arg1: *mut SLrline_Type, arg2: VOID_STAR)>,
    );
}
extern "C" {
    pub fn SLrline_set_update_postread_cb(
        arg1: *mut SLrline_Type,
        arg2: Option<unsafe extern "C" fn(arg1: *mut SLrline_Type, arg2: VOID_STAR)>,
    );
}
extern "C" {
    pub fn SLrline_set_update_width_cb(
        arg1: *mut SLrline_Type,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: *mut SLrline_Type,
                arg2: c_int,
                arg3: VOID_STAR,
            ),
        >,
    );
}
extern "C" {
    pub fn SLrline_get_update_client_data(
        arg1: *mut SLrline_Type,
        arg2: *mut VOID_STAR,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_get_keymap(arg1: *mut SLrline_Type) -> *mut SLkeymap_Type;
}
extern "C" {
    pub fn SLrline_redraw(arg1: *mut SLrline_Type);
}
extern "C" {
    pub fn SLrline_save_line(arg1: *mut SLrline_Type) -> c_int;
}
extern "C" {
    pub fn SLrline_add_to_history(
        arg1: *mut SLrline_Type,
        arg2: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLrline_init(
        appname: *mut c_char,
        user_initfile: *mut c_char,
        sys_initfile: *mut c_char,
    ) -> c_int;
}
pub type SLang_RLine_Info_Type = SLrline_Type;
extern "C" {
    pub static mut SLtt_Num_Chars_Output: c_ulong;
}
extern "C" {
    pub static mut SLtt_Baud_Rate: c_int;
}
extern "C" {
    pub static mut SLtt_Screen_Rows: c_int;
}
extern "C" {
    pub static mut SLtt_Screen_Cols: c_int;
}
extern "C" {
    pub static mut SLtt_Term_Cannot_Insert: c_int;
}
extern "C" {
    pub static mut SLtt_Term_Cannot_Scroll: c_int;
}
extern "C" {
    pub static mut SLtt_Use_Ansi_Colors: c_int;
}
extern "C" {
    pub static mut SLtt_Ignore_Beep: c_int;
}
extern "C" {
    pub static mut SLtt_Force_Keypad_Init: c_int;
}
extern "C" {
    pub static mut SLang_TT_Write_FD: c_int;
}
extern "C" {
    pub static mut SLtt_Graphics_Char_Pairs: *mut c_char;
}
extern "C" {
    pub static mut SLtt_Blink_Mode: c_int;
}
extern "C" {
    pub static mut SLtt_Use_Blink_For_ACS: c_int;
}
extern "C" {
    pub static mut SLtt_Newline_Ok: c_int;
}
extern "C" {
    pub static mut SLtt_Has_Alt_Charset: c_int;
}
extern "C" {
    pub static mut SLtt_Has_Status_Line: c_int;
}
extern "C" {
    pub static mut SLtt_Try_Termcap: c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLsmg_Char_Type {
    pub nchars: c_uint,
    pub wchars: [SLwchar_Type; SLSMG_MAX_CHARS_PER_CELL],
    pub color: SLsmg_Color_Type,
}
extern "C" {
    pub fn SLtt_flush_output() -> c_int;
}
extern "C" {
    pub fn SLtt_set_scroll_region(arg1: c_int, arg2: c_int);
}
extern "C" {
    pub fn SLtt_reset_scroll_region();
}
extern "C" {
    pub fn SLtt_reverse_video(arg1: c_int);
}
extern "C" {
    pub fn SLtt_bold_video();
}
extern "C" {
    pub fn SLtt_begin_insert();
}
extern "C" {
    pub fn SLtt_end_insert();
}
extern "C" {
    pub fn SLtt_del_eol();
}
extern "C" {
    pub fn SLtt_goto_rc(arg1: c_int, arg2: c_int);
}
extern "C" {
    pub fn SLtt_delete_nlines(arg1: c_int);
}
extern "C" {
    pub fn SLtt_delete_char();
}
extern "C" {
    pub fn SLtt_erase_line();
}
extern "C" {
    pub fn SLtt_normal_video();
}
extern "C" {
    pub fn SLtt_cls();
}
extern "C" {
    pub fn SLtt_beep();
}
extern "C" {
    pub fn SLtt_reverse_index(arg1: c_int);
}
extern "C" {
    pub fn SLtt_smart_puts(
        arg1: *mut SLsmg_Char_Type,
        arg2: *mut SLsmg_Char_Type,
        arg3: c_int,
        arg4: c_int,
    );
}
extern "C" {
    pub fn SLtt_write_string(arg1: *mut c_char);
}
extern "C" {
    pub fn SLtt_putchar(arg1: c_char);
}
extern "C" {
    pub fn SLtt_init_video() -> c_int;
}
extern "C" {
    pub fn SLtt_reset_video() -> c_int;
}
extern "C" {
    pub fn SLtt_get_terminfo();
}
extern "C" {
    pub fn SLtt_get_screen_size();
}
extern "C" {
    pub fn SLtt_set_cursor_visibility(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn SLtt_set_mouse_mode(
        arg1: c_int,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLtt_initialize(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLtt_enable_cursor_keys();
}
extern "C" {
    pub fn SLtt_set_term_vtxxx(arg1: *mut c_int);
}
extern "C" {
    pub fn SLtt_wide_width();
}
extern "C" {
    pub fn SLtt_narrow_width();
}
extern "C" {
    pub fn SLtt_set_alt_char_set(arg1: c_int);
}
extern "C" {
    pub fn SLtt_write_to_status_line(
        arg1: *mut c_char,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLtt_disable_status_line();
}
extern "C" {
    pub fn SLtt_tgetstr(arg1: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn SLtt_tgetnum(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLtt_tgetflag(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLtt_tgetent(name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLtt_tgoto(
        cap: *mut c_char,
        col: c_int,
        row: c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn SLtt_tputs(
        str_: *mut c_char,
        affcnt: c_int,
        putcfun: Option<
            unsafe extern "C" fn(arg1: c_int) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLtt_tigetent(arg1: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn SLtt_tigetstr(
        arg1: *mut c_char,
        arg2: *mut *mut c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn SLtt_tigetnum(
        arg1: *mut c_char,
        arg2: *mut *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLtt_get_color_object(arg1: c_int) -> SLtt_Char_Type;
}
extern "C" {
    pub fn SLtt_set_color_object(
        arg1: c_int,
        arg2: SLtt_Char_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLtt_set_color(
        arg1: c_int,
        arg2: *const c_char,
        arg3: *const c_char,
        arg4: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLtt_set_mono(
        arg1: c_int,
        arg2: *const c_char,
        arg3: SLtt_Char_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLtt_add_color_attribute(
        arg1: c_int,
        arg2: SLtt_Char_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLtt_set_color_fgbg(
        arg1: c_int,
        arg2: SLtt_Char_Type,
        arg3: SLtt_Char_Type,
    ) -> c_int;
}
extern {
    pub type SLprep_Type;
}
extern "C" {
    pub fn SLprep_new() -> *mut SLprep_Type;
}
extern "C" {
    pub fn SLprep_delete(arg1: *mut SLprep_Type);
}
extern "C" {
    pub fn SLprep_line_ok(
        arg1: *mut c_char,
        arg2: *mut SLprep_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLprep_set_flags(
        arg1: *mut SLprep_Type,
        flags: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLprep_set_comment(
        arg1: *mut SLprep_Type,
        arg2: *mut c_char,
        arg3: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLprep_set_prefix(
        arg1: *mut SLprep_Type,
        arg2: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn SLprep_set_exists_hook(
        arg1: *mut SLprep_Type,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: *mut SLprep_Type,
                arg2: *mut c_char,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLprep_set_eval_hook(
        arg1: *mut SLprep_Type,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: *mut SLprep_Type,
                arg2: *mut c_char,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn SLdefine_for_ifdef(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLsmg_fill_region(
        arg1: c_int,
        arg2: c_int,
        arg3: c_uint,
        arg4: c_uint,
        arg5: SLwchar_Type,
    );
}
extern "C" {
    pub fn SLsmg_set_char_set(arg1: c_int);
}
extern "C" {
    pub static mut SLsmg_Scroll_Hash_Border: c_int;
}
extern "C" {
    pub fn SLsmg_suspend_smg() -> c_int;
}
extern "C" {
    pub fn SLsmg_resume_smg() -> c_int;
}
extern "C" {
    pub fn SLsmg_erase_eol();
}
extern "C" {
    pub fn SLsmg_gotorc(arg1: c_int, arg2: c_int);
}
extern "C" {
    pub fn SLsmg_erase_eos();
}
extern "C" {
    pub fn SLsmg_reverse_video();
}
extern "C" {
    pub fn SLsmg_set_color(arg1: SLsmg_Color_Type);
}
extern "C" {
    pub fn SLsmg_normal_video();
}
extern "C" {
    pub fn SLsmg_printf(arg1: *const c_char, ...);
}
/*
extern "C" {
    pub fn SLsmg_vprintf(arg1: *const c_char, arg2: *mut __va_list_tag);
}
*/
extern "C" {
    pub fn SLsmg_write_string(arg1: *const c_char);
}
extern "C" {
    pub fn SLsmg_write_nstring(arg1: *const c_char, arg2: c_uint);
}
extern "C" {
    pub fn SLsmg_write_chars(u: *const SLuchar_Type, umax: *mut SLuchar_Type);
}
extern "C" {
    pub fn SLsmg_write_nchars(str_: *const c_char, len: c_uint);
}
extern "C" {
    pub fn SLsmg_write_char(ch: SLwchar_Type);
}
extern "C" {
    pub fn SLsmg_write_wrapped_string(
        arg1: *mut SLuchar_Type,
        arg2: c_int,
        arg3: c_int,
        arg4: c_uint,
        arg5: c_uint,
        arg6: c_int,
    );
}
extern "C" {
    pub fn SLsmg_cls();
}
extern "C" {
    pub fn SLsmg_refresh();
}
extern "C" {
    pub fn SLsmg_touch_lines(arg1: c_int, arg2: c_uint);
}
extern "C" {
    pub fn SLsmg_touch_screen();
}
extern "C" {
    #[must_use]
    pub fn SLsmg_init_smg() -> c_int;
}
extern "C" {
    pub fn SLsmg_reinit_smg() -> c_int;
}
extern "C" {
    pub fn SLsmg_reset_smg();
}
extern "C" {
    pub fn SLsmg_char_at(arg1: *mut SLsmg_Char_Type) -> c_int;
}
extern "C" {
    pub fn SLsmg_set_screen_start(
        arg1: *mut c_int,
        arg2: *mut c_int,
    );
}
extern "C" {
    pub fn SLsmg_draw_hline(arg1: c_uint);
}
extern "C" {
    pub fn SLsmg_draw_vline(arg1: c_int);
}
extern "C" {
    pub fn SLsmg_draw_object(
        arg1: c_int,
        arg2: c_int,
        arg3: SLwchar_Type,
    );
}
extern "C" {
    pub fn SLsmg_draw_box(
        arg1: c_int,
        arg2: c_int,
        arg3: c_uint,
        arg4: c_uint,
    );
}
extern "C" {
    pub fn SLsmg_get_column() -> c_int;
}
extern "C" {
    pub fn SLsmg_get_row() -> c_int;
}
extern "C" {
    pub fn SLsmg_forward(arg1: c_int);
}
extern "C" {
    pub fn SLsmg_write_color_chars(arg1: *mut SLsmg_Char_Type, arg2: c_uint);
}
extern "C" {
    pub fn SLsmg_read_raw(
        arg1: *mut SLsmg_Char_Type,
        arg2: c_uint,
    ) -> c_uint;
}
extern "C" {
    pub fn SLsmg_write_raw(
        arg1: *mut SLsmg_Char_Type,
        arg2: c_uint,
    ) -> c_uint;
}
extern "C" {
    pub fn SLsmg_set_color_in_region(
        arg1: c_int,
        arg2: c_int,
        arg3: c_int,
        arg4: c_uint,
        arg5: c_uint,
    );
}
extern "C" {
    pub fn SLsmg_strwidth(u: *mut SLuchar_Type, max: *mut SLuchar_Type) -> c_uint;
}
extern "C" {
    pub fn SLsmg_strbytes(
        u: *mut SLuchar_Type,
        max: *mut SLuchar_Type,
        width: c_uint,
    ) -> c_uint;
}
extern "C" {
    pub fn SLsmg_embedded_escape_mode(on_or_off: c_int) -> c_int;
}
extern "C" {
    pub static mut SLsmg_Display_Eight_Bit: c_int;
}
extern "C" {
    pub static mut SLsmg_Tab_Width: c_int;
}
extern "C" {
    pub static mut SLsmg_Newline_Behavior: c_int;
}
extern "C" {
    pub static mut SLsmg_Backspace_Moves: c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLsmg_Term_Type {
    pub tt_normal_video: Option<unsafe extern "C" fn()>,
    pub tt_set_scroll_region: Option<
        unsafe extern "C" fn(arg1: c_int, arg2: c_int),
    >,
    pub tt_goto_rc: Option<
        unsafe extern "C" fn(arg1: c_int, arg2: c_int),
    >,
    pub tt_reverse_index: Option<unsafe extern "C" fn(arg1: c_int)>,
    pub tt_reset_scroll_region: Option<unsafe extern "C" fn()>,
    pub tt_delete_nlines: Option<unsafe extern "C" fn(arg1: c_int)>,
    pub tt_cls: Option<unsafe extern "C" fn()>,
    pub tt_del_eol: Option<unsafe extern "C" fn()>,
    pub tt_smart_puts: Option<
        unsafe extern "C" fn(
            arg1: *mut SLsmg_Char_Type,
            arg2: *mut SLsmg_Char_Type,
            arg3: c_int,
            arg4: c_int,
        ),
    >,
    pub tt_flush_output: Option<unsafe extern "C" fn() -> c_int>,
    pub tt_reset_video: Option<unsafe extern "C" fn() -> c_int>,
    pub tt_init_video: Option<unsafe extern "C" fn() -> c_int>,
    pub tt_screen_rows: *mut c_int,
    pub tt_screen_cols: *mut c_int,
    pub tt_term_cannot_scroll: *mut c_int,
    pub tt_has_alt_charset: *mut c_int,
    pub tt_graphic_char_pairs: *mut *mut c_char,
    pub unicode_ok: *mut c_int,
    pub reserved: [c_long; 4usize],
}
extern "C" {
    pub fn SLsmg_set_terminal_info(arg1: *mut SLsmg_Term_Type);
}
extern "C" {
    pub fn SLkp_define_keysym(
        arg1: *mut c_char,
        arg2: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLkp_init() -> c_int;
}
extern "C" {
    pub fn SLkp_set_getkey_function(
        arg1: Option<unsafe extern "C" fn() -> c_int>,
    );
}
extern "C" {
    pub fn SLkp_getkey() -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLscroll_Type {
    pub next: *mut SLscroll_Type,
    pub prev: *mut SLscroll_Type,
    pub flags: c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLscroll_Window_Type {
    pub flags: c_uint,
    pub top_window_line: *mut SLscroll_Type,
    pub bot_window_line: *mut SLscroll_Type,
    pub current_line: *mut SLscroll_Type,
    pub lines: *mut SLscroll_Type,
    pub nrows: c_uint,
    pub hidden_mask: c_uint,
    pub line_num: c_uint,
    pub num_lines: c_uint,
    pub window_row: c_uint,
    pub border: c_uint,
    pub cannot_scroll: c_int,
}
extern "C" {
    pub fn SLscroll_find_top(arg1: *mut SLscroll_Window_Type) -> c_int;
}
extern "C" {
    pub fn SLscroll_find_line_num(arg1: *mut SLscroll_Window_Type) -> c_int;
}
extern "C" {
    pub fn SLscroll_next_n(
        arg1: *mut SLscroll_Window_Type,
        arg2: c_uint,
    ) -> c_uint;
}
extern "C" {
    pub fn SLscroll_prev_n(
        arg1: *mut SLscroll_Window_Type,
        arg2: c_uint,
    ) -> c_uint;
}
extern "C" {
    pub fn SLscroll_pageup(arg1: *mut SLscroll_Window_Type) -> c_int;
}
extern "C" {
    pub fn SLscroll_pagedown(arg1: *mut SLscroll_Window_Type) -> c_int;
}
pub type SLSig_Fun_Type = Option<unsafe extern "C" fn(arg1: c_int)>;
extern "C" {
    pub fn SLsignal(arg1: c_int, arg2: SLSig_Fun_Type) -> SLSig_Fun_Type;
}
extern "C" {
    pub fn SLsignal_intr(arg1: c_int, arg2: SLSig_Fun_Type) -> SLSig_Fun_Type;
}
extern "C" {
    pub fn SLsig_block_signals() -> c_int;
}
extern "C" {
    pub fn SLsig_unblock_signals() -> c_int;
}
extern "C" {
    pub fn SLsystem(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLsystem_intr(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLsig_forbid_signal(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn SLerrno_strerror(arg1: c_int) -> *mut c_char;
}
extern "C" {
    pub fn SLerrno_set_errno(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn SLfpu_clear_except_bits();
}
extern "C" {
    pub fn SLfpu_test_except_bits(bits: c_uint) -> c_uint;
}
extern "C" {
    pub fn SLang_get_int_type(nbits: c_int) -> SLtype;
}
extern "C" {
    pub fn SLang_get_int_size(arg1: SLtype) -> c_int;
}
extern "C" {
    pub fn SLadd_intrinsic_variable(
        arg1: *mut c_char,
        arg2: VOID_STAR,
        arg3: SLtype,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLadd_intrinsic_function(
        arg1: *mut c_char,
        arg2: FVOID_STAR,
        arg3: SLtype,
        arg4: c_uint,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn SLns_add_intrinsic_variable(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut c_char,
        arg3: VOID_STAR,
        arg4: SLtype,
        arg5: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn SLns_add_intrinsic_function(
        arg1: *mut SLang_NameSpace_Type,
        arg2: *mut c_char,
        arg3: FVOID_STAR,
        arg4: SLtype,
        arg5: c_uint,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_patch_intrin_fun_table(
        table: *mut SLang_Intrin_Fun_Type,
        from_types: *mut SLtype,
        to_types: *mut SLtype,
        num: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLclass_patch_intrin_fun_table1(
        table: *mut SLang_Intrin_Fun_Type,
        from_type: SLtype,
        to_type: SLtype,
    ) -> c_int;
}
extern "C" {
    pub fn SLang_define_case(arg1: *mut c_int, arg2: *mut c_int);
}
extern "C" {
    pub fn SLang_init_case_tables();
}
extern "C" {
    pub static mut _pSLChg_UCase_Lut: [c_uchar; 256usize];
}
extern "C" {
    pub static mut _pSLChg_LCase_Lut: [c_uchar; 256usize];
}
extern {
    pub type SLRegexp_Type;
}
extern "C" {
    pub fn SLregexp_compile(
        pattern: *mut c_char,
        flags: c_uint,
    ) -> *mut SLRegexp_Type;
}
extern "C" {
    pub fn SLregexp_free(arg1: *mut SLRegexp_Type);
}
extern "C" {
    pub fn SLregexp_match(
        compiled_regexp: *mut SLRegexp_Type,
        str_: *mut c_char,
        len: SLstrlen_Type,
    ) -> *mut c_char;
}
extern "C" {
    pub fn SLregexp_nth_match(
        arg1: *mut SLRegexp_Type,
        nth: c_uint,
        ofsp: *mut SLstrlen_Type,
        lenp: *mut SLstrlen_Type,
    ) -> c_int;
}
extern "C" {
    pub fn SLregexp_get_hints(
        arg1: *mut SLRegexp_Type,
        flagsp: *mut c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn SLregexp_quote_string(
        pattern: *mut c_char,
        buf: *mut c_char,
        buflen: c_uint,
    ) -> *mut c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLcmd_Cmd_Table_Type {
    pub table: *mut SLcmd_Cmd_Type,
    pub argc: c_int,
    pub string_args: *mut *mut c_char,
    pub int_args: *mut c_int,
    pub double_args: *mut f64,
    pub arg_type: *mut SLtype,
    pub reserved: [c_ulong; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SLcmd_Cmd_Type {
    pub cmdfun: Option<
        unsafe extern "C" fn(
            arg1: c_int,
            arg2: *mut SLcmd_Cmd_Table_Type,
        ) -> c_int,
    >,
    pub cmd: *mut c_char,
    pub arg_type: *mut c_char,
}
extern "C" {
    pub fn SLcmd_execute_string(
        arg1: *mut c_char,
        arg2: *mut SLcmd_Cmd_Table_Type,
    ) -> c_int;
}
extern {
    pub type SLsearch_Type;
}
extern "C" {
    pub fn SLsearch_new(
        u: *mut SLuchar_Type,
        search_flags: c_int,
    ) -> *mut SLsearch_Type;
}
extern "C" {
    pub fn SLsearch_delete(arg1: *mut SLsearch_Type);
}
extern "C" {
    pub fn SLsearch_forward(
        st: *mut SLsearch_Type,
        pmin: *mut SLuchar_Type,
        pmax: *mut SLuchar_Type,
    ) -> *mut SLuchar_Type;
}
extern "C" {
    pub fn SLsearch_backward(
        st: *mut SLsearch_Type,
        pmin: *mut SLuchar_Type,
        pstart: *mut SLuchar_Type,
        pmax: *mut SLuchar_Type,
    ) -> *mut SLuchar_Type;
}
extern "C" {
    pub fn SLsearch_match_len(arg1: *mut SLsearch_Type) -> SLstrlen_Type;
}
extern "C" {
    pub fn SLpath_basename(arg1: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn SLpath_extname(arg1: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn SLpath_is_absolute_path(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLpath_get_delimiter() -> c_int;
}
extern "C" {
    pub fn SLpath_set_delimiter(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn SLpath_set_load_path(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLpath_get_load_path() -> *mut c_char;
}
extern "C" {
    pub fn SLpath_dircat(
        arg1: *mut c_char,
        arg2: *mut c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn SLpath_find_file_in_path(
        arg1: *mut c_char,
        arg2: *mut c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn SLpath_dirname(arg1: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn SLpath_file_exists(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn SLpath_pathname_sans_extname(
        arg1: *mut c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn SLpath_getcwd() -> *mut c_char;
}
extern "C" {
    pub fn SLang_set_module_load_path(arg1: *mut c_char) -> c_int;
}
/*
extern "C" {
    pub fn SLvsnprintf(
        arg1: *mut c_char,
        arg2: c_uint,
        arg3: *mut c_char,
        arg4: *mut __va_list_tag,
    ) -> c_int;
}
*/
extern "C" {
    pub fn SLsnprintf(
        arg1: *mut c_char,
        arg2: c_uint,
        arg3: *mut c_char,
        ...
    ) -> c_int;
}
