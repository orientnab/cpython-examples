use cpython::{py_fn, py_module_initializer, PyList, PySet, PyResult, Python};

py_module_initializer!(libmyrustlib, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(
        py,
        "sum_as_string",
        py_fn!(py, sum_as_string_py(a: i64, b: i64)),
    )?;
    m.add(py, "length_list", py_fn!(py, length_list(list: PyList)))?;
    m.add(
        py,
        "length_list_rust",
        py_fn!(py, length_list_rust(list: Vec<i64>)),
    )?;
    m.add(py, "sort_list", py_fn!(py, sort_list(list: Vec<i64>)))?;
    m.add(py, "list_to_set", py_fn!(py, list_to_set(list: PyList)))?;
    Ok(())
});

// logic implemented as a normal rust function
fn sum_as_string(a: i64, b: i64) -> String {
    format!("{}", a + b).to_string()
}

fn sum_as_string_py(_: Python, a: i64, b: i64) -> PyResult<String> {
    let out = sum_as_string(a, b);
    Ok(out)
}

fn length_list(py: Python, list: PyList) -> PyResult<usize> {
    let length = list.len(py);
    Ok(length)
}

fn length_list_rust(_: Python, list: Vec<i64>) -> PyResult<usize> {
    let length = list.len();
    Ok(length)
}

fn sort_list(_: Python, mut list: Vec<i64>) -> PyResult<Vec<i64>> {
    list.sort();
    Ok(list)
}

fn list_to_set(py: Python, list: PyList) -> PyResult<PySet> {
    PySet::new(py, list)
}
