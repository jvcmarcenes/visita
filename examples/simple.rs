
use visita::*;

#[derive(Debug, Clone)]
enum BinOp { Add, Sub }

#[node_group(data = ())]
#[derive(Debug, Clone)]
enum Expr {

	#[derive(Debug, Clone)]
	NumLit(f32),

	#[derive(Debug, Clone)]
	Bin {
		op: BinOp,
		lhs: Expr,
		rhs: Expr,
	}

}

#[visitor(Expr, output = f32)]
struct Interpreter;

impl Visitor<NumLit> for Interpreter {
	fn visit(&mut self, node: &NumLit, _data: &Data<Self, NumLit>) -> Self::Output {
		node.0
	}
}

impl Visitor<Bin> for Interpreter {
	fn visit(&mut self, node: &Bin, _data: &Data<Self, Bin>) -> Self::Output {
		match node.op {
			BinOp::Add => node.lhs.accept(self) + node.rhs.accept(self),
			BinOp::Sub => node.lhs.accept(self) - node.rhs.accept(self),
		}
	}
}

#[visitor(Expr, output = String)]
struct Printer;

impl Visitor<NumLit> for Printer {
	fn visit(&mut self, node: &NumLit, _data: &Data<Self, Bin>) -> Self::Output {
		format!("{}", node.0)
	}
}

impl Visitor<Bin> for Printer {
	fn visit(&mut self, node: &Bin, _data: &Data<Self, Bin>) -> Self::Output {
		format!("({} {} {})",
			node.lhs.accept(self),
			match node.op { BinOp::Add => "+", BinOp::Sub => "-" },
			node.rhs.accept(self),
		)
	}
}

fn main() {
	let expr: Expr = Bin {
		op: BinOp::Add,
		lhs: NumLit(23.0).to_node(()),
		rhs: Bin {
			op: BinOp::Sub,
			lhs: NumLit(42.0).to_node(()),
			rhs: NumLit(19.0).to_node(()),
		}.to_node(()),
	}.to_node(());
	
	let interpreter_res = expr.accept(&mut Interpreter);
	let printer_res = expr.accept(&mut Printer);
	
	println!("interpreter: {interpreter_res}");
	println!("printer: {printer_res}");
}
