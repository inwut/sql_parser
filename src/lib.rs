use pest::Parser;
use pest_derive::Parser;
use std::fmt;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Debug, Error)]
pub enum SQLError {
    #[error("Failed to parse SQL statement: {0}")]
    ParseError(String),
}

#[derive(Debug)]
pub enum SQLNode {
    AggField { func: Option<String>, identifier: String },
    SelectStmt(Vec<SQLNode>),
    FromClause(String),
    JoinClause { table: String, left: String, right: String },
    WhereClause(Vec<SQLNode>),
    WhereCondition(Vec<SQLNode>),
    GroupByClause(Vec<SQLNode>),
    OrderByClause { identifier: String, order_type: Option<String> },
    LimitClause(String),
    Identifier(String),
    Number(String),
    ComparisonOp(String),
    LogicalOp(String),
}


impl fmt::Display for SQLNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SQLNode::AggField { func, identifier } => {
                if let Some(func_name) = func {
                    writeln!(f, "- agg_field")?;
                    writeln!(f, "    - aggregate_func: {:?}", func_name)?;
                    write!(f, "    - identifier: {:?}", identifier)
                } else {
                    write!(f, "- agg_field > identifier: {:?}", identifier)
                }
            }
            SQLNode::SelectStmt(children) => {
                writeln!(f, "- select_stmt")?;
                for child in children {
                    writeln!(f, "  {}", child)?;
                }
                Ok(())
            }
            SQLNode::FromClause(table) => write!(f, "- from_clause > identifier: {:?}", table),
            SQLNode::JoinClause { table, left, right } => {
                writeln!(f, "- join_clause")?;
                writeln!(f, "    - identifier: {:?}", table)?;
                writeln!(f, "    - identifier: {:?}", left)?;
                write!(f, "    - identifier: {:?}", right)?;
                Ok(())
            }
            SQLNode::WhereClause(conditions) => {
                write!(f, "- where_clause")?;
                for cond in conditions {
                    write!(f, "\n    {}", cond)?;
                }
                Ok(())
            }
            SQLNode::WhereCondition(children) => {
                write!(f, "- where_condition")?;
                for child in children {
                    write!(f, "\n        {}", child)?;
                }
                Ok(())
            }
            SQLNode::GroupByClause(identifiers) => {
                write!(f, "- group_by_clause")?;
                for ident in identifiers {
                    write!(f, "\n    {}", ident)?;
                }
                Ok(())
            }
            SQLNode::OrderByClause { identifier, order_type } => {
                write!(f, "- order_by_clause\n    - identifier: {:?}", identifier)?;
                if let Some(order) = order_type {
                    write!(f, "\n    - order_type: {:?}", order)?;
                }
                Ok(())
            }
            SQLNode::LimitClause(num) => write!(f, "- limit_clause > number: {:?}", num),
            SQLNode::Identifier(ident) => write!(f, "- identifier: {:?}", ident),
            SQLNode::Number(num) => write!(f, "- number: {:?}", num),
            SQLNode::ComparisonOp(op) => write!(f, "- comparison_op: {:?}", op),
            SQLNode::LogicalOp(op) => write!(f, "- logical_op: {:?}", op),
        }
    }
}

pub fn parse_sql(input: &str) -> Result<SQLNode, SQLError> {
    let mut parsed = 
        Grammar::parse(Rule::select_stmt, input).map_err(|e| SQLError::ParseError(e.to_string()))?;
    let mut nodes = Vec::new();
    
    for pair in parsed.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::agg_field => {
                let mut inner_pairs = pair.into_inner();
                let func_pair = inner_pairs.next().unwrap();
                
                let identifier = if func_pair.as_rule() == Rule::aggregate_func {
                    inner_pairs.next().unwrap().as_str().to_string()
                } else {
                    func_pair.as_str().to_string()
                };
                
                let func = if func_pair.as_rule() == Rule::aggregate_func {
                    Some(func_pair.as_str().to_string())
                } else {
                    None
                };

                nodes.push(SQLNode::AggField { func, identifier });
            }
            Rule::from_clause => {
                let table = pair.into_inner().next().unwrap().as_str();
                nodes.push(SQLNode::FromClause(table.to_string()));
            }
            Rule::join_clause => {
                let mut inner_pairs = pair.into_inner();
                let table = inner_pairs.next().unwrap().as_str().to_string();
                let left = inner_pairs.next().unwrap().as_str().to_string();
                let right = inner_pairs.next().unwrap().as_str().to_string();

                nodes.push(SQLNode::JoinClause { table, left, right });
            }
            Rule::where_clause => {
                let mut conditions = Vec::new();
                let mut current_condition = Vec::new();

                for cond in pair.into_inner() {
                    match cond.as_rule() {
                        Rule::where_condition => {
                            let mut inner_cond = cond.into_inner();
                            let identifier = inner_cond.next().unwrap().as_str().to_string();
                            let comparison_op = inner_cond.next().unwrap().as_str().to_string();
                            let value = inner_cond.next().unwrap().as_str().to_string();

                            current_condition.push(SQLNode::Identifier(identifier));
                            current_condition.push(SQLNode::ComparisonOp(comparison_op));
                            if value.parse::<f64>().is_ok() {
                                current_condition.push(SQLNode::Number(value));
                            } else {
                                current_condition.push(SQLNode::Identifier(value));
                            }
                        }
                        Rule::logical_op => {
                            let logical_op = cond.as_str().to_string();
                            conditions.push(SQLNode::WhereCondition(current_condition));
                            conditions.push(SQLNode::LogicalOp(logical_op));
                            current_condition = Vec::new();
                        }
                        _ => {}
                    }
                }
                if !current_condition.is_empty() {
                    conditions.push(SQLNode::WhereCondition(current_condition));
                }

                nodes.push(SQLNode::WhereClause(conditions));
            }
            Rule::group_by_clause => {
                let identifiers = pair.into_inner().map(|ident| {
                    SQLNode::Identifier(ident.as_str().to_string())
                }).collect();
                
                nodes.push(SQLNode::GroupByClause(identifiers));
            }
            Rule::order_by_clause => {
                let mut inner_pairs = pair.into_inner();
                let identifier = inner_pairs.next().unwrap().as_str().to_string();
                let order_type = inner_pairs.next().map(|p| p.as_str().to_string());
                nodes.push(SQLNode::OrderByClause { identifier, order_type });
            }
            Rule::limit_clause => {
                let limit_num = pair.into_inner().next().unwrap().as_str().to_string();
                nodes.push(SQLNode::LimitClause(limit_num));
            }
            _ => {}
        }
    }

    Ok(SQLNode::SelectStmt(nodes))
}
