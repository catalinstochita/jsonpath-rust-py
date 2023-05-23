mod tests;

use std::str::FromStr;
use jsonpath_rust::{JsonPathFinder, JsonPathInst, JsonPathQuery};
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};
use serde_json::Value;

#[pyfunction]
fn find_slice<'a>(py: Python<'a>,json: &PyAny,path:&str) -> PyResult<PyObject>{
    let json: Box<Value> = Box::new(depythonize(json).unwrap());

    let path: Box<JsonPathInst> = Box::from(
        JsonPathInst::from_str(path).expect("the path is incorrect"),
    );
    let finder = JsonPathFinder::new(json, path);
    let result:Vec<Value> = finder.find_slice()
        //Convert slice to values
        .into_iter().map(|slice|slice.to_data()).collect();

    match pythonize(py,&result) {
        Ok(ok) => {Ok(ok)}
        Err(err) => {Err(PyErr::from(err))}
    }
}

#[pyfunction]
fn path<'a>(py: Python<'a>,json: &PyAny,path:&str) -> PyResult<PyObject>{
    let json: Value = depythonize(json).unwrap();

    let result = &json.path(path).unwrap();

    match pythonize(py,&result) {
        Ok(ok) => {Ok(ok)}
        Err(err) => {Err(PyErr::from(err))}
    }
}

#[pymodule]
fn jsonpath_rust_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(find_slice))?;
    m.add_wrapped(wrap_pyfunction!(path))?;

    Ok(())
}