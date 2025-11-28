pub(crate) trait New {
    const NEW: Self;
}

impl New for () {
    const NEW: Self = ();
}

impl<T0: New> New for (T0,) {
    const NEW: Self = (New::NEW);
}

impl<T0: New, T1: New> New for (T0, T1) {
    const NEW: Self = (New::NEW, New::NEW);
}

impl<T0: New, T1: New, T2: New> New for (T0, T1, T2) {
    const NEW: Self = (New::NEW, New::NEW, New::NEW);
}

impl<T0: New, T1: New, T2: New, T3: New> New for (T0, T1, T2, T3) {
    const NEW: Self = (New::NEW, New::NEW, New::NEW, New::NEW);
}

impl<T0: New, T1: New, T2: New, T3: New, T4: New> New for (T0, T1, T2, T3, T4) {
    const NEW: Self = (New::NEW, New::NEW, New::NEW, New::NEW, New::NEW);
}

impl<T0: New, T1: New, T2: New, T3: New, T4: New, T5: New> New for (T0, T1, T2, T3, T4, T5) {
    const NEW: Self = (New::NEW, New::NEW, New::NEW, New::NEW, New::NEW, New::NEW);
}

#[macro_export]
macro_rules! impl_new {
    ($T: tt, $($Generic:tt, $Bound:tt),+) => {
        impl<$($Generic : $Bound),*> $crate::new::New for $T<$($Generic,)*> {
            const NEW: Self = Self(PhantomData);
        }
    }
}
