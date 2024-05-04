use crate::{backend::SchemaBuilder, types::*, SchemaStatementBuilder};

/// Drop a view
///
/// # Examples
///
/// ```
/// ```
#[derive(Default, Debug, Clone)]
pub struct ViewDropStatement {
    pub(crate) views: Vec<TableRef>,
    pub(crate) options: Vec<ViewDropOpt>,
    pub(crate) if_exists: bool,
}

/// All available view drop options.
#[derive(Debug, Clone)]
pub enum ViewDropOpt {
    Restrict,
    Cascade,
}

impl ViewDropStatement {
    /// Construct drop view statement.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set view name.
    pub fn view<T: 'static>(&mut self, view: T) -> &mut Self
    where
        T: IntoTableRef,
    {
        self.views.push(view.into_table_ref());
        self
    }

    /// Drop view if exists.
    pub fn if_exists(&mut self) -> &mut Self {
        self.if_exists = true;
        self
    }

    /// Drop option restrict.
    pub fn restrict(&mut self) -> &mut Self {
        self.options.push(ViewDropOpt::Restrict);
        self
    }

    /// Drop option cacade.
    pub fn cascade(&mut self) -> &mut Self {
        self.options.push(ViewDropOpt::Cascade);
        self
    }

    pub fn take(&mut self) -> Self {
        Self {
            views: std::mem::take(&mut self.views),
            options: std::mem::take(&mut self.options),
            if_exists: self.if_exists,
        }
    }
}

impl SchemaStatementBuilder for ViewDropStatement {
    fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        let mut sql = String::with_capacity(256);
        schema_builder.prepare_view_drop_statement(self, &mut sql);
        sql
    }

    fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String {
        let mut sql = String::with_capacity(256);
        schema_builder.prepare_view_drop_statement(self, &mut sql);
        sql
    }
}