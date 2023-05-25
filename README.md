# jsonpath-rust-py

## The jsonpath description

### Functions

#### Size

A function `length()` transforms the output of the filtered expression into a size of this element
It works with arrays, therefore it returns a length of a given array, otherwise null.

`$.some_field.length()`

**To use it** for objects, the operator `[*]` can be used.
`$.object.[*].length()`

### Operators

| Operator                   | Description                                                                                                                                                  | Where to use                                                                                                                                |
|----------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------|
| `$`                        | Pointer to the root of the json.                                                                                                                             | It is gently advising to start every jsonpath from the root. Also, inside the filters to point out that the path is starting from the root. |
| `@`                        | Pointer to the current element inside the filter operations.                                                                                                 | It is used inside the filter operations to iterate the collection.                                                                          |
| `*` or `[*]`               | Wildcard. It brings to the list all objects and elements regardless their names.                                                                             | It is analogue a flatmap operation.                                                                                                         |
| `<..>`                     | Descent operation. It brings to the list all objects, children of that objects and etc                                                                       | It is analogue a flatmap operation.                                                                                                         |
| `.<name>` or `.['<name>']` | the key pointing to the field of the object                                                                                                                  | It is used to obtain the specific field.                                                                                                    |
| `['<name>' (, '<name>')]`  | the list of keys                                                                                                                                             | the same usage as for a single key but for list                                                                                             |
| `[<number>]`               | the filter getting the element by its index.                                                                                                                 |                                                                                                                                             |
| `[<number> (, <number>)]`  | the list if elements of array according to their indexes representing these numbers.                                                                         |                                                                                                                                             |
| `[<start>:<end>:<step>]`   | slice operator to get a list of element operating with their indexes. By default step = 1, start = 0, end = array len. The elements can be omitted ```[:]``` |                                                                                                                                             |
| `[?(<expression>)]`        | the logical expression to filter elements in the list.                                                                                                       | It is used with arrays preliminary.                                                                                                         |

### Filter expressions

The expressions appear in the filter operator like that `[?(@.len > 0)]`. The expression in general consists of the
following elements:

- Left and right operands, that is ,in turn, can be a static value,representing as a primitive type like a number,
  string value `'value'`, array of them or another json path instance.
- Expression sign, denoting what action can be performed

| Expression sign | Description                                                                                | Where to use                                                                                             |
|-----------------|--------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------|
| `!`             | Logical Not                                                                                | To invert filters                                                                                        |
| `==`            | Equal                                                                                      | To compare numbers or string literals                                                                    |
| `!=`            | Unequal                                                                                    | To compare numbers or string literals in opposite way to equals                                          |
| `<`             | Less                                                                                       | To compare numbers                                                                                       |
| `>`             | Greater                                                                                    | To compare numbers                                                                                       |
| `<=`            | Less or equal                                                                              | To compare numbers                                                                                       |
| `>=`            | Greater or equal                                                                           | To compare numbers                                                                                       |
| `~=`            | Regular expression                                                                         | To find the incoming right side in the left side.                                                        |
| `in`            | Find left element in the list of right elements.                                           |                                                                                                          |
| `nin`           | The same one as saying above but carrying the opposite sense.                              |                                                                                                          |
| `size`          | The size of array on the left size should be corresponded to the number on the right side. |                                                                                                          |
| `noneOf`        | The left size has no intersection with right                                               |                                                                                                          |
| `anyOf`         | The left size has at least one intersection with right                                     |                                                                                                          |
| `subsetOf`      | The left is a subset of the right side                                                     |                                                                                                          |
|                 | Exists operator.                                                                           | The operator checks the existence of the field depicted on the left side like that `[?(@.key.isActive)]` |

Filter expressions can be chained using `||` and `&&` (logical or and logical and correspondingly) in the following way:

