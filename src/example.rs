use crate::digraph::*;
use std:: {
	fmt:: {
		Debug,
		Display,
		Formatter,
	}};

pub type TopoExec<T> = Digraph<String, Executor<T>, bool>;
type Work<T> = fn (&T) -> WorkResult<T>;

#[derive(Clone, Debug)]
pub enum WorkResult<T: Clone + Display + Debug> {
	Result(T),
	Empty
}

#[derive(Clone)]
pub struct Executor<T: Clone + Display + Debug> {
	pub payload: T,
	pub work: Work<T>,
	pub result: WorkResult<T>,
}

impl<T: Clone + Display + Debug> Debug for Executor<T> {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "self")
    }
}

impl<T: Clone + Display + Debug> Display for Executor<T> {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "self")
    }
}

impl<T: Clone + Display + Debug> Executor<T> {
	pub fn new(payload: T, work: Work<T> ) -> Self {
		Self {
			payload,
			work,
			result: WorkResult::Empty,
		}
	}

	pub fn exec(&mut self) {
		let f = self.work;
		self.result = f(&self.payload);
	}

	pub fn consume(&mut self) -> WorkResult<T> {
		let res = self.result.clone();
		self.result = WorkResult::Empty;
		res
	}
}

fn foo(str: &String) -> WorkResult<String> {
	let res = str.to_owned() + "2";
	WorkResult::Result(res)
}

pub fn test() {
	let mut g = TopoExec::<String>::new();
	g.insert(
		String::from("START"),
		Executor::new(
			String::from(""),
			foo ));
	g.insert(
		String::from("START"),
		Executor::new(
			String::from("N1"),
			foo ));
	g.insert(
		String::from("START"),
		Executor::new(
			String::from("N1"),
			foo ));
	g.insert(
		String::from("START"),
		Executor::new(
			String::from("N1"),
			foo ));
	g.insert(
		String::from("START"),
		Executor::new(
			String::from("N1"),
			foo ));
}