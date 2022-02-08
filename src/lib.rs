// SPDX-License-Identifier: (Apache-2.0 OR MIT)

#![cfg_attr(feature = "unstable-simd", feature(core_intrinsics))]
#![cfg_attr(feature = "unstable-simd", feature(optimize_attribute))]
#![allow(unused_unsafe)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_field_names)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::zero_prefixed_literal)]

#[macro_use]
mod util;

mod deserialize;
mod exc;
mod ffi;
mod opt;
mod serialize;
mod typeref;
mod unicode;

use pyo3::{prelude::*, AsPyPointer};
// use pyo3::types::*;
// use pyo3::create_exception;
// use pyo3::ffi::*;
use std::borrow::Cow;
use std::os::raw::c_char;
use std::ptr::NonNull;

const DUMPS_DOC: &str =
    "dumps(obj, /, default=None, option=None)\n--\n\nSerialize Python objects to JSON.\0";
const LOADS_DOC: &str = "loads(obj, /)\n--\n\nDeserialize JSON to Python objects.\0";

macro_rules! opt {
    ($mptr:expr, $name:expr, $opt:expr) => {
        unsafe {
            #[cfg(all(not(target_os = "windows"), target_pointer_width = "64"))]
            PyModule_AddIntConstant($mptr, $name.as_ptr() as *const c_char, $opt as i64);
            #[cfg(all(not(target_os = "windows"), target_pointer_width = "32"))]
            PyModule_AddIntConstant($mptr, $name.as_ptr() as *const c_char, $opt as i32);
            #[cfg(target_os = "windows")]
            PyModule_AddIntConstant($mptr, $name.as_ptr() as *const c_char, $opt as i32);
        }
    };
}

// create_exception!(orjson, JsonEncodeError, pyo3::exceptions::PyException);


/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


/// A Python module implemented in Rust.
#[pymodule]
fn orjson(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    let version = env!("CARGO_PKG_VERSION");
    m.add("__version__", version)?;
    
    let mptr = m.as_ptr();

    unsafe {
        pyo3::ffi::PyModule_AddObject(
            mptr,
            "veron\0".as_ptr() as *const c_char,
            pyo3::ffi::PyUnicode_FromStringAndSize("asd".as_ptr() as *const c_char, version.len() as isize),
        )
    };

    // maturin>=0.11.0 creates a python package that imports *, hiding dunder by default
    let all: [&str; 3] = [
        "__all__\0",
        "__version__\0",
        "veron\0",
        // "dumps\0",
        // "JSONDecodeError\0",
        // "JSONEncodeError\0",
        // "loads\0",
        // "OPT_APPEND_NEWLINE\0",
        // "OPT_INDENT_2\0",
        // "OPT_NAIVE_UTC\0",
        // "OPT_NON_STR_KEYS\0",
        // "OPT_OMIT_MICROSECONDS\0",
        // "OPT_PASSTHROUGH_DATACLASS\0",
        // "OPT_PASSTHROUGH_DATETIME\0",
        // "OPT_PASSTHROUGH_SUBCLASS\0",
        // "OPT_SERIALIZE_DATACLASS\0",
        // "OPT_SERIALIZE_NUMPY\0",
        // "OPT_SERIALIZE_UUID\0",
        // "OPT_SORT_KEYS\0",
        // "OPT_STRICT_INTEGER\0",
        // "OPT_UTC_Z\0",
    ];

    unsafe {
        let pyall = pyo3::ffi::PyTuple_New(all.len() as isize);
        for (i, obj) in all.iter().enumerate() {
            pyo3::ffi::PyTuple_SET_ITEM(
                pyall,
                i as isize,
                pyo3::ffi::PyUnicode_InternFromString(obj.as_ptr() as *const c_char),
            )
        }
        pyo3::ffi::PyModule_AddObject(mptr, "__all__\0".as_ptr() as *const c_char, pyall);
    };

    // m.add("JsonEncodeError", _py.get_type::<JsonEncodeError>())?;
    Ok(())
}

// #[allow(non_snake_case)]
// #[no_mangle]
// #[cold]
// #[cfg_attr(feature = "unstable-simd", optimize(size))]
// pub unsafe extern "C" fn PyInit_orjson() -> *mut PyObject {
//     let init = PyModuleDef {
//         m_base: PyModuleDef_HEAD_INIT,
//         m_name: "orjson\0".as_ptr() as *const c_char,
//         m_doc: std::ptr::null(),
//         m_size: 0,
//         m_methods: std::ptr::null_mut(),
//         m_slots: std::ptr::null_mut(),
//         m_traverse: None,
//         m_clear: None,
//         m_free: None,
//     };
//     let mptr = PyModule_Create(Box::into_raw(Box::new(init)));

