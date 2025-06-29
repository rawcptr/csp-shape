use csp_shape::{
    BinaryOpConstraint, EqualityConstraint, GreaterThanConstraint, LessThanConstraint,
    constraint::Constraints, term::Subst,
};

fn main() {
    let mut subst = Subst::new();

    let constraints: Constraints = Constraints::new(vec![
        // EqualityConstraint::boxed("a".into(), 10.into()),
        // EqualityConstraint::boxed("e".into(), 14.into()),
        // BinaryOpConstraint::sum("a".into(), "b".into(), "c".into()),
        // LessThanConstraint::boxed("d".into(), "a".into()),
        // // GreaterThanConstraint::boxed("b".into(), "a".into()), // should fail
        // EqualityConstraint::boxed("d".into(), 4.into()),
        // EqualityConstraint::boxed("b".into(), 6.into()),
        // BinaryOpConstraint::product("c".into(), "d".into(), "e".into()),

        // bing bang booom 
        BinaryOpConstraint::sum("a".into(), "a".into(), 1.into()),
    ]);

    constraints.solve(&mut subst);
}
