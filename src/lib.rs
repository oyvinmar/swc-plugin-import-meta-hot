use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{Expr, Ident, MemberExpr, MetaPropKind, Program},
        transforms::testing::test,
        visit::{visit_mut_pass, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
    trace_macro::swc_trace,
};

pub struct TransformVisitor;

#[swc_trace]
impl VisitMut for TransformVisitor {
    fn visit_mut_expr(&mut self, n: &mut Expr) {
        n.visit_mut_children_with(self);

        if let Expr::Member(member) = n {
            if is_visiting_import_meta_hot(member) {
                member.obj = Box::new(Ident::new_no_ctxt("module".into(), DUMMY_SP).into());
            } else if is_visiting_import_meta_env(member) {
                member.obj = Box::new(Ident::new_no_ctxt("process".into(), DUMMY_SP).into());
            }
        }
    }
}

fn is_visiting_import_meta_hot(n: &MemberExpr) -> bool {
    let Some(obj) = n.obj.as_meta_prop() else {
        return false;
    };
    let Some(prop) = n.prop.as_ident() else {
        return false;
    };

    obj.kind == MetaPropKind::ImportMeta && prop.sym == "hot"
}

fn is_visiting_import_meta_env(n: &MemberExpr) -> bool {
    let Some(obj) = n.obj.as_meta_prop() else {
        return false;
    };
    let Some(prop) = n.prop.as_ident() else {
        return false;
    };

    obj.kind == MetaPropKind::ImportMeta && prop.sym == "env"
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.apply(visit_mut_pass(TransformVisitor))
}

test!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor),
    transform_import_meta_hot,
    r#"import.meta.hot"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor),
    transform_import_meta_hot_accept,
    r#"import.meta.hot.accept()"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor),
    transform_import_meta_hot_key,
    r#"import.meta.hot["data"]"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor),
    no_transform_import_meta,
    r#"import.meta"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor),
    transform_import_meta_env,
    r#"import.meta.env"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor),
    transform_import_meta_env_access,
    r#"import.meta.env.NODE_ENV"#
);
