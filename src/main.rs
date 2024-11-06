use pest::Parser;
use anyhow::anyhow;
use sql_parser::*;

fn main() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::group_by_clause, "GROUP BY name, surname HAVING SUM(n) >= 0")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    dbg!(pair);
    Ok(())
}