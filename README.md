## sql_query_parser

### Overview

The `sql_query_parser` project provides a custom Rust parser for SQL-like queries, implemented using the Pest crate. It can parse SELECT statements with advanced query capabilities, including joins, conditional filtering, aggregate functions, grouping, ordering, and limiting the results.

### Technical Description

The parser processes SQL-like queries by breaking down the input string into a series of tokens, recognizing various syntax elements such as keywords, identifiers (table or column names), operators, and literals (numbers and strings). Each of these elements is mapped to a specific rule in the grammar, which is defined using regular expressions in the pest parser syntax. The grammar specifies how tokens can be combined to form valid SQL-like statements. It is implemented as a set of grammar rules that correspond to different parts of the SQL query, such as `SELECT`, `FROM`, `WHERE`, `JOIN`, `GROUP BY`, etc.

The parser works by recursively applying these rules to build a parse tree (AST). The tree structure represents the relationships between the components of the query and contains detailed information about selected fields, target tables, filtering conditions, sorting directions etc. The AST can be used in various ways, such as further processing and integration with other systems for **query execution**, **data retrieval** and **output formatting** and working with databases in general. 

The `sql_query_parser` processes base SQL-like clauses, specifically:

* **SELECT clause**: Allows selecting fields or using aggregate functions (`COUNT`, `SUM`, `AVG`, `MAX`, `MIN`) on them.
* **FROM and JOIN clauses**: Defines data sources and relationships between them using `JOIN` and `ON` with `=` condition on fields.
* **WHERE clause**: Filters records with conditions on fields, supporting comparison (`=`, `!=`, `<`, `>`, `<=`, `>=`) and logical (`AND`, `OR`) operators.
* **GROUP BY clause**: Enables grouping of results by multiple fields.
* **ORDER BY clause**: Orders results based on specified fields, supporting ascending (`ASC`) and descending (`DESC`) sorting.
* **LIMIT clause**: Limits the number of returned results.

After parsing, each node of the AST is displayed in a custom format defined in the `fmt::Display` implementation, which organizes the output in a clear, indented format. It can be useful for:

* Analyzing query structure and understanding the components of the SQL query.
* Identifying key parts of the query for optimization decisions.
* Providing a clear structure to easily check for syntax errors in SQL queries.

### Example Input

``` sql
SELECT name, SUM(sales)
FROM products
WHERE quantity >= 50 AND producer = "Producer"
GROUP BY category, price
ORDER BY sales DESC
LIMIT 20; 
```
### Example Output

```
- select_stmt
  - agg_field > identifier: "name"
  - agg_field
    - aggregate_func: "SUM"
    - identifier: "sales"
  - from_clause > identifier: "products"
  - where_clause
    - where_condition
        - identifier: "quantity"
        - comparison_op: ">="
        - number: "50"
    - logical_op: "AND"
    - where_condition
        - identifier: "producer"
        - comparison_op: "="
        - identifier: "\"Producer\""
  - group_by_clause
    - identifier: "category"
    - identifier: "price"
  - order_by_clause
    - identifier: "sales"
    - order_type: "DESC"
  - limit_clause > number: "20"
```

### Grammar Rules

```
reserved_keyword = {
    "SELECT" | "FROM" | "JOIN" | "ON" | "WHERE" | "GROUP BY" |
    "ORDER BY" | "LIMIT" | "AND" | "OR" |
    "COUNT" | "SUM" | "AVG" | "MAX" | "MIN" |
    "ASC" | "DESC"
}

WHITESPACE     = _{ " " | "\t" | "\n" }
identifier     = @{ !reserved_keyword ~ ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "_")* }
number         =  @{ ASCII_DIGIT+ }
string         =  { "\"" ~ (!"\"" ~ ANY)* ~ "\"" }
comparison_op  =  { "!=" | ">=" | "<=" |">" | "<" | "=" }
logical_op     =  { "AND" | "OR" }
aggregate_func =  { "COUNT" | "SUM" | "AVG" | "MAX" | "MIN" }
order_type     =  { "ASC" | "DESC"}

select_stmt = { "SELECT" ~ (agg_field ~ ("," ~ agg_field)* ~ ("," ~ identifier)*)
~ from_clause
~ (join_clause)? 
~ (where_clause)?
~ (group_by_clause)? 
~ (order_by_clause)? 
~ (limit_clause)? 
~ ";" }

agg_field = { aggregate_func ~ "(" ~ identifier ~ ")" | identifier }
where_condition = { identifier ~ comparison_op ~ (number | string) }

from_clause = { "FROM" ~ identifier }
join_clause = { "JOIN" ~ identifier ~ "ON" ~ identifier ~ "=" ~ identifier }
where_clause = { "WHERE" ~ where_condition ~ (logical_op ~ where_condition)* }
group_by_clause = { "GROUP BY" ~ identifier ~ ("," ~ identifier)* }
order_by_clause = { "ORDER BY" ~ identifier ~ (order_type)? ~ ("," ~ identifier ~ (order_type)?)* }
limit_clause = { "LIMIT" ~ number }
```