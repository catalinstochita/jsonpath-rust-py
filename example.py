from jsonpath_rust_py import find_slice, path

json = {"field": [{"f": 1}, {"f": 0}, {"f": 3}]}
select = "$.field[?(!(@.f == 0))]"

res = find_slice(json, select)
print(res)


select = "$..field[?(@.f == 1)].f"
res = path(json, select)
print(res)