//     let version = env!("CARGO_PKG_VERSION");
//     unsafe {
//         PyModule_AddObject(
//             mptr,
//             "__version__\0".as_ptr() as *const c_char,
//             PyUnicode_FromStringAndSize(version.as_ptr() as *const c_char, version.len() as isize),
//         )
//     };

//     let wrapped_dumps: PyMethodDef;

//     #[cfg(Py_3_8)]
//     {
//         wrapped_dumps = PyMethodDef {
//             ml_name: "dumps\0".as_ptr() as *const c_char,
//             ml_meth: Some(unsafe {
//                 std::mem::transmute::<pyo3::ffi::_PyCFunctionFastWithKeywords, PyCFunction>(dumps)
//             }),
//             ml_flags: pyo3::ffi::METH_FASTCALL | METH_KEYWORDS,
//             ml_doc: DUMPS_DOC.as_ptr() as *const c_char,
//         };
//     }
//     #[cfg(not(Py_3_8))]
//     {
//         wrapped_dumps = PyMethodDef {
//             ml_name: "dumps\0".as_ptr() as *const c_char,
//             ml_meth: Some(unsafe {
//                 std::mem::transmute::<PyCFunctionWithKeywords, PyCFunction>(dumps)
//             }),
//             ml_flags: METH_VARARGS | METH_KEYWORDS,
//             ml_doc: DUMPS_DOC.as_ptr() as *const c_char,
//         };
//     }
//     unsafe {
//         PyModule_AddObject(
//             mptr,
//             "dumps\0".as_ptr() as *const c_char,
//             PyCFunction_NewEx(
//                 Box::into_raw(Box::new(wrapped_dumps)),
//                 std::ptr::null_mut(),
//                 PyUnicode_InternFromString("orjson\0".as_ptr() as *const c_char),
//             ),
//         )
//     };

//     let wrapped_loads = PyMethodDef {
//         ml_name: "loads\0".as_ptr() as *const c_char,
//         ml_meth: Some(loads),
//         ml_flags: METH_O,
//         ml_doc: LOADS_DOC.as_ptr() as *const c_char,
//     };

//     unsafe {
//         PyModule_AddObject(
//             mptr,
//             "loads\0".as_ptr() as *const c_char,
//             PyCFunction_NewEx(
//                 Box::into_raw(Box::new(wrapped_loads)),
//                 std::ptr::null_mut(),
//                 PyUnicode_InternFromString("orjson\0".as_ptr() as *const c_char),
//             ),
//         )
//     };

//     opt!(mptr, "OPT_APPEND_NEWLINE\0", opt::APPEND_NEWLINE);
//     opt!(mptr, "OPT_INDENT_2\0", opt::INDENT_2);
//     opt!(mptr, "OPT_NAIVE_UTC\0", opt::NAIVE_UTC);
//     opt!(mptr, "OPT_NON_STR_KEYS\0", opt::NON_STR_KEYS);
//     opt!(mptr, "OPT_OMIT_MICROSECONDS\0", opt::OMIT_MICROSECONDS);
//     opt!(
//         mptr,
//         "OPT_PASSTHROUGH_DATACLASS\0",
//         opt::PASSTHROUGH_DATACLASS
//     );
//     opt!(
//         mptr,
//         "OPT_PASSTHROUGH_DATETIME\0",
//         opt::PASSTHROUGH_DATETIME
//     );
//     opt!(
//         mptr,
//         "OPT_PASSTHROUGH_SUBCLASS\0",
//         opt::PASSTHROUGH_SUBCLASS
//     );
//     opt!(mptr, "OPT_SERIALIZE_DATACLASS\0", opt::SERIALIZE_DATACLASS);
//     opt!(mptr, "OPT_SERIALIZE_NUMPY\0", opt::SERIALIZE_NUMPY);
//     opt!(mptr, "OPT_SERIALIZE_UUID\0", opt::SERIALIZE_UUID);
//     opt!(mptr, "OPT_SORT_KEYS\0", opt::SORT_KEYS);
//     opt!(mptr, "OPT_STRICT_INTEGER\0", opt::STRICT_INTEGER);
//     opt!(mptr, "OPT_UTC_Z\0", opt::UTC_Z);

//     typeref::init_typerefs();

//     unsafe {
//         PyModule_AddObject(
//             mptr,
//             "JSONDecodeError\0".as_ptr() as *const c_char,
//             typeref::JsonDecodeError,
//         );
//         PyModule_AddObject(
//             mptr,
//             "JSONEncodeError\0".as_ptr() as *const c_char,
//             typeref::JsonEncodeError,
//         )
//     };

