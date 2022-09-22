use std::str::Chars;

pub(crate) trait SteppedIterator
where
    Self: Iterator,
{
    fn advance_stepped(t: &<Self as Iterator>::Item) -> usize;

    fn take_stepped(self, n: usize) -> TakeStepped<Self>
    where
        Self: Sized,
    {
        TakeStepped::new(self, n)
    }

    fn skip_stepped(self, n: usize) -> SkipStepped<Self>
    where
        Self: Sized,
    {
        SkipStepped::new(self, n)
    }
}

#[derive(Clone)]
pub(crate) struct TakeStepped<I> {
    pub(crate) iter: I,
    pub(crate) n: usize,
}

impl<I> TakeStepped<I>
where
    I: SteppedIterator,
{
    pub(crate) fn new(iter: I, n: usize) -> Self {
        Self { iter, n }
    }
}

impl<I> Iterator for TakeStepped<I>
where
    I: SteppedIterator,
{
    type Item = <I as Iterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            if self.n == 0 {
                return None;
            }
            self.n = self.n.saturating_sub(I::advance_stepped(&item));
            return Some(item);
        }
        None
    }
}

impl<I> SteppedIterator for TakeStepped<I>
where
    I: SteppedIterator,
{
    fn advance_stepped(t: &<Self as Iterator>::Item) -> usize {
        I::advance_stepped(t)
    }
}

#[derive(Clone)]
pub(crate) struct SkipStepped<I> {
    pub(crate) iter: I,
    pub(crate) n: usize,
}

impl<I> SkipStepped<I>
where
    I: SteppedIterator,
{
    pub(crate) fn new(iter: I, n: usize) -> Self {
        Self { iter, n }
    }
}

impl<I> Iterator for SkipStepped<I>
where
    I: SteppedIterator,
{
    type Item = <I as Iterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        #[inline]
        fn check<'a, T>(
            count: &'a mut usize,
            advancer: &'a impl Fn(&T) -> usize,
        ) -> impl FnMut(&T) -> bool + 'a {
            move |chara| {
                if *count == 0 {
                    true
                } else {
                    *count = count.saturating_sub((advancer)(chara));
                    false
                }
            }
        }
        self.iter.find(check(&mut self.n, &I::advance_stepped))
    }
}

impl<I> SteppedIterator for SkipStepped<I>
where
    I: SteppedIterator,
{
    fn advance_stepped(t: &<Self as Iterator>::Item) -> usize {
        I::advance_stepped(t)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MonoChars<'a> {
    pub(crate) iter: Chars<'a>,
}

impl<'a> Iterator for MonoChars<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> SteppedIterator for MonoChars<'a> {
    fn take_stepped(self, n: usize) -> TakeStepped<Self> {
        TakeStepped::new(self, n)
    }

    fn skip_stepped(self, n: usize) -> SkipStepped<Self> {
        SkipStepped::new(self, n)
    }

    fn advance_stepped(t: &<Self as Iterator>::Item) -> usize {
        if t.is_ascii() {
            1
        } else {
            2
        }
    }
}

pub(crate) trait IntoMonoChars {
    fn mono_chars(&mut self) -> MonoChars<'_>;
}

impl IntoMonoChars for String {
    fn mono_chars(&mut self) -> MonoChars<'_> {
        MonoChars { iter: self.chars() }
    }
}
