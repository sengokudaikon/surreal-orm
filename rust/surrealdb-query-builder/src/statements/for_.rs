use std::fmt::{self, Display};

use crate::{
    binding::{BindingsList, Parametric},
    filter::{Conditional, Filter},
    sql::{Buildable, Queryable, RawStatement, ToRawStatement},
    Erroneous,
};

#[derive(Clone, Copy, Debug)]
pub enum ForCrudType {
    Create,
    Select,
    Update,
    Delete,
}

impl Display for ForCrudType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let crud_type = match self {
            ForCrudType::Create => "create",
            ForCrudType::Select => "select",
            ForCrudType::Update => "update",
            ForCrudType::Delete => "delete",
        };
        write!(f, "{}", crud_type)
    }
}

#[derive(Clone, Debug)]
struct ForData {
    crud_types: Vec<ForCrudType>,
    condition: Option<Filter>,
    bindings: BindingsList,
}

impl Erroneous for For {}
impl Queryable for For {}

impl Parametric for For {
    fn get_bindings(&self) -> BindingsList {
        self.0.bindings.to_vec()
    }
}

#[derive(Clone, Debug)]
pub enum ForArgs {
    ForOption(ForCrudType),
    ForOptions(Vec<ForCrudType>),
}
impl From<ForCrudType> for ForArgs {
    fn from(value: ForCrudType) -> Self {
        Self::ForOption(value)
    }
}

impl From<Vec<ForCrudType>> for ForArgs {
    fn from(value: Vec<ForCrudType>) -> Self {
        Self::ForOptions(value)
    }
}
impl From<ForArgs> for Vec<ForCrudType> {
    fn from(value: ForArgs) -> Self {
        match value {
            ForArgs::ForOption(one) => vec![one],
            ForArgs::ForOptions(many) => many,
        }
    }
}

impl<'a, const N: usize> From<&[ForCrudType; N]> for ForArgs {
    fn from(value: &[ForCrudType; N]) -> Self {
        Self::ForOptions(value.to_vec())
    }
}

pub struct ForStart(ForData);

impl ForStart {
    pub fn where_(mut self, condition: impl Conditional) -> For {
        let condition = Filter::new(condition);
        self.0.condition = Some(condition.clone());
        self.0.bindings.extend(condition.get_bindings());
        For(self.0)
    }
}

pub fn for_(for_crud_types: impl Into<ForArgs>) -> ForStart {
    ForStart(ForData {
        crud_types: for_crud_types.into().into(),
        condition: None,
        bindings: vec![],
    })
}

#[derive(Clone, Debug)]
pub struct For(ForData);

impl Buildable for For {
    fn build(&self) -> String {
        let mut query = format!("FOR");
        if !&self.0.crud_types.is_empty() {
            query = format!(
                "{query} {}",
                &self
                    .0
                    .crud_types
                    .iter()
                    .map(|ct| {
                        let ct = ct.to_string();
                        ct
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        if let Some(cond) = &self.0.condition {
            query = format!("{query}\n\tWHERE {cond}");
        }
        query
    }
}

impl Display for For {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

pub struct NONE;

#[derive(Clone)]
pub enum PermissionForables {
    For(For),
    Fors(Vec<For>),
    RawStatement(RawStatement),
    RawStatementList(Vec<RawStatement>),
}

impl ToRawStatement for PermissionForables {
    fn to_raw(self) -> RawStatement {
        match self {
            PermissionForables::For(for_one) => for_one.to_raw(),
            PermissionForables::Fors(for_many) => RawStatement::new(
                for_many
                    .into_iter()
                    .map(|f| f.to_raw().to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            PermissionForables::RawStatement(r) => r,
            PermissionForables::RawStatementList(raw_list) => RawStatement::new(
                raw_list
                    .into_iter()
                    .map(|f| f.to_raw().to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        }
    }
}

impl From<For> for PermissionForables {
    fn from(value: For) -> Self {
        Self::For(value)
    }
}

impl From<Vec<For>> for PermissionForables {
    fn from(value: Vec<For>) -> Self {
        Self::Fors(value)
    }
}

impl<'a, const N: usize> From<&[For; N]> for PermissionForables {
    fn from(value: &[For; N]) -> Self {
        Self::Fors(value.to_vec())
    }
}

impl From<RawStatement> for PermissionForables {
    fn from(value: RawStatement) -> Self {
        Self::RawStatement(value)
    }
}

impl From<Vec<RawStatement>> for PermissionForables {
    fn from(value: Vec<RawStatement>) -> Self {
        Self::RawStatementList(value)
    }
}

impl From<&Vec<RawStatement>> for PermissionForables {
    fn from(value: &Vec<RawStatement>) -> Self {
        Self::RawStatementList(value.to_vec())
    }
}

impl<'a, const N: usize> From<&[RawStatement; N]> for PermissionForables {
    fn from(value: &[RawStatement; N]) -> Self {
        Self::RawStatementList(value.to_vec())
    }
}

#[cfg(test)]
#[cfg(feature = "mock")]
mod tests {

    use super::*;
    use std::time::Duration;

    use crate::{
        statements::{order, select},
        Field, Operatable,
    };

    use super::*;

    #[test]
    fn test_define_for_statement_state_machine() {
        let name = Field::new("name");

        let for_res = for_(ForCrudType::Create).where_(name.like("Oyelowo"));
        assert_eq!(
            for_res.to_string(),
            "FOR create\n\tWHERE name ~ $_param_00000000".to_string()
        );
        insta::assert_display_snapshot!(for_res);
        insta::assert_debug_snapshot!(for_res.get_bindings());
    }

    #[test]
    fn test_define_for_statement_state_machine_multiple() {
        use ForCrudType::*;
        let name = Field::new("name");

        let for_res = for_(&[Create, Delete, Select, Update]).where_(name.is("Oyedayo"));
        assert_eq!(
            for_res.to_string(),
            "FOR create, delete, select, update\n\tWHERE name IS $_param_00000000".to_string()
        );
        insta::assert_display_snapshot!(for_res);
        insta::assert_debug_snapshot!(for_res.get_bindings());
    }
}
