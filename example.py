from jsonpath_rust_py import find_slice

json = {"field": [{"f": 1}, {"f": 0}, {"f": 3}]}
path = "$.field[?(!(@.f == 0))]"

res = find_slice(json, path)
print(res)