```json
          {
  "key": [
    {
      "city": "London",
      "capital": true,
      "size": "big"
    },
    {
      "city": "Berlin",
      "capital": true,
      "size": "big"
    },
    {
      "city": "Tokyo",
      "capital": true,
      "size": "big"
    },
    {
      "city": "Moscow",
      "capital": true,
      "size": "big"
    },
    {
      "city": "Athlon",
      "capital": false,
      "size": "small"
    },
    {
      "city": "Dortmund",
      "capital": false,
      "size": "big"
    },
    {
      "city": "Dublin",
      "capital": true,
      "size": "small"
    }
  ]
}
```

The path ``` $.key[?(@.capital == false || @size == 'small')].city ``` will give the following result:

```json
[
  "Athlon",
  "Dublin",
  "Dortmund"
]
```

And the path ``` $.key[?(@.capital == false && @size != 'small')].city ``` ,in its turn, will give the following result:

```json
[
  "Dortmund"
]
```

By default, the operators have the different priority so `&&` has a higher priority so to change it the brackets can be
used.
``` $.[?((@.f == 0 || @.f == 1) && ($.x == 15))].city ```

## Examples

Given the json

 ```json
 {
  "store": {
    "book": [
      {
        "category": "reference",
        "author": "Nigel Rees",
        "title": "Sayings of the Century",
        "price": 8.95
      },
      {
        "category": "fiction",
        "author": "Evelyn Waugh",
        "title": "Sword of Honour",
        "price": 12.99
      },
      {
        "category": "fiction",
        "author": "Herman Melville",
        "title": "Moby Dick",
        "isbn": "0-553-21311-3",
        "price": 8.99
      },
      {
        "category": "fiction",
        "author": "J. R. R. Tolkien",
        "title": "The Lord of the Rings",
        "isbn": "0-395-19395-8",
        "price": 22.99
      }
    ],
    "bicycle": {
      "color": "red",
      "price": 19.95
    }
  },
  "expensive": 10
}
 ```

| JsonPath                             | Result                                                       |
|--------------------------------------|:-------------------------------------------------------------|
| `$.store.book[*].author`             | The authors of all books                                     |
| `$..book[?(@.isbn)]`                 | All books with an ISBN number                                |
| `$.store.*`                          | All things, both books and bicycles                          |
| `$..author`                          | All authors                                                  |
| `$.store..price`                     | The price of everything                                      |
| `$..book[2]`                         | The third book                                               |
| `$..book[-2]`                        | The second to last book                                      |
| `$..book[0,1]`                       | The first two books                                          |
| `$..book[:2]`                        | All books from index 0 (inclusive) until index 2 (exclusive) |
| `$..book[1:2]`                       | All books from index 1 (inclusive) until index 2 (exclusive) |
| `$..book[-2:]`                       | Last two books                                               |
| `$..book[2:]`                        | Book number two from tail                                    |
| `$.store.book[?(@.price < 10)]`      | All books in store cheaper than 10                           |
| `$..book[?(@.price <= $.expensive)]` | All books in store that are not "expensive"                  |
| `$..book[?(@.author ~= /.*REES/i)]`  | All books matching regex (ignore case)                       |
| `$..*`                               | Give me every thing                                          |

## The library

The library intends to provide the basic functionality for ability to find the slices of data using the syntax, by calling the functions written in rust.

```python
from jsonpath_rust_py import find_slice, path

json = {"field": [{"f": 1}, {"f": 0}, {"f": 3}]}
select = "$.field[?(!(@.f == 0))]"

res = find_slice(json, select)
print(res)


select = "$..field[?(@.f == 1)].f"
res = path(json, select)
print(res)
```

For now we only support find_slice and path

## Enhanced Functionality
This library incorporates features from the Rust library [jsonpath-rust](https://github.com/catalinstochita/jsonpath-rust), which in turn, is an enhanced fork of the original [jsonpath-rust](https://github.com/besok/jsonpath-rust) developed by Besok.

Key improvements in the forked version include:
- Implementation of logical 'Not' (!) operator for filters
