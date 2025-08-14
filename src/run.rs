pub fn runpy(py: String) {}
use proc_macro::Ident;
use proc_macro::Literal;
use proc_macro::TokenStream;
use proc_macro::quote;
use std::cell::RefCell;
use std::ffi::CStr;
use std::ffi::CString;
use std::sync::Mutex;
use std::sync::RwLock;

use proc_macro::Span;
use proc_macro::TokenTree;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::*;

struct R(proc_macro::TokenStream);
impl<'py, 's> FromPyObject<'py> for R {
    fn extract_bound(x: &Bound<'py, PyAny>) -> Result<R, PyErr> {
        let tree: TokenTree = match () {
            () if let Ok(x) = x.extract::<i128>() => Literal::i128_unsuffixed(x).into(),
            () if let Ok(x) = x.extract::<f64>() => Literal::f64_unsuffixed(x).into(),
            () if let Ok(x) = x.extract::<bool>() => {
                Ident::new(&x.to_string(), Span::call_site()).into()
            }
            () if let Ok(x) = x.extract::<String>() => {
                return Ok(R(x
                    .parse::<TokenStream>()
                    .unwrap_or(quote::quote!(compile_error!("lex failure")).into())));
            }

            // () if let Ok(x) = x.downcast::<PyList>() => {
            //     if let Ok(y) = x.get_item(0) {
            //         match () {
            //             () if y.is_instance_of::<PyFloat>() => Val::Array(Array::Float(
            //                 x.into_iter().map(|x| x.extract::<f64>()).try_collect()?,
            //             )),
            //             () if y.is_instance_of::<PyInt>() => Val::Array(Array::Int(
            //                 x.into_iter().map(|x| x.extract::<i128>()).try_collect()?,
            //             )),
            //             _ => {
            //                 return Err(PyTypeError::new_err("bad array types"));
            //             }
            //         }
            //     } else {
            //         Val::Array(Array::Int(vec![]))
            //     }
            // }
            // () if let Ok(x) = x.downcast::<PySet>() => Val::Set(
            //     x.into_iter()
            //         .map(|x| x.extract::<Val<'s>>())
            //         .try_collect()?,
            // ),
            _ => return Err(PyTypeError::new_err("bad types")),
        };
        let mut t = TokenStream::new();
        t.extend([tree]);
        Ok(R(t))
    }
}

pub fn exec<'s>(code: &CStr) -> TokenStream {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|g| {
        g.run(
            cr#"__import__("sys").stdout = __import__("io").StringIO()"#,
            None,
            None,
        )
        .unwrap();
        Ok::<_, ()>(
            g.run(&code, None, None)
                .map(|()| {
                    g.eval(cr#"__import__("sys").stdout.getvalue()"#, None, None)
                        .unwrap()
                        .extract::<String>()
                        .unwrap()
                        .parse::<TokenStream>()
                        .unwrap()
                })
                .unwrap_or_else(|x| {
                    eprintln!("error:");
                    x.display(g);
                    let e = x.to_string();
                    quote::quote!(compile_error!("there was a problem running the python: ", #e))
                        .into()
                }),
        )
        // stack.extend(
        //     x.into_iter()
        //         .skip(n.saturating_sub(argc.output))
        //         .map(|x| x.extract::<Val<'_>>().map(|x| x.spun(span)))
        //         .try_collect::<Vec<_>>()
        //         .map_err(|_| Error::lazy(code.span, "nooo"))?,
        // );
    })
    .unwrap()
}
