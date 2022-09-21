use std::str::Chars;

#[derive(Clone)]
pub(crate) struct SteppedIter<I, Ad> {
    pub(crate) iter: I,
    pub(crate) advancer: Ad,
}

impl<I, Ad> Iterator for SteppedIter<I, Ad>
where
    I: Iterator,
    Ad: Fn(&I::Item) -> usize,
{
    type Item = <I as Iterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub(crate) trait SteppedIterator<I, Ad>
where
    Self: Iterator,
    I: Iterator,
    Ad: Fn(&I::Item) -> usize,
{
    fn take_stepped(self, n: usize) -> TakeStepped<I, Ad>;
    fn skip_stepped(self, n: usize) -> SkipStepped<I, Ad>;
}

impl<I, Ad> SteppedIterator<I, Ad> for SteppedIter<I, Ad>
where
    I: Iterator,
    Ad: Fn(&I::Item) -> usize,
{
    fn take_stepped(self, n: usize) -> TakeStepped<I, Ad> {
        TakeStepped::<I, Ad>::new(self.iter, n, self.advancer)
    }

    fn skip_stepped(self, n: usize) -> SkipStepped<I, Ad> {
        SkipStepped::<I, Ad>::new(self.iter, n, self.advancer)
    }
}

#[derive(Clone)]
pub(crate) struct TakeStepped<I, Ad> {
    pub(crate) iter: I,
    pub(crate) n: usize,
    pub(crate) advancer: Ad,
}

impl<I, Ad> TakeStepped<I, Ad>
where
    I: Iterator,
    Ad: Fn(&I::Item) -> usize,
{
    pub(crate) fn new(iter: I, n: usize, advancer: Ad) -> Self {
        Self { iter, n, advancer }
    }
}

impl<I, Ad> Iterator for TakeStepped<I, Ad>
where
    I: Iterator,
    Ad: Fn(&I::Item) -> usize,
{
    type Item = <I as Iterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            if self.n == 0 {
                return None;
            }
            self.n = self.n.saturating_sub((self.advancer)(&item));
            return Some(item);
        }
        None
    }
}

impl<I, Ad> SteppedIterator<I, Ad> for TakeStepped<I, Ad>
where
    I: Iterator,
    Ad: Fn(&I::Item) -> usize,
{
    fn take_stepped(self, n: usize) -> TakeStepped<I, Ad> {
        TakeStepped::new(self.iter, n, self.advancer)
    }

    fn skip_stepped(self, n: usize) -> SkipStepped<I, Ad> {
        SkipStepped::new(self.iter, n, self.advancer)
    }
}

#[derive(Clone)]
pub(crate) struct SkipStepped<I, Ad> {
    pub(crate) iter: I,
    pub(crate) n: usize,
    pub(crate) advancer: Ad,
}

impl<I, Ad> SkipStepped<I, Ad>
where
    I: Iterator,
    Ad: Fn(&I::Item) -> usize,
{
    pub(crate) fn new(iter: I, n: usize, advancer: Ad) -> Self {
        Self { iter, n, advancer }
    }
}

impl<I, Ad> Iterator for SkipStepped<I, Ad>
where
    I: Iterator,
    Ad: Fn(&I::Item) -> usize,
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
        self.iter.find(check(&mut self.n, &self.advancer))
    }
}

impl<I, Ad> SteppedIterator<I, Ad> for SkipStepped<I, Ad>
where
    I: Iterator,
    Ad: Fn(&I::Item) -> usize,
{
    fn take_stepped(self, n: usize) -> TakeStepped<I, Ad> {
        TakeStepped::new(self.iter, n, self.advancer)
    }

    fn skip_stepped(self, n: usize) -> SkipStepped<I, Ad> {
        SkipStepped::new(self.iter, n, self.advancer)
    }
}

pub(crate) trait IntoSteppedIter<Ad>
where
    Self: Iterator + Sized,
    Ad: Fn(&Self::Item) -> usize,
{
    fn into_stepped_iter(&mut self, advaner: Ad) -> SteppedIter<Self, Ad>;
}

impl<'a, Ad> IntoSteppedIter<Ad> for Chars<'a>
where
    Ad: Fn(&char) -> usize,
{
    fn into_stepped_iter(&mut self, advancer: Ad) -> SteppedIter<Self, Ad> {
        // Clone original parsed UTF-8 string
        SteppedIter {
            iter: self.into_iter().to_owned(),
            advancer,
        }
    }
}
