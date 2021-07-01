pub fn format_apply<F>(apply: F, args: core::fmt::Arguments<'_>) -> core::fmt::Result
where
    F: FnMut(&str) -> core::fmt::Result,
{
    struct FakeWriter<F: FnMut(&str) -> core::fmt::Result> {
        functor: F,
    }
    impl<F: FnMut(&str) -> core::fmt::Result> core::fmt::Write for FakeWriter<F> {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            (self.functor)(s)
        }
    }
    use core::fmt::Write;
    FakeWriter { functor: apply }.write_fmt(args)
}
