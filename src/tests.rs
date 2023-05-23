#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use jsonpath_rust::{JsonPathFinder, JsonPathInst, JsonPathQuery};
    use jsonpath_rust::JsonPathValue::Slice;
    use serde_json::{json, Value};

    #[test]
    fn find_slice_test() {
        let json: Box<Value> = Box::new(json!({
            "field":[{"f":1},{"f":0},{"f":3}]
        }));
        let path: Box<JsonPathInst> = Box::from(
            JsonPathInst::from_str("$.field[?(!(@.f == 0))]").expect("the path is correct"),
        );
        let finder = JsonPathFinder::new(json, path);
        let result = finder.find_slice();
        assert_eq!(result, vec![Slice(&json!({"f":1})),Slice(&json!({"f":3}))]);
    }

    #[test]
    fn path_test(){
        let json: Value = json!({
            "field":[{"f":1},{"f":0},{"f":3}]
        });
        let result = &json.path("$..field[?(@.f == 1)].f").unwrap();

        assert_eq!(result, &json!([1]));
    }
}