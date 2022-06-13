use crate::*;
use crate::enums::*;
use crate::dinode::*;
use crate::async_ptr::*;

pub type FlowPtr = AsyncPtr<Flow>;
pub type FlowNode = Node<usize, Empty, FlowPtr>;

#[derive(Clone)]
pub struct Flow { pub max: u64, pub cur: u64, pub rev: AsyncPtr<Flow> }

impl std::fmt::Display for Flow {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}/{}", self.max, self.cur)
	}
}

impl Flow {
	pub fn connect(s: &FlowNode, t: &FlowNode, amount: u64) {
		let fflow = AsyncPtr::from(Flow { max: amount, cur: 0, rev: AsyncPtr::null() });
		let rflow = AsyncPtr::from(Flow { max: amount, cur: amount, rev: AsyncPtr::null() });
		fflow.update(|flow| flow.rev = rflow.clone());
		rflow.update(|flow| flow.rev = fflow.clone());
		connect!(s => t, fflow);
		connect!(t => s, rflow);
	}
}