// Generated bindings for python v3.7.15
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::useless_transmute)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::trivially_copy_pass_by_ref)]

/* automatically generated by rust-bindgen 0.65.1 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __darwin_wchar_t = ::std::os::raw::c_int;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut ::std::os::raw::c_uchar,
    pub _size: ::std::os::raw::c_int,
}
impl Default for __sbuf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILE {
    pub _p: *mut ::std::os::raw::c_uchar,
    pub _r: ::std::os::raw::c_int,
    pub _w: ::std::os::raw::c_int,
    pub _flags: ::std::os::raw::c_short,
    pub _file: ::std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::std::os::raw::c_int,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub _read: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: fpos_t,
            arg3: ::std::os::raw::c_int,
        ) -> fpos_t,
    >,
    pub _write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::std::os::raw::c_int,
    pub _ubuf: [::std::os::raw::c_uchar; 3usize],
    pub _nbuf: [::std::os::raw::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::std::os::raw::c_int,
    pub _offset: fpos_t,
}
impl Default for __sFILE {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type FILE = __sFILE;
pub type wchar_t = __darwin_wchar_t;
pub type Py_ssize_t = isize;
pub type Py_hash_t = Py_ssize_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
}
impl Default for _object {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type PyObject = _object;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyVarObject {
    pub ob_base: PyObject,
    pub ob_size: Py_ssize_t,
}
impl Default for PyVarObject {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type unaryfunc =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject>;
pub type binaryfunc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject,
>;
pub type ternaryfunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> *mut PyObject,
>;
pub type inquiry =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject) -> ::std::os::raw::c_int>;
pub type lenfunc = ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject) -> Py_ssize_t>;
pub type ssizeargfunc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: Py_ssize_t) -> *mut PyObject,
>;
pub type ssizeobjargproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: Py_ssize_t,
        arg3: *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
pub type objobjargproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bufferinfo {
    pub buf: *mut ::std::os::raw::c_void,
    pub obj: *mut PyObject,
    pub len: Py_ssize_t,
    pub itemsize: Py_ssize_t,
    pub readonly: ::std::os::raw::c_int,
    pub ndim: ::std::os::raw::c_int,
    pub format: *mut ::std::os::raw::c_char,
    pub shape: *mut Py_ssize_t,
    pub strides: *mut Py_ssize_t,
    pub suboffsets: *mut Py_ssize_t,
    pub internal: *mut ::std::os::raw::c_void,
}
impl Default for bufferinfo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type Py_buffer = bufferinfo;
pub type getbufferproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut Py_buffer,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type releasebufferproc =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut Py_buffer)>;
pub type objobjproc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject) -> ::std::os::raw::c_int,
>;
pub type visitproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type traverseproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: visitproc,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyNumberMethods {
    pub nb_add: binaryfunc,
    pub nb_subtract: binaryfunc,
    pub nb_multiply: binaryfunc,
    pub nb_remainder: binaryfunc,
    pub nb_divmod: binaryfunc,
    pub nb_power: ternaryfunc,
    pub nb_negative: unaryfunc,
    pub nb_positive: unaryfunc,
    pub nb_absolute: unaryfunc,
    pub nb_bool: inquiry,
    pub nb_invert: unaryfunc,
    pub nb_lshift: binaryfunc,
    pub nb_rshift: binaryfunc,
    pub nb_and: binaryfunc,
    pub nb_xor: binaryfunc,
    pub nb_or: binaryfunc,
    pub nb_int: unaryfunc,
    pub nb_reserved: *mut ::std::os::raw::c_void,
    pub nb_float: unaryfunc,
    pub nb_inplace_add: binaryfunc,
    pub nb_inplace_subtract: binaryfunc,
    pub nb_inplace_multiply: binaryfunc,
    pub nb_inplace_remainder: binaryfunc,
    pub nb_inplace_power: ternaryfunc,
    pub nb_inplace_lshift: binaryfunc,
    pub nb_inplace_rshift: binaryfunc,
    pub nb_inplace_and: binaryfunc,
    pub nb_inplace_xor: binaryfunc,
    pub nb_inplace_or: binaryfunc,
    pub nb_floor_divide: binaryfunc,
    pub nb_true_divide: binaryfunc,
    pub nb_inplace_floor_divide: binaryfunc,
    pub nb_inplace_true_divide: binaryfunc,
    pub nb_index: unaryfunc,
    pub nb_matrix_multiply: binaryfunc,
    pub nb_inplace_matrix_multiply: binaryfunc,
}
impl Default for PyNumberMethods {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PySequenceMethods {
    pub sq_length: lenfunc,
    pub sq_concat: binaryfunc,
    pub sq_repeat: ssizeargfunc,
    pub sq_item: ssizeargfunc,
    pub was_sq_slice: *mut ::std::os::raw::c_void,
    pub sq_ass_item: ssizeobjargproc,
    pub was_sq_ass_slice: *mut ::std::os::raw::c_void,
    pub sq_contains: objobjproc,
    pub sq_inplace_concat: binaryfunc,
    pub sq_inplace_repeat: ssizeargfunc,
}
impl Default for PySequenceMethods {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PyMappingMethods {
    pub mp_length: lenfunc,
    pub mp_subscript: binaryfunc,
    pub mp_ass_subscript: objobjargproc,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PyAsyncMethods {
    pub am_await: unaryfunc,
    pub am_aiter: unaryfunc,
    pub am_anext: unaryfunc,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PyBufferProcs {
    pub bf_getbuffer: getbufferproc,
    pub bf_releasebuffer: releasebufferproc,
}
pub type freefunc = ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>;
pub type destructor = ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject)>;
pub type printfunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut FILE,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type getattrfunc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut ::std::os::raw::c_char) -> *mut PyObject,
>;
pub type getattrofunc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject,
>;
pub type setattrfunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut ::std::os::raw::c_char,
        arg3: *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
pub type setattrofunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
pub type reprfunc =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject>;
pub type hashfunc = ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject) -> Py_hash_t>;
pub type richcmpfunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: ::std::os::raw::c_int,
    ) -> *mut PyObject,
>;
pub type getiterfunc =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject>;
pub type iternextfunc =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject>;
pub type descrgetfunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> *mut PyObject,
>;
pub type descrsetfunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
pub type initproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
pub type newfunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut _typeobject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> *mut PyObject,
>;
pub type allocfunc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut _typeobject, arg2: Py_ssize_t) -> *mut PyObject,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _typeobject {
    pub ob_base: PyVarObject,
    pub tp_name: *const ::std::os::raw::c_char,
    pub tp_basicsize: Py_ssize_t,
    pub tp_itemsize: Py_ssize_t,
    pub tp_dealloc: destructor,
    pub tp_print: printfunc,
    pub tp_getattr: getattrfunc,
    pub tp_setattr: setattrfunc,
    pub tp_as_async: *mut PyAsyncMethods,
    pub tp_repr: reprfunc,
    pub tp_as_number: *mut PyNumberMethods,
    pub tp_as_sequence: *mut PySequenceMethods,
    pub tp_as_mapping: *mut PyMappingMethods,
    pub tp_hash: hashfunc,
    pub tp_call: ternaryfunc,
    pub tp_str: reprfunc,
    pub tp_getattro: getattrofunc,
    pub tp_setattro: setattrofunc,
    pub tp_as_buffer: *mut PyBufferProcs,
    pub tp_flags: ::std::os::raw::c_ulong,
    pub tp_doc: *const ::std::os::raw::c_char,
    pub tp_traverse: traverseproc,
    pub tp_clear: inquiry,
    pub tp_richcompare: richcmpfunc,
    pub tp_weaklistoffset: Py_ssize_t,
    pub tp_iter: getiterfunc,
    pub tp_iternext: iternextfunc,
    pub tp_methods: *mut PyMethodDef,
    pub tp_members: *mut PyMemberDef,
    pub tp_getset: *mut PyGetSetDef,
    pub tp_base: *mut _typeobject,
    pub tp_dict: *mut PyObject,
    pub tp_descr_get: descrgetfunc,
    pub tp_descr_set: descrsetfunc,
    pub tp_dictoffset: Py_ssize_t,
    pub tp_init: initproc,
    pub tp_alloc: allocfunc,
    pub tp_new: newfunc,
    pub tp_free: freefunc,
    pub tp_is_gc: inquiry,
    pub tp_bases: *mut PyObject,
    pub tp_mro: *mut PyObject,
    pub tp_cache: *mut PyObject,
    pub tp_subclasses: *mut PyObject,
    pub tp_weaklist: *mut PyObject,
    pub tp_del: destructor,
    pub tp_version_tag: ::std::os::raw::c_uint,
    pub tp_finalize: destructor,
}
impl Default for _typeobject {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type PyTypeObject = _typeobject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _heaptypeobject {
    pub ht_type: PyTypeObject,
    pub as_async: PyAsyncMethods,
    pub as_number: PyNumberMethods,
    pub as_mapping: PyMappingMethods,
    pub as_sequence: PySequenceMethods,
    pub as_buffer: PyBufferProcs,
    pub ht_name: *mut PyObject,
    pub ht_slots: *mut PyObject,
    pub ht_qualname: *mut PyObject,
    pub ht_cached_keys: *mut _dictkeysobject,
}
impl Default for _heaptypeobject {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type PyHeapTypeObject = _heaptypeobject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyBytesObject {
    pub ob_base: PyVarObject,
    pub ob_shash: Py_hash_t,
    pub ob_sval: [::std::os::raw::c_char; 1usize],
}
impl Default for PyBytesObject {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type Py_UCS4 = u32;
pub type Py_UCS2 = u16;
pub type Py_UCS1 = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyASCIIObject {
    pub ob_base: PyObject,
    pub length: Py_ssize_t,
    pub hash: Py_hash_t,
    pub state: PyASCIIObject__bindgen_ty_1,
    pub wstr: *mut wchar_t,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct PyASCIIObject__bindgen_ty_1 {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
impl PyASCIIObject__bindgen_ty_1 {
    #[inline]
    pub fn interned(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_interned(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn kind(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_kind(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn compact(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_compact(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn ascii(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_ascii(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn ready(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_ready(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        interned: ::std::os::raw::c_uint,
        kind: ::std::os::raw::c_uint,
        compact: ::std::os::raw::c_uint,
        ascii: ::std::os::raw::c_uint,
        ready: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 2u8, {
            let interned: u32 = unsafe { ::std::mem::transmute(interned) };
            interned as u64
        });
        __bindgen_bitfield_unit.set(2usize, 3u8, {
            let kind: u32 = unsafe { ::std::mem::transmute(kind) };
            kind as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let compact: u32 = unsafe { ::std::mem::transmute(compact) };
            compact as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let ascii: u32 = unsafe { ::std::mem::transmute(ascii) };
            ascii as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let ready: u32 = unsafe { ::std::mem::transmute(ready) };
            ready as u64
        });
        __bindgen_bitfield_unit
    }
}
impl Default for PyASCIIObject {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyCompactUnicodeObject {
    pub _base: PyASCIIObject,
    pub utf8_length: Py_ssize_t,
    pub utf8: *mut ::std::os::raw::c_char,
    pub wstr_length: Py_ssize_t,
}
impl Default for PyCompactUnicodeObject {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyUnicodeObject {
    pub _base: PyCompactUnicodeObject,
    pub data: PyUnicodeObject__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union PyUnicodeObject__bindgen_ty_1 {
    pub any: *mut ::std::os::raw::c_void,
    pub latin1: *mut Py_UCS1,
    pub ucs2: *mut Py_UCS2,
    pub ucs4: *mut Py_UCS4,
}
impl Default for PyUnicodeObject__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for PyUnicodeObject {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type PyLongObject = _longobject;
pub type digit = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _longobject {
    pub ob_base: PyVarObject,
    pub ob_digit: [digit; 1usize],
}
impl Default for _longobject {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyFloatObject {
    pub ob_base: PyObject,
    pub ob_fval: f64,
}
impl Default for PyFloatObject {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyTupleObject {
    pub ob_base: PyVarObject,
    pub ob_item: [*mut PyObject; 1usize],
}
impl Default for PyTupleObject {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyListObject {
    pub ob_base: PyVarObject,
    pub ob_item: *mut *mut PyObject,
    pub allocated: Py_ssize_t,
}
impl Default for PyListObject {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type PyDictKeysObject = _dictkeysobject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyDictObject {
    pub ob_base: PyObject,
    pub ma_used: Py_ssize_t,
    pub ma_version_tag: u64,
    pub ma_keys: *mut PyDictKeysObject,
    pub ma_values: *mut *mut PyObject,
}
impl Default for PyDictObject {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type PyCFunction = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyMethodDef {
    pub ml_name: *const ::std::os::raw::c_char,
    pub ml_meth: PyCFunction,
    pub ml_flags: ::std::os::raw::c_int,
    pub ml_doc: *const ::std::os::raw::c_char,
}
impl Default for PyMethodDef {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type PyThread_type_lock = *mut ::std::os::raw::c_void;
pub type _PyFrameEvalFunction = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut _frame, arg2: ::std::os::raw::c_int) -> *mut PyObject,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PyCoreConfig {
    pub install_signal_handlers: ::std::os::raw::c_int,
    pub ignore_environment: ::std::os::raw::c_int,
    pub use_hash_seed: ::std::os::raw::c_int,
    pub hash_seed: ::std::os::raw::c_ulong,
    pub allocator: *const ::std::os::raw::c_char,
    pub dev_mode: ::std::os::raw::c_int,
    pub faulthandler: ::std::os::raw::c_int,
    pub tracemalloc: ::std::os::raw::c_int,
    pub import_time: ::std::os::raw::c_int,
    pub show_ref_count: ::std::os::raw::c_int,
    pub show_alloc_count: ::std::os::raw::c_int,
    pub dump_refs: ::std::os::raw::c_int,
    pub malloc_stats: ::std::os::raw::c_int,
    pub coerce_c_locale: ::std::os::raw::c_int,
    pub coerce_c_locale_warn: ::std::os::raw::c_int,
    pub utf8_mode: ::std::os::raw::c_int,
    pub program_name: *mut wchar_t,
    pub argc: ::std::os::raw::c_int,
    pub argv: *mut *mut wchar_t,
    pub program: *mut wchar_t,
    pub nxoption: ::std::os::raw::c_int,
    pub xoptions: *mut *mut wchar_t,
    pub nwarnoption: ::std::os::raw::c_int,
    pub warnoptions: *mut *mut wchar_t,
    pub module_search_path_env: *mut wchar_t,
    pub home: *mut wchar_t,
    pub nmodule_search_path: ::std::os::raw::c_int,
    pub module_search_paths: *mut *mut wchar_t,
    pub executable: *mut wchar_t,
    pub prefix: *mut wchar_t,
    pub base_prefix: *mut wchar_t,
    pub exec_prefix: *mut wchar_t,
    pub base_exec_prefix: *mut wchar_t,
    pub _disable_importlib: ::std::os::raw::c_int,
}
impl Default for _PyCoreConfig {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PyMainInterpreterConfig {
    pub install_signal_handlers: ::std::os::raw::c_int,
    pub argv: *mut PyObject,
    pub executable: *mut PyObject,
    pub prefix: *mut PyObject,
    pub base_prefix: *mut PyObject,
    pub exec_prefix: *mut PyObject,
    pub base_exec_prefix: *mut PyObject,
    pub warnoptions: *mut PyObject,
    pub xoptions: *mut PyObject,
    pub module_search_path: *mut PyObject,
}
impl Default for _PyMainInterpreterConfig {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _is {
    pub next: *mut _is,
    pub tstate_head: *mut _ts,
    pub id: i64,
    pub id_refcount: i64,
    pub id_mutex: PyThread_type_lock,
    pub modules: *mut PyObject,
    pub modules_by_index: *mut PyObject,
    pub sysdict: *mut PyObject,
    pub builtins: *mut PyObject,
    pub importlib: *mut PyObject,
    pub check_interval: ::std::os::raw::c_int,
    pub num_threads: ::std::os::raw::c_long,
    pub pythread_stacksize: usize,
    pub codec_search_path: *mut PyObject,
    pub codec_search_cache: *mut PyObject,
    pub codec_error_registry: *mut PyObject,
    pub codecs_initialized: ::std::os::raw::c_int,
    pub fscodec_initialized: ::std::os::raw::c_int,
    pub core_config: _PyCoreConfig,
    pub config: _PyMainInterpreterConfig,
    pub dlopenflags: ::std::os::raw::c_int,
    pub builtins_copy: *mut PyObject,
    pub import_func: *mut PyObject,
    pub eval_frame: _PyFrameEvalFunction,
    pub co_extra_user_count: Py_ssize_t,
    pub co_extra_freefuncs: [freefunc; 255usize],
    pub before_forkers: *mut PyObject,
    pub after_forkers_parent: *mut PyObject,
    pub after_forkers_child: *mut PyObject,
    pub pyexitfunc: ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject)>,
    pub pyexitmodule: *mut PyObject,
    pub tstate_next_unique_id: u64,
}
impl Default for _is {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type PyInterpreterState = _is;
pub type Py_tracefunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut _frame,
        arg3: ::std::os::raw::c_int,
        arg4: *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _err_stackitem {
    pub exc_type: *mut PyObject,
    pub exc_value: *mut PyObject,
    pub exc_traceback: *mut PyObject,
    pub previous_item: *mut _err_stackitem,
}
impl Default for _err_stackitem {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type _PyErr_StackItem = _err_stackitem;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ts {
    pub prev: *mut _ts,
    pub next: *mut _ts,
    pub interp: *mut PyInterpreterState,
    pub frame: *mut _frame,
    pub recursion_depth: ::std::os::raw::c_int,
    pub overflowed: ::std::os::raw::c_char,
    pub recursion_critical: ::std::os::raw::c_char,
    pub stackcheck_counter: ::std::os::raw::c_int,
    pub tracing: ::std::os::raw::c_int,
    pub use_tracing: ::std::os::raw::c_int,
    pub c_profilefunc: Py_tracefunc,
    pub c_tracefunc: Py_tracefunc,
    pub c_profileobj: *mut PyObject,
    pub c_traceobj: *mut PyObject,
    pub curexc_type: *mut PyObject,
    pub curexc_value: *mut PyObject,
    pub curexc_traceback: *mut PyObject,
    pub exc_state: _PyErr_StackItem,
    pub exc_info: *mut _PyErr_StackItem,
    pub dict: *mut PyObject,
    pub gilstate_counter: ::std::os::raw::c_int,
    pub async_exc: *mut PyObject,
    pub thread_id: ::std::os::raw::c_ulong,
    pub trash_delete_nesting: ::std::os::raw::c_int,
    pub trash_delete_later: *mut PyObject,
    pub on_delete: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub on_delete_data: *mut ::std::os::raw::c_void,
    pub coroutine_origin_tracking_depth: ::std::os::raw::c_int,
    pub coroutine_wrapper: *mut PyObject,
    pub in_coroutine_wrapper: ::std::os::raw::c_int,
    pub async_gen_firstiter: *mut PyObject,
    pub async_gen_finalizer: *mut PyObject,
    pub context: *mut PyObject,
    pub context_ver: u64,
    pub id: u64,
}
impl Default for _ts {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type PyThreadState = _ts;
pub type getter = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut ::std::os::raw::c_void) -> *mut PyObject,
>;
pub type setter = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyGetSetDef {
    pub name: *const ::std::os::raw::c_char,
    pub get: getter,
    pub set: setter,
    pub doc: *const ::std::os::raw::c_char,
    pub closure: *mut ::std::os::raw::c_void,
}
impl Default for PyGetSetDef {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyMemberDef {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyCodeObject {
    pub ob_base: PyObject,
    pub co_argcount: ::std::os::raw::c_int,
    pub co_kwonlyargcount: ::std::os::raw::c_int,
    pub co_nlocals: ::std::os::raw::c_int,
    pub co_stacksize: ::std::os::raw::c_int,
    pub co_flags: ::std::os::raw::c_int,
    pub co_firstlineno: ::std::os::raw::c_int,
    pub co_code: *mut PyObject,
    pub co_consts: *mut PyObject,
    pub co_names: *mut PyObject,
    pub co_varnames: *mut PyObject,
    pub co_freevars: *mut PyObject,
    pub co_cellvars: *mut PyObject,
    pub co_cell2arg: *mut Py_ssize_t,
    pub co_filename: *mut PyObject,
    pub co_name: *mut PyObject,
    pub co_lnotab: *mut PyObject,
    pub co_zombieframe: *mut ::std::os::raw::c_void,
    pub co_weakreflist: *mut PyObject,
    pub co_extra: *mut ::std::os::raw::c_void,
}
impl Default for PyCodeObject {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PyTryBlock {
    pub b_type: ::std::os::raw::c_int,
    pub b_handler: ::std::os::raw::c_int,
    pub b_level: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _frame {
    pub ob_base: PyVarObject,
    pub f_back: *mut _frame,
    pub f_code: *mut PyCodeObject,
    pub f_builtins: *mut PyObject,
    pub f_globals: *mut PyObject,
    pub f_locals: *mut PyObject,
    pub f_valuestack: *mut *mut PyObject,
    pub f_stacktop: *mut *mut PyObject,
    pub f_trace: *mut PyObject,
    pub f_trace_lines: ::std::os::raw::c_char,
    pub f_trace_opcodes: ::std::os::raw::c_char,
    pub f_gen: *mut PyObject,
    pub f_lasti: ::std::os::raw::c_int,
    pub f_lineno: ::std::os::raw::c_int,
    pub f_iblock: ::std::os::raw::c_int,
    pub f_executing: ::std::os::raw::c_char,
    pub f_blockstack: [PyTryBlock; 20usize],
    pub f_localsplus: [*mut PyObject; 1usize],
}
impl Default for _frame {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type PyFrameObject = _frame;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyDictKeyEntry {
    pub me_hash: Py_hash_t,
    pub me_key: *mut PyObject,
    pub me_value: *mut PyObject,
}
impl Default for PyDictKeyEntry {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type dict_lookup_func = ::std::option::Option<
    unsafe extern "C" fn(
        mp: *mut PyDictObject,
        key: *mut PyObject,
        hash: Py_hash_t,
        value_addr: *mut *mut PyObject,
    ) -> Py_ssize_t,
>;
#[repr(C)]
#[derive(Debug, Default)]
pub struct _dictkeysobject {
    pub dk_refcnt: Py_ssize_t,
    pub dk_size: Py_ssize_t,
    pub dk_lookup: dict_lookup_func,
    pub dk_usable: Py_ssize_t,
    pub dk_nentries: Py_ssize_t,
    pub dk_indices: __IncompleteArrayField<::std::os::raw::c_char>,
}
