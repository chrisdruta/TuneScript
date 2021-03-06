pub trait Frequency: Iterator {
    fn frequency(self, f: f32) -> FrequencyState<Self>
    where
        Self: Sized,
    {
        FrequencyState {
            freq: f,
            cur: None,
            backlog: if f >= 1. { f } else { 1. },
            underlying: self,
        }
    }
}

impl<I> Frequency for I
where
    I: Iterator,
{
}

#[derive(Clone)]
pub struct FrequencyState<I: Iterator> {
    freq: f32,
    cur: Option<I::Item>,
    backlog: f32,
    underlying: I,
}

impl<I> Iterator for FrequencyState<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        let val = if self.backlog >= 1. {
            let int_part = self.backlog as usize;
            self.backlog -= int_part as f32;

            let mut next = Vec::new();
            for _ in 0..int_part {
                match self.underlying.next() {
                    Some(v) => next.push(v),
                    None => break,
                };
            }

            self.cur = if next.len() == int_part {
                Some(next[next.len() - 1].clone())
            } else {
                None
            };
            self.backlog += self.freq;

            if !next.is_empty() {
                Some(next)
            } else {
                None
            }
        } else {
            self.backlog += self.freq;
            match self.cur {
                None => None,
                Some(ref val) => Some(vec![val.clone()]),
            }
        };
        val
    }
}
