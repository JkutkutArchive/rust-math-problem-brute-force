/// Exercise:
/// Use the numbers 5,6,7,8,9 in order, from smallest to largest,
/// together with one of the symbols +,-,* and / and two pairs of brackets
/// to make a calculation with an answer of 25/8
///
/// For example:
/// 43/9
/// (5+6)-(7*8)/9 = 43/9

use std::time::Instant;

use itertools::Itertools;

fn parenthesis_exprs(
	nums: &[i32]
) -> Vec<String> {
	let mut exprs = Vec::new();
	for s1 in 0..nums.len() {
		for e1 in s1..nums.len() {
			for s2 in 0..nums.len() {
				for e2 in s2..nums.len() {
					let mut expr = nums.iter().map(|n| n.to_string()).collect::<Vec<String>>();
					expr[s1] = "(".to_string() + &expr[s1];
					expr[s2] = "(".to_string() + &expr[s2];
					expr[e1] = expr[e1].to_string() + ")";
					expr[e2] = expr[e2].to_string() + ")";
					exprs.push(format!(
						"{}op1{}op2{}op3{}op4{}",
						expr[0], expr[1], expr[2], expr[3], expr[4]
					));
				}
			}
		}
	}
	exprs
}

fn solve(print_all: bool) {
	let nums = [5, 6, 7, 8, 9];
	let ops = ["+", "-", "*", "/"];
	let target = 25.0 / 8.0;
	let delta_error = 1e-8;

	let exprs = parenthesis_exprs(&nums);

	let ops_iter = ops.iter()
		.cartesian_product(ops.iter())
		.cartesian_product(ops.iter())
		.cartesian_product(ops.iter())
		.map(|(((a,b),c),d)| (*a,*b,*c,*d));

	for (op1, op2, op3, op4) in ops_iter {
		for e in &exprs {
			let e = e
				.replace("op1", op1)
				.replace("op2", op2)
				.replace("op3", op3)
				.replace("op4", op4);
			let result = meval::eval_str(&e);
			match result {
				Ok(v) => {
					if (v - target).abs() < delta_error {
						println!("✅ {} = {} = 25/8", e, v);
					}
					else if print_all {
						eprintln!("❌ {} = {}", e, v);
					}
				},
				Err(e) => panic!("❌ Invalid expression: {}", e),
			}
		}
	}
}

use std::env::args;

fn main() {
	let print_all = match args().nth(1) {
		Some(s) => s == "--print-all",
		_ => false
	};
	let now = Instant::now();
	println!("Starting...");
	solve(print_all);
	println!("Finished in {}ms", now.elapsed().as_millis());
}
