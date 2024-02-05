use crate::expression::Expression;

mod by_hand;

/// Simplify an [`Expression`].
pub(super) fn run(expression: &Expression) -> Expression {
    by_hand::run(expression)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    macro_rules! test_simplify {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let parsed_input = Expression::from_str($input)
                    .unwrap_or_else(|error| panic!("Parsing input `{}` failed: {error}", $input));
                let parsed_expected = Expression::from_str($expected).unwrap_or_else(|error| {
                    panic!(
                        "Parsing expected expression `{}` failed: {error}",
                        $expected
                    )
                });
                let computed = run(&parsed_input);
                assert_eq!(parsed_expected, computed);
            }
        };
    }

    test_simplify! {
        function_cis,
        "cis(0)",
        "1"
    }

    test_simplify! {
        function_cos,
        "cis(0)",
        "1"
    }

    test_simplify! {
        function_exp,
        "exp(1)",
        "2.718281828459045"
    }

    test_simplify! {
        function_sin,
        "sin(0)",
        "0"
    }

    test_simplify! {
        function_sqrt,
        "sqrt(9)",
        "3"
    }

    test_simplify! {
        infix_add_0_r,
        "x + 0",
        "x"
    }

    test_simplify! {
        infix_add_0_l,
        "0 + x",
        "x"
    }

    test_simplify! {
        infix_add,
        "1 + 2",
        "3"
    }

    test_simplify! {
        infix_sub_0_r,
        "x - 0",
        "x"
    }

    test_simplify! {
        infix_sub_self,
        "x - x",
        "0"
    }

    test_simplify! {
        infix_mul_0_r,
        "x * 0",
        "0"
    }

    test_simplify! {
        infix_mul_0_l,
        "0 * x",
        "0"
    }

    test_simplify! {
        infix_mul_1_r,
        "x * 1",
        "x"
    }

    test_simplify! {
        infix_mul_1_l,
        "1 * x",
        "x"
    }

    test_simplify! {
        infix_div_0_l,
        "0 / x",
        "0"
    }

    test_simplify! {
        infix_div_1_r,
        "x / 1",
        "x"
    }

    test_simplify! {
        infix_div_self,
        "x / x",
        "1"
    }

    test_simplify! {
        infix_exp_0_r,
        "0^x",
        "0"
    }

    test_simplify! {
        infix_exp_0_l,
        "x^0",
        "1"
    }

    test_simplify! {
        infix_sub_neg,
        "x - (-y)",
        "x + y"
    }

    test_simplify! {
        infix_mul_double_neg,
        "(-x) * (-y)",
        "x * y"
    }

    test_simplify! {
        infix_div_double_neg,
        "(-x) / (-y)",
        "x / y"
    }

    test_simplify! {
        infix_affine_full,
        "(a1 * x + b1) + (a2 * x + b2)",
        "(a1 + a2) * x + (b1 + b2)"
    }

    test_simplify! {
        infix_affine_coeffs,
        "(a1 * x) + (a2 * x)",
        "(a1 + a2) * x"
    }

    test_simplify! {
        infix_affine_constants,
        "(x + b1) + (x + b2)",
        "(2 * x) + (b1 + b2)"
    }

    test_simplify! {
        infix_mul_div_ll,
        "(y * x) / x",
        "y"
    }

    test_simplify! {
        infix_mul_div_lr,
        "(x * y) / x",
        "y"
    }

    test_simplify! {
        infix_mul_div_rl,
        "x / (y * x)",
        "1 / y"
    }

    test_simplify! {
        infix_mul_div_rr,
        "x / (x * y)",
        "1 / y"
    }

    test_simplify! {
        infix_div_mul_l,
        "(x / y) * y",
        "x"
    }

    test_simplify! {
        infix_div_mul_r,
        "y * (x / y)",
        "x"
    }

    test_simplify! {
        docstring_example,
        "cos(2 * pi) + 2",
        "3"
    }

    test_simplify! {
        issue_208_1,
        "0 * theta[0]",
        "0"
    }

    test_simplify! {
        issue_208_2,
        "theta[0] / 1",
        "theta[0]"
    }

    test_simplify! {
        issue_208_3,
        "(theta[0] * 5) / 5",
        "theta"
    }

    test_simplify! {
        memory_ref,
        "theta[0]",
        "theta[0]"
    }

    test_simplify! {
        var,
        "%foo",
        "%foo"
    }

    test_simplify! {
        prefix_neg,
        "-(-1)",
        "1"
    }

    test_simplify! {
        sub_neg,
        "2 - (-1)",
        "3"
    }

    test_simplify! {
        neg_sub,
        "-(1 - 2)",
        "1"
    }

    test_simplify! {
        fold_constant_mul,
        "2 * pi",
        "6.283185307179586"
    }

    test_simplify! {
        fold_constant_mul_div,
        "(2 * pi) / 6.283185307179586",
        "1"
    }

    test_simplify! {
        fold_constant_mul_div_2,
        "2 * (pi / 6.283185307179586)",
        "1"
    }

    test_simplify! {
        fold_constant_mul_div_with_ref,
        "((a[0] * 2) * pi) / 6.283185307179586",
        "a[0]"
    }

    test_simplify! {
        fold_constant_mul_div_with_ref_2,
        "a[0] * (2 * pi) / 6.283185307179586",
        "a[0]"
    }

    test_simplify! {
        fold_constant_mul_div_with_ref_3,
        "a[0] * (2 * (pi / 6.283185307179586))",
        "a[0]"
    }

    test_simplify! {
        affine,
        "(2 * x[0] + 3) + (4 * x[0] + 5)",
        "6 * x[0] + 8"
    }

    test_simplify! {
        affine_2,
        "2 * x[0] + (4 * x[0] + 5)",
        "6 * x[0] + 5"
    }

    test_simplify! {
        affine_3,
        "2 * x[0] + 4 * x[0]",
        "6 * x[0]"
    }

    test_simplify! {
        affine_4,
        "(x[0] + 3) + (x[0] + 5)",
        "2 * x[0] + 8"
    }

    test_simplify! {
        double_subtraction,
        "3 - 2 - 1",
        "0"
    }

    // TODO doesn't fully simplify in a reasonable amount of recursion
    //     test_simplify! {
    //         the_big_one,
    //         "(6.283185307179586*(-((-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(-((1827.142690137572+-(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702) - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+1293.2884354900702)+1830.4305845069357))+(2404.366183299857+(-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702)) - 2473.4667568746527)+(1292.2571023206997 - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+1293.2884354900702)+(1827.142690137572+-(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702)) - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702)+1830.4305845069357))+2997.220957806505) - ((1827.142690137572+-(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702) - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+1293.2884354900702)+1830.4305845069357))+(2404.366183299857+(-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702)) - 2473.4667568746527)+3082.921997445349)+-((-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+1293.2884354900702)+(-((-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(-((1827.142690137572+-(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702) - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+1293.2884354900702)+1830.4305845069357))+(2404.366183299857+(-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702)) - 2473.4667568746527)+(1292.2571023206997 - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+1293.2884354900702)+(1827.142690137572+-(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702)) - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702)+1830.4305845069357))+2997.220957806505) - ((1827.142690137572+-(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702) - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+1293.2884354900702)+1830.4305845069357))+(2404.366183299857+(-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702)) - 2473.4667568746527)+3082.921997445349)+(1827.142690137572+-(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702) - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+1293.2884354900702)+1830.4305845069357))+-((1827.142690137572+-(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702) - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+1293.2884354900702)+1830.4305845069357))+(2404.366183299857+(-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702)) - 2473.4667568746527))+3552.7822825370968) - ((-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(-((1827.142690137572+-(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702) - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+1293.2884354900702)+1830.4305845069357))+(2404.366183299857+(-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702)) - 2473.4667568746527)+(1292.2571023206997 - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+1293.2884354900702)+(1827.142690137572+-(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702)) - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702)+1830.4305845069357))+2997.220957806505) - ((1827.142690137572+-(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702) - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+1293.2884354900702)+1830.4305845069357))+(2404.366183299857+(-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702)) - 2473.4667568746527)+3082.921997445349)+(((1827.142690137572+-(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702) - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - ((-3.141592653589793+gamma[0]*-1.3670709112264738)/6.283185307179586+1293.2884354900702)+1830.4305845069357))+(2404.366183299857+(-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+(1292.2571023206997 - (-3.141592653589793+(gamma[0]*-1.3670709112264738)/6.283185307179586)+1293.2884354900702)) - 2473.4667568746527)+3654.308518679512+(-3.141592653589793+gamma[0]*-1.4598346220303238)/6.283185307179586)))+0.4345210910722077)/6.283185307179586",
    //         "-0.637964476122525*gamma[0] - 14553.9199845484"
    //     }
}