use pest::Parser;
use anyhow::anyhow;
use sql_parser::*;

fn main() -> anyhow::Result<()> {
    let query = "SELECT b FROM table;";
    let pair = Grammar::parse(Rule::select_stmt, query)?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    dbg!(pair);
    Ok(())
}