//     // maturin>=0.11.0 creates a python package that imports *, hiding dunder by default
//     let all: [&str; 20] = [
//         "__all__\0",
//         "__version__\0",
//         "dumps\0",
//         "JSONDecodeError\0",
//         "JSONEncodeError\0",
//         "loads\0",
//         "OPT_APPEND_NEWLINE\0",
//         "OPT_INDENT_2\0",
//         "OPT_NAIVE_UTC\0",
//         "OPT_NON_STR_KEYS\0",
//         "OPT_OMIT_MICROSECONDS\0",
//         "OPT_PASSTHROUGH_DATACLASS\0",
//         "OPT_PASSTHROUGH_DATETIME\0",
//         "OPT_PASSTHROUGH_SUBCLASS\0",
//         "OPT_SERIALIZE_DATACLASS\0",
//         "OPT_SERIALIZE_NUMPY\0",
//         "OPT_SERIALIZE_UUID\0",
//         "OPT_SORT_KEYS\0",
//         "OPT_STRICT_INTEGER\0",
//         "OPT_UTC_Z\0",
//     ];

//     let pyall = PyTuple_New(all.len() as isize);
//     for (i, obj) in all.iter().enumerate() {
//         PyTuple_SET_ITEM(
//             pyall,
//             i as isize,
//             PyUnicode_InternFromString(obj.as_ptr() as *const c_char),
//         )
//     }

//     unsafe {
//         PyModule_AddObject(mptr, "__all__\0".as_ptr() as *const c_char, pyall);
//     };

//     mptr
// }

// #[cold]
// #[inline(never)]
// #[cfg_attr(feature = "unstable-simd", optimize(size))]
// fn raise_loads_exception(err: deserialize::DeserializeError) -> *mut PyObject {
//     let pos = err.pos() as i64;
//     let msg = err.message;
//     let doc = err.data;
//     unsafe {
//         let err_msg =
//             PyUnicode_FromStringAndSize(msg.as_ptr() as *const c_char, msg.len() as isize);
//         let args = PyTuple_New(3);
//         let doc = PyUnicode_FromStringAndSize(doc.as_ptr() as *const c_char, doc.len() as isize);
//         let pos = PyLong_FromLongLong(pos);
//         PyTuple_SET_ITEM(args, 0, err_msg);
//         PyTuple_SET_ITEM(args, 1, doc);
//         PyTuple_SET_ITEM(args, 2, pos);
//         PyErr_SetObject(typeref::JsonDecodeError, args);
//         Py_DECREF(args);
//     };
//     std::ptr::null_mut()
// }

// #[cold]
// #[inline(never)]
// #[cfg_attr(feature = "unstable-simd", optimize(size))]
fn raise_dumps_exception(msg: Cow<str>) -> *mut pyo3::ffi::PyObject {
    unsafe {
        let err_msg =
            pyo3::ffi::PyUnicode_FromStringAndSize(msg.as_ptr() as *const c_char, msg.len() as isize);
            pyo3::ffi::PyErr_SetObject(typeref::JsonEncodeError, err_msg);
            pyo3::ffi::Py_DECREF(err_msg);
    };
    std::ptr::null_mut()
}

