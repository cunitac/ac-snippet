use cargo_snippet::snippet;

#[snippet("rev")]
#[derive(PartialEq, Eq, Debug)]
struct Rev<T>(T);

#[snippet("rev")]
impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

#[snippet("rev")]
impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}
