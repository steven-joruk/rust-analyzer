//! Provides set of implementation for hir's objects that allows get back location in file.

use either::Either;
use hir_def::{
    nameres::{ModuleOrigin, ModuleSource},
    src::{HasChildSource, HasSource as _},
    Lookup, VariantId,
};
use hir_expand::InFile;
use syntax::ast;

use crate::{
    db::HirDatabase, Adt, Const, Enum, Field, FieldSource, Function, Impl, LifetimeParam, MacroDef,
    Module, Static, Struct, Trait, TypeAlias, TypeOrConstParam, Union, Variant,
};

pub trait HasSource {
    type Ast;
    /// Fetches the definition's source node.
    /// Using [`crate::Semantics::source`] is preferred when working with [`crate::Semantics`],
    /// as that caches the parsed file in the semantics' cache.
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>;
}

/// NB: Module is !HasSource, because it has two source nodes at the same time:
/// definition and declaration.
impl Module {
    /// Returns a node which defines this module. That is, a file or a `mod foo {}` with items.
    pub fn definition_source(self, db: &dyn HirDatabase) -> InFile<ModuleSource> {
        let def_map = self.id.def_map(db.upcast());
        def_map[self.id.local_id].definition_source(db.upcast())
    }

    pub fn is_mod_rs(self, db: &dyn HirDatabase) -> bool {
        let def_map = self.id.def_map(db.upcast());
        match def_map[self.id.local_id].origin {
            ModuleOrigin::File { is_mod_rs, .. } => is_mod_rs,
            _ => false,
        }
    }

    /// Returns a node which declares this module, either a `mod foo;` or a `mod foo {}`.
    /// `None` for the crate root.
    pub fn declaration_source(self, db: &dyn HirDatabase) -> Option<InFile<ast::Module>> {
        let def_map = self.id.def_map(db.upcast());
        def_map[self.id.local_id].declaration_source(db.upcast())
    }
}

impl HasSource for Field {
    type Ast = FieldSource;
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>> {
        let var = VariantId::from(self.parent);
        let src = var.child_source(db.upcast());
        let field_source = src.map(|it| match it[self.id].clone() {
            Either::Left(it) => FieldSource::Pos(it),
            Either::Right(it) => FieldSource::Named(it),
        });
        Some(field_source)
    }
}
impl HasSource for Adt {
    type Ast = ast::Adt;
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>> {
        match self {
            Adt::Struct(s) => Some(s.source(db)?.map(ast::Adt::Struct)),
            Adt::Union(u) => Some(u.source(db)?.map(ast::Adt::Union)),
            Adt::Enum(e) => Some(e.source(db)?.map(ast::Adt::Enum)),
        }
    }
}
impl HasSource for Struct {
    type Ast = ast::Struct;
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db.upcast()).source(db.upcast()))
    }
}
impl HasSource for Union {
    type Ast = ast::Union;
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db.upcast()).source(db.upcast()))
    }
}
impl HasSource for Enum {
    type Ast = ast::Enum;
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db.upcast()).source(db.upcast()))
    }
}
impl HasSource for Variant {
    type Ast = ast::Variant;
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<ast::Variant>> {
        Some(self.parent.id.child_source(db.upcast()).map(|map| map[self.id].clone()))
    }
}
impl HasSource for Function {
    type Ast = ast::Fn;
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db.upcast()).source(db.upcast()))
    }
}
impl HasSource for Const {
    type Ast = ast::Const;
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db.upcast()).source(db.upcast()))
    }
}
impl HasSource for Static {
    type Ast = ast::Static;
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db.upcast()).source(db.upcast()))
    }
}
impl HasSource for Trait {
    type Ast = ast::Trait;
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db.upcast()).source(db.upcast()))
    }
}
impl HasSource for TypeAlias {
    type Ast = ast::TypeAlias;
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db.upcast()).source(db.upcast()))
    }
}
impl HasSource for MacroDef {
    type Ast = Either<ast::Macro, ast::Fn>;
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>> {
        Some(self.id.ast_id().either(
            |id| id.with_value(Either::Left(id.to_node(db.upcast()))),
            |id| id.with_value(Either::Right(id.to_node(db.upcast()))),
        ))
    }
}
impl HasSource for Impl {
    type Ast = ast::Impl;
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db.upcast()).source(db.upcast()))
    }
}

impl HasSource for TypeOrConstParam {
    type Ast = Either<ast::TypeOrConstParam, ast::Trait>;
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>> {
        let child_source = self.id.parent.child_source(db.upcast());
        Some(child_source.map(|it| it[self.id.local_id].clone()))
    }
}

impl HasSource for LifetimeParam {
    type Ast = ast::LifetimeParam;
    fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>> {
        let child_source = self.id.parent.child_source(db.upcast());
        Some(child_source.map(|it| it[self.id.local_id].clone()))
    }
}