pub unsafe extern "C" fn dumps(
    _self: *mut pyo3::ffi::PyObject,
    args: *mut pyo3::ffi::PyObject,
    kwds: *mut pyo3::ffi::PyObject,
) -> *mut pyo3::ffi::PyObject {
    let mut default: Option<NonNull<pyo3::ffi::PyObject>> = None;
    let mut optsptr: Option<NonNull<pyo3::ffi::PyObject>> = None;

    let obj = pyo3::ffi::PyTuple_GET_ITEM(args, 0);

    let num_args = pyo3::ffi::PyTuple_GET_SIZE(args);
    if unlikely!(num_args == 0) {
        return raise_dumps_exception(Cow::Borrowed(
            "dumps() missing 1 required positional argument: 'obj'",
        ));
    }
    if num_args & 2 == 2 {
        default = Some(NonNull::new_unchecked(pyo3::ffi::PyTuple_GET_ITEM(args, 1)));
    }
    if num_args & 3 == 3 {
        optsptr = Some(NonNull::new_unchecked(pyo3::ffi::PyTuple_GET_ITEM(args, 2)));
    }

    if !kwds.is_null() {
        let len = unsafe { crate::ffi::PyDict_GET_SIZE(kwds) };
        let mut pos = 0isize;
        let mut arg: *mut pyo3::ffi::PyObject = std::ptr::null_mut();
        let mut val: *mut pyo3::ffi::PyObject = std::ptr::null_mut();
        for _ in 0..=len.saturating_sub(1) {
            unsafe { pyo3::ffi::_PyDict_Next(kwds, &mut pos, &mut arg, &mut val, std::ptr::null_mut()) };
            if arg == typeref::DEFAULT {
                if unlikely!(num_args & 2 == 2) {
                    return raise_dumps_exception(Cow::Borrowed(
                        "dumps() got multiple values for argument: 'default'",
                    ));
                }
                default = Some(NonNull::new_unchecked(val));
            } else if arg == typeref::OPTION {
                if unlikely!(num_args & 3 == 3) {
                    return raise_dumps_exception(Cow::Borrowed(
                        "dumps() got multiple values for argument: 'option'",
                    ));
                }
                optsptr = Some(NonNull::new_unchecked(val));
            } else if arg.is_null() {
                break;
            } else {
                return raise_dumps_exception(Cow::Borrowed(
                    "dumps() got an unexpected keyword argument",
                ));
            }
        }
    }

    let mut optsbits: i32 = 0;
    if let Some(opts) = optsptr {
        if (*opts.as_ptr()).ob_type != typeref::INT_TYPE {
            return raise_dumps_exception(Cow::Borrowed("Invalid opts"));
        }
        optsbits = pyo3::ffi::PyLong_AsLong(optsptr.unwrap().as_ptr()) as i32;
        if optsbits < 0 || optsbits > opt::MAX_OPT {
            return raise_dumps_exception(Cow::Borrowed("Invalid opts"));
        }
    }

    match crate::serialize::serialize(obj, default, optsbits as opt::Opt) {
        Ok(val) => val.as_ptr(),
        Err(err) => raise_dumps_exception(Cow::Owned(err)),
    }
}



// #[pyfunction]
// fn dumps(
//     args: &PyTuple, 
//     kwds: Option<&PyDict>
// ) -> PyResult<()> {
//     let mut default: Option<NonNull<PyObject>> = None;
//     let mut optsptr: Option<NonNull<PyObject>> = None;

//     let obj = args.get_item(0);

//     let num_args = args.len();
//     if unlikely!(num_args == 0) {
//         Err(JsonEncodeError(
//             "dumps() takes at least 1 argument (0 given)"
//         ))
//         // return raise_dumps_exception(Cow::Borrowed(
//         //     "dumps() missing 1 required positional argument: 'obj'",
//         // ));
//     }
//     if num_args & 2 == 2 {
//         default = Some(NonNull::new_unchecked(args.get_item(1)));
//     }
//     if num_args & 3 == 3 {
//         optsptr = Some(NonNull::new_unchecked(args.get_item(2)));
//     }

//     if !kwds.is_null() {
//         let len = unsafe { kwds.len() };
//         let mut pos = 0isize;
//         let mut arg: *mut PyObject = std::ptr::null_mut();
//         let mut val: *mut PyObject = std::ptr::null_mut();
//         for _ in 0..=len.saturating_sub(1) {
//             unsafe { pyo3::ffi::_PyDict_Next(kwds, &mut pos, &mut arg, &mut val, std::ptr::null_mut()) };
//             if arg == typeref::DEFAULT {
//                 if unlikely!(num_args & 2 == 2) {
//                     return raise_dumps_exception(Cow::Borrowed(
//                         "dumps() got multiple values for argument: 'default'",
//                     ));
//                 }
//                 default = Some(NonNull::new_unchecked(val));
//             } else if arg == typeref::OPTION {
//                 if unlikely!(num_args & 3 == 3) {
//                     return raise_dumps_exception(Cow::Borrowed(
//                         "dumps() got multiple values for argument: 'option'",
//                     ));
//                 }
//                 optsptr = Some(NonNull::new_unchecked(val));
//             } else if arg.is_null() {
//                 break;
//             } else {
//                 return raise_dumps_exception(Cow::Borrowed(
//                     "dumps() got an unexpected keyword argument",
//                 ));
//             }
//         }
//     }

//     let mut optsbits: i32 = 0;
//     if let Some(opts) = optsptr {
//         if (*opts.as_ptr()).ob_type != typeref::INT_TYPE {
//             return raise_dumps_exception(Cow::Borrowed("Invalid opts"));
//         }
//         optsbits = pyo3::ffi::PyLong_AsLong(optsptr.unwrap().as_ptr()) as i32;
//         if optsbits < 0 || optsbits > opt::MAX_OPT {
//             return raise_dumps_exception(Cow::Borrowed("Invalid opts"));
//         }
//     }

//     match crate::serialize::serialize(obj, default, optsbits as opt::Opt) {
//         Ok(val) => val.as_ptr(),
//         Err(err) => raise_dumps_exception(Cow::Owned(err)),
//     }
// }
