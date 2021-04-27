use std::cell::Cell;
use std::rc::Rc;
use std::rc::Weak;

pub struct TimerQueue<P, T> {
    v: Vec<Rc<TQElem<P, T>>>,
}

struct TQElem<P, T> {
    priority: Cell<P>,
    data: T,
    idx: usize,
}
impl<P, T> TQElem<P, T> {
    fn new(data: T, priority: P, idx: usize) -> Self {
        Self {
            data,
            idx,
            priority: Cell::new(priority),
        }
    }
}
/// Hide the fact that this is just a weak pointer to an element so lusers
/// don't "acidentally" modify something in the Q.
pub struct TQHandle<P, T> {
    h: Weak<TQElem<P, T>>,
}
impl<P, T> TQHandle<P, T> {
    fn new(h: Weak<TQElem<P, T>>) -> Self {
        Self { h }
    }
}

impl<P, T> TimerQueue<P, T>
where
    P: PartialOrd + Clone,
{
    pub fn new() -> Self {
        TimerQueue { v: Vec::new() }
    }
    pub fn insert(&mut self, data: T, priority: P) -> TQHandle<P, T> {
        let idx = self.v.len();
        let s = Rc::new(TQElem::new(data, priority, idx));
        let ret = TQHandle::new(Rc::downgrade(&s));
        self.v.push(s);
        self.heap_up(idx);
        ret
    }
    pub fn change_priority(&mut self, h: &TQHandle<T, P>, new_priority: P) -> Option<P> {
        let idx = h.h.upgrade()?.idx;
        let old_priority = self.v[idx].priority.replace(new_priority);
        let idx = self.heap_up(idx);
        self.heap_down(idx);
        Some(old_priority)
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.v.is_empty() {
            None
        } else {
            let e = Rc::try_unwrap(self.v.pop()?).expect("Internal TimerQ error");
            Some(e.data)
        }
    }
    pub fn peek(&self) -> Option<P> {
        Some(self.v.get()?.priority.get_mut().clone())
    }
    fn heap_up(&mut self, _idx: usize) -> usize {
        0
    }
    fn heap_down(&mut self, _idx: usize) -> usize {
        0
    }
    pub fn rm(&mut self, _h: &TQHandle<T, P>) -> Option<T> {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
