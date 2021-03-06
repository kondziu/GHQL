use glob;

#[derive(PartialEq,Debug,Clone)]
pub struct Query {
    //assignments: Vector<Assignment>
    pub expressions: Expressions,
}

#[derive(PartialEq,Debug,Clone)]
pub struct Expressions {
    pub head: Expression,
    pub tail: Vec<(Connective, Expression)>,
}

#[derive(PartialEq,Debug,Clone)]
pub enum Operand {
    Feature(Feature),
    Number(i64),
}

#[derive(PartialEq,Debug,Clone)]
pub enum Expression {
    Simple(Feature),
    Compound { operator: RelationalOperator, left: Operand, right: Operand },
}

#[derive(PartialEq,Debug,Copy,Clone)]
pub enum Connective {
    Conjunction,
    //Disjunction
}

#[derive(PartialEq,Debug,Clone)]
pub enum Feature {
    Commits   { parameters: Vec<Parameter>, property: Option<Property> },
    Additions { parameters: Vec<Parameter>, property: Option<Property> },
    Deletions { parameters: Vec<Parameter>, property: Option<Property> },
    Changes   { parameters: Vec<Parameter>, property: Option<Property> },
}

#[derive(PartialEq,Debug,Clone)]
pub enum Parameter {
    Path { operator: StringOperator, pattern: glob::Pattern }
}

#[derive(PartialEq,Debug,Copy,Clone)]
pub enum Property {
    ElapsedTime,
    //Percentile(u32),
}

#[derive(PartialEq,Debug,Copy,Clone)]
pub enum StringOperator {
    Equal,
    Different,
}

#[derive(PartialEq,Debug,Copy,Clone)]
pub enum RelationalOperator {
    Equal,
    Different,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl Query {
    pub fn simple (expressions: Expressions) -> Query {
        Query {expressions}
    }
}

impl Expressions {
    pub fn new (head: Expression, tail: Vec<(Connective, Expression)>) -> Expressions {
        Expressions { head, tail }
    }
    pub fn from_expression (head: Expression) -> Expressions {
        Expressions { head, tail: Vec::new() }
    }
    pub fn from_expressions (connective: Connective, mut expressions: Vec<Expression>) -> Result<Expressions, String> {
        if expressions.len() < 1 {
            return Err("At least one feature must be provided.".to_string());
        }

        let head: Expression = expressions.remove(0);
        let tail: Vec<(Connective, Expression)> =
            expressions.into_iter().map(|expression| (connective, expression)).collect();

        return Ok(Expressions { head, tail })
    }
}

impl Expression {
    pub fn from_feature(feature: Feature) -> Expression {
        Expression::Simple(feature)
    }
    pub fn new(operator: RelationalOperator, left: Operand, right: Operand) -> Expression {
        Expression::Compound { operator, left, right }
    }
}

impl Operand {
    pub fn from_feature(feature: Feature) -> Operand {
        Operand::Feature(feature)
    }
    pub fn from_number(number: i64) -> Operand {
        Operand::Number(number)
    }
}

impl Feature {
    pub fn commits_simple() -> Feature {
        Feature::Commits { parameters: Vec::new(), property: None }
    }
    pub fn commits_with_parameter(parameter: Parameter) -> Feature {
        Feature::Commits { parameters: vec![parameter], property: None }
    }
    pub fn commits_with_parameters(parameters: Vec<Parameter>) -> Feature {
        Feature::Commits { parameters, property: None }
    }
    pub fn commits_with_property(property: Property) -> Feature {
        Feature::Commits { parameters: Vec::new(), property: Some(property) }
    }
    pub fn commits(parameters: Vec<Parameter>, property: Property) -> Feature {
        Feature::Commits { parameters, property: Some(property) }
    }

    pub fn additions_simple() -> Feature {
        Feature::Additions { parameters: Vec::new(), property: None }
    }
    pub fn additions_with_parameter(parameter: Parameter) -> Feature {
        Feature::Additions { parameters: vec![parameter], property: None }
    }
    pub fn additions_with_parameters(parameters: Vec<Parameter>) -> Feature {
        Feature::Additions { parameters, property: None }
    }
    pub fn additions_with_property(property: Property) -> Feature {
        Feature::Additions { parameters: Vec::new(), property: Some(property) }
    }
    pub fn additions(parameters: Vec<Parameter>, property: Property) -> Feature {
        Feature::Additions { parameters, property: Some(property) }
    }

    pub fn deletions_simple() -> Feature {
        Feature::Deletions { parameters: Vec::new(), property: None }
    }
    pub fn deletions_with_parameter(parameter: Parameter) -> Feature {
        Feature::Deletions { parameters: vec![parameter], property: None }
    }
    pub fn deletions_with_parameters(parameters: Vec<Parameter>) -> Feature {
        Feature::Deletions { parameters, property: None }
    }
    pub fn deletions_with_property(property: Property) -> Feature {
        Feature::Deletions { parameters: Vec::new(), property: Some(property) }
    }
    pub fn deletions(parameters: Vec<Parameter>, property: Property) -> Feature {
        Feature::Deletions { parameters, property: Some(property) }
    }

    pub fn changes_simple() -> Feature {
        Feature::Changes { parameters: Vec::new(), property: None }
    }
    pub fn changes_with_parameter(parameter: Parameter) -> Feature {
        Feature::Changes { parameters: vec![parameter], property: None }
    }
    pub fn changes_with_parameters(parameters: Vec<Parameter>) -> Feature {
        Feature::Changes { parameters, property: None }
    }
    pub fn changes_with_property(property: Property) -> Feature {
        Feature::Changes { parameters: Vec::new(), property: Some(property) }
    }
    pub fn changes(parameters: Vec<Parameter>, property: Property) -> Feature {
        Feature::Changes { parameters, property: Some(property) }
    }
}

impl Parameter {
    pub fn path_equal(pattern: String) -> Parameter {
        Parameter::Path {
            operator: StringOperator::Equal,
            pattern: glob::Pattern::new(&pattern).unwrap()
        }
    }
    pub fn path_different(pattern: String) -> Parameter {
        Parameter::Path {
            operator: StringOperator::Different,
            pattern: glob::Pattern::new(&pattern).unwrap()
        }
    }
    pub fn path_equal_str(pattern: &str) -> Parameter {
        Parameter::Path {
            operator: StringOperator::Equal,
            pattern: glob::Pattern::new(pattern).unwrap()
        }
    }
    pub fn path_different_str(pattern: &str) -> Parameter {
        Parameter::Path {
            operator: StringOperator::Different,
            pattern: glob::Pattern::new(pattern).unwrap()
        }
    }
}

#[macro_export]
macro_rules! compose_parameters {
    ( $head: expr, $tail: expr ) => {{
        let mut parameters = Vec::new();
        parameters.push($head);
        let clean_tail: Vec<Parameter> = $tail.iter().map(|e| e.1.clone()).collect();
        parameters.extend(clean_tail);
        parameters
    }}
}
