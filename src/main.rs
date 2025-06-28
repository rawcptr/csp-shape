use csp_shape::{
    BinaryOpConstraint, 
    EqualityConstraint, 
    constraint::Constraints, 
    term::Subst
};

fn main() {
    let mut subst = Subst::new();

    let constraints: Constraints = Constraints::new(vec![
        EqualityConstraint::boxed("a".into(), 10.into()),
        EqualityConstraint::boxed("b".into(), 6.into()),
        EqualityConstraint::boxed("d".into(), 4.into()),
        BinaryOpConstraint::sum("a".into(), "b".into(), "c".into()),
        BinaryOpConstraint::product("c".into(), "d".into(), "e".into()),
    ]);

    constraints.solve(&mut subst);
}