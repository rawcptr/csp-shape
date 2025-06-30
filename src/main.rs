#[allow(unused)]
use csp_shape::{
    constraint_err, csp_bail,
    domain::Domain,
    error::{TraceFrame, UnifyError},
    fresh_var,
    term::VarGen,
};

fn main() {
    let mut var_gen = VarGen::new();

    let x = fresh_var!(var_gen, "x");
    let y = fresh_var!(var_gen,);

    let trace = vec![
        TraceFrame::Branched {
            var: x.clone(),
            value: 42,
        },
        TraceFrame::Constrained {
            constraint: format!("{} == {}", x.no_name(), y.no_name()).into(),
            domains: vec![
                (x.clone(), Domain::Single(42)),
                (y.clone(), Domain::Range { min: 40, max: 43 }),
            ],
        },
        TraceFrame::Backtracked {
            var: x,
            failed_value: 42,
        },
    ];
    if let Err(e) = wow(trace) {
        println!("{e:?}");
    }
}

fn wow(trace: Vec<TraceFrame>) -> Result<(), UnifyError> {
    csp_bail!("AHHH GET ME OUTTA HERE!!!", trace, "im not solving dat shi");
}
