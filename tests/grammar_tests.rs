use anyhow::anyhow;
use pest::Parser;
use sql_query_parser::*;

#[test]
fn test_reserved_keyword() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::reserved_keyword, "ORDER BY")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "ORDER BY");

    let pair = Grammar::parse(Rule::reserved_keyword, "ASC")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "ASC");

    let pair = Grammar::parse(Rule::reserved_keyword, "SELECT")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "SELECT");

    let pair = Grammar::parse(Rule::reserved_keyword, "GROUP BY")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "GROUP BY");

    let pair = Grammar::parse(Rule::reserved_keyword, "id");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_whitespace() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::WHITESPACE, " ");
    assert!(pair.is_ok());

    let pair = Grammar::parse(Rule::WHITESPACE, "\t");
    assert!(pair.is_ok());

    let pair = Grammar::parse(Rule::WHITESPACE, "\n");
    assert!(pair.is_ok());

    let pair = Grammar::parse(Rule::WHITESPACE, "\r\n");
    assert!(pair.is_ok());

    Grammar::parse(Rule::select_stmt, "SELECT a\r\nFROM b;")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    Grammar::parse(Rule::select_stmt, "SELECT a\nFROM b;")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    Grammar::parse(Rule::select_stmt, "SELECT a\tFROM b;")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    Ok(())
}

#[test]
fn test_identifier() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::identifier, "id")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "id");

    let pair = Grammar::parse(Rule::identifier, "id_1")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "id_1");

    let pair = Grammar::parse(Rule::identifier, "1a");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::identifier, "SELECT");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_number() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::number, "13")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "13");

    let pair = Grammar::parse(Rule::number, "a");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_string() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::string, "\"str\"")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "\"str\"");

    let pair = Grammar::parse(Rule::string, "str");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_comparison_op() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::comparison_op, "<=")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "<=");

    let pair = Grammar::parse(Rule::comparison_op, "+");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_logical_op() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::logical_op, "AND")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "AND");

    let pair = Grammar::parse(Rule::logical_op, "NO");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_aggregate_func() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::aggregate_func, "MAX")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "MAX");

    let pair = Grammar::parse(Rule::aggregate_func, "STR");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_order_type() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::order_type, "DESC")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "DESC");

    let pair = Grammar::parse(Rule::order_type, "ORDER");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_agg_field() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::agg_field, "COUNT(id)")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "COUNT(id)");

    let pair = Grammar::parse(Rule::agg_field, "id")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "id");

    let pair = Grammar::parse(Rule::agg_field, "()");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_where_condition() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::where_condition, "n < 56")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "n < 56");

    let pair = Grammar::parse(Rule::where_condition, "city = \"Kyiv\"")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "city = \"Kyiv\"");

    let pair = Grammar::parse(Rule::where_condition, "n =");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_from_clause() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::from_clause, "FROM table_name")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "FROM table_name");

    let pair = Grammar::parse(Rule::from_clause, "FROM");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_join_clause() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::join_clause, "JOIN table ON id1 = id2")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "JOIN table ON id1 = id2");

    let pair = Grammar::parse(Rule::join_clause, "JOIN table");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_where_clause() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::where_clause, "WHERE id = 45")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "WHERE id = 45");

    let pair = Grammar::parse(Rule::where_clause, "WHERE id = 45 AND a > 56")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "WHERE id = 45 AND a > 56");

    let pair = Grammar::parse(
        Rule::where_clause,
        "WHERE id = 45 AND a > 56 OR b = \"str\"",
    )?
    .next()
    .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "WHERE id = 45 AND a > 56 OR b = \"str\"");

    let pair = Grammar::parse(Rule::where_clause, "WHERE id");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_group_by_clause() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::group_by_clause, "GROUP BY name")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "GROUP BY name");

    let pair = Grammar::parse(Rule::group_by_clause, "GROUP BY name, surname")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "GROUP BY name, surname");

    let pair = Grammar::parse(Rule::group_by_clause, "GROUP name");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_order_by_clause() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::order_by_clause, "ORDER BY name")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "ORDER BY name");

    let pair = Grammar::parse(Rule::order_by_clause, "ORDER BY name DESC")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "ORDER BY name DESC");

    let pair = Grammar::parse(Rule::order_by_clause, "ORDER BY ASC");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_limit_clause() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::limit_clause, "LIMIT 10")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "LIMIT 10");

    let pair = Grammar::parse(Rule::limit_clause, "LIMIT num");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_select_stmt() -> anyhow::Result<()> {
    let query = "SELECT COUNT(name), surname FROM users;";
    let pair = Grammar::parse(Rule::select_stmt, query)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), query);

    let query = "SELECT COUNT(name), surname FROM users WHERE a = 5 OR b > 67;";
    let pair = Grammar::parse(Rule::select_stmt, query)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), query);

    let query =
        "SELECT name, SUM(sales) FROM products GROUP BY category, price ORDER BY sales DESC;";
    let pair = Grammar::parse(Rule::select_stmt, query)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), query);

    let query = "SELECT name FROM employees WHERE age > 30 GROUP BY department ORDER BY salary ASC LIMIT 10;";
    let pair = Grammar::parse(Rule::select_stmt, query)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), query);

    Ok(())
}
