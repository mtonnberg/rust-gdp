use core::mem::{align_of, size_of};
use rust_gdp::{predicates::*, relations::*, uint::*, *};

fn test<P: Predicate<T>, T: ?Sized>(v: &T) -> bool {
    P::test(v)
}

#[test]
fn representations_are_erased() {
    assert_eq!(size_of::<Proof<And<True, True>>>(), 0);
    assert_eq!(size_of::<Proof<Holds<'static, NonEmpty>>>(), 0);
    assert_eq!(size_of::<Proof<Related<'static, 'static, Less>>>(), 0);
    assert_eq!(size_of::<Refined<u64, Positive>>(), size_of::<u64>());
    assert_eq!(align_of::<Refined<u64, Positive>>(), align_of::<u64>());
    assert_eq!(size_of::<U13>(), size_of::<u16>());
    assert_eq!(size_of::<Named<'static, u64>>(), size_of::<u64>());
}
#[test]
fn scoped_proofs_and_logic() {
    with_name("abc", |s| {
        let p = s.check::<NonEmpty>().unwrap();
        let q = s.check::<Ascii>().unwrap();
        let pair = p.and(q);
        let (p, _) = pair.split();
        let p = identity().apply(p);
        let _ = and_left().apply(pair);
        let _ = and_right().apply(pair);
        let _ = and_left().then(identity()).apply(pair);
        let _ = p
            .or_left::<Holds<'_, NonEmpty>>()
            .cases(identity(), identity());
        let both = p.both(q);
        let (_, _) = both.components();
        let refined = s.refine(both);
        assert_eq!(*refined.get(), "abc");
        refined.with_name(|s, p| {
            let (_, _) = p.components();
            assert_eq!(*s.get(), "abc");
        });
    });
    with_name("", |s| {
        assert!(s.check::<NonEmpty>().is_none());
        let negative = s.decide::<NonEmpty>().unwrap_err();
        assert_eq!(*s.refine(negative).get(), "");
    });
    let _ = truth().and(truth()).split();
    let _ = truth().or_right::<True>();
}
#[test]
fn refinements_check_only_at_boundary() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    enum Count {}
    impl Predicate<u32> for Count {
        fn test(v: &u32) -> bool {
            CALLS.fetch_add(1, Ordering::SeqCst);
            *v > 0
        }
    }
    let r = Refined::<_, Count>::new(3).unwrap();
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
    r.with_name(|n, p| {
        assert_eq!(*n.get(), 3);
        assert_eq!(n.refine(p).into_inner(), 3);
    });
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
    assert_eq!(Refined::<_, Positive>::new(-1).unwrap_err(), -1);
    let p = Refined::<_, Positive>::new(1).unwrap();
    assert_eq!(p.try_map::<_, Positive>(|n| n - 2).unwrap_err(), -1);
}
#[test]
fn text_and_collection_predicates() {
    assert!(!test::<NonEmpty, _>(""));
    assert!(test::<NonEmpty, _>("🦀"));
    assert!(test::<Length<4>, _>("🦀"));
    assert!(!test::<Ascii, _>("🦀"));
    assert!(test::<Ascii, _>("hello"));
    assert!(test::<MinLength<2>, _>("hi"));
    assert!(test::<MaxLength<2>, _>("hi"));
    assert!(!test::<Trimmed, _>("\u{2003}hello"));
    assert!(test::<Trimmed, _>("hello world"));
    assert!(test::<Utf8, _>(&b"ok"[..]));
    assert!(!test::<Utf8, _>(&[0xff][..]));
    assert!(test::<Ascii, _>(&b"ok"[..]));
    assert!(test::<Each<Positive>, _>(&[1, 2][..]));
    assert!(!test::<Each<Positive>, _>(&[1, 0][..]));
    assert!(test::<Each<Positive>, _>(&[] as &[i32]));
    assert!(!test::<Exists<Positive>, _>(&[] as &[i32]));
    assert!(test::<Exists<Positive>, _>(&[-1, 2][..]));
    assert!(test::<Sorted, _>(&[1, 1, 2][..]));
    assert!(!test::<Sorted, _>(&[2, 1][..]));
    assert!(!test::<Sorted, _>(&[0.0, f64::NAN][..]));
    assert!(test::<All<Positive, Even>, _>(&2));
    assert!(test::<Any<Positive, Even>, _>(&-2));
    assert!(test::<Not<Positive>, _>(&-2));
    assert!(test::<IsSome, _>(&Some(1)));
    assert!(test::<IsNone, _>(&None::<()>));
    assert!(test::<IsOk, _>(&Ok::<_, ()>(1)));
    assert!(test::<IsErr, _>(&Err::<(), _>(1)));
}
#[cfg(feature = "alloc")]
#[test]
fn allocated_predicates() {
    let s = String::from("hello");
    assert!(test::<All<NonEmpty, All<Ascii, Trimmed>>, _>(&s));
    assert!(test::<Length<5>, _>(&s));
    assert!(test::<MinLength<5>, _>(&s));
    assert!(test::<MaxLength<5>, _>(&s));
    let v = vec![1, 2];
    assert!(test::<All<NonEmpty, Each<Positive>>, _>(&v));
    assert!(test::<Exists<Even>, _>(&v));
    assert!(test::<Sorted, _>(&v));
    assert!(test::<Length<2>, _>(&v));
    assert!(test::<MinLength<2>, _>(&v));
    assert!(test::<MaxLength<2>, _>(&v));
}
#[test]
fn numeric_boundaries() {
    for n in i16::MIN..=i16::MAX {
        assert_eq!(test::<Positive, _>(&n), n > 0);
        assert_eq!(test::<NonZero, _>(&n), n != 0);
        assert_eq!(test::<Even, _>(&n), n % 2 == 0);
        assert_eq!(test::<Odd, _>(&n), n % 2 != 0);
        assert_eq!(test::<InRange<0, 8191>, _>(&n), (0..=8191).contains(&n));
    }
    assert!(test::<InRange<0, { u128::MAX }>, _>(&u128::MAX));
    assert!(!test::<InRange<2, 1>, _>(&1u8));
    for n in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        -0.0,
        0.0,
        1.0,
        -1.0,
    ] {
        assert_eq!(test::<Positive, _>(&n), n > 0.0);
        assert_eq!(test::<NonZero, _>(&n), !n.is_nan() && n != 0.0);
        assert_eq!(test::<Finite, _>(&n), n.is_finite());
    }
}
#[test]
fn u13_exhaustive_and_width_edges() {
    assert!(UInt::<0>::new(0).is_none());
    assert!(UInt::<129>::new(0).is_none());
    assert_eq!(UInt::<128>::new(u128::MAX).unwrap().get(), u128::MAX);
    assert!(UInt::<1>::new(2).is_none());
    assert!(U13::new(8192).is_none());
    for n in 0..=8191 {
        let a = U13::new(n).unwrap();
        assert_eq!(a.get(), n);
        assert_eq!(test::<Positive, _>(&a), n > 0);
        assert_eq!(test::<NonZero, _>(&a), n > 0);
        assert_eq!(test::<Even, _>(&a), n % 2 == 0);
        assert_eq!(test::<Odd, _>(&a), n % 2 != 0);
        assert!(test::<InRange<0, 8191>, _>(&a));
        assert_eq!(
            a.checked_add(U13::new(1).unwrap()).map(|v| v.get()),
            if n < 8191 { Some(n + 1) } else { None }
        );
        assert_eq!(
            a.checked_sub(U13::new(1).unwrap()).map(|v| v.get()),
            n.checked_sub(1)
        );
        assert_eq!(
            a.checked_mul(U13::new(2).unwrap()).map(|v| v.get()),
            if n * 2 <= 8191 { Some(n * 2) } else { None }
        );
    }
    let max = UInt::<128>::new(u128::MAX).unwrap();
    let two = UInt::<128>::new(2).unwrap();
    assert!(max.checked_add(two).is_none());
    assert!(max.checked_mul(two).is_none());
}
#[test]
fn binary_evidence() {
    with_name(3, |a| {
        with_name(4, |b| {
            assert!(a.relate::<_, Less>(&b).is_some());
            assert!(a.relate::<_, LessEqual>(&b).is_some());
            assert!(a.relate::<_, Equal>(&b).is_none());
            assert!(b.relate::<_, Less>(&a).is_none());
            assert!(a.relate::<_, Equal>(&a).is_some());
        })
    });
    with_name(f64::NAN, |a| {
        assert!(a.relate::<_, LessEqual>(&a).is_none());
        assert!(a.relate::<_, Equal>(&a).is_none());
    });
}

#[test]
fn quantified_borrowed_elements() {
    let values = [1, 2, 3];
    let all = Refined::<_, Each<Positive>>::new(&values[..]).unwrap();
    fn consume(v: CheckedRef<'_, i32, Positive>) -> i32 {
        *v.get()
    }
    assert_eq!(all.iter().map(consume).sum::<i32>(), 6);
    assert_eq!(consume(all.element(1).unwrap()), 2);
    assert!(all.element(3).is_none());
    let empty = Refined::<_, Each<Positive>>::new(&[] as &[i32]).unwrap();
    assert!(empty.element(0).is_none());
    assert_eq!(empty.iter().len(), 0);
    let p = Refined::<_, Positive>::new(1).unwrap();
    assert_eq!(consume(p.as_checked()), 1);
    assert_eq!(
        size_of::<CheckedRef<'_, i32, Positive>>(),
        size_of::<&i32>()
    );
    assert_eq!(*rust_gdp::transform::trim("  hi  ").get(), "hi");
}
#[cfg(feature = "alloc")]
#[test]
fn quantified_owned_elements_and_transformations() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    enum Count {}
    impl Predicate<i32> for Count {
        fn test(n: &i32) -> bool {
            CALLS.fetch_add(1, Ordering::SeqCst);
            *n > 0
        }
    }
    let all = rust_gdp::collections::filter_checked::<_, Count>([-1, 1, 2, 0]);
    assert_eq!(CALLS.load(Ordering::SeqCst), 4);
    assert_eq!(all.iter().map(|v| *v.get()).sum::<i32>(), 3);
    assert!(all.element(2).is_none());
    assert_eq!(*all.element(0).unwrap().get(), 1);
    let mapped = all.map_checked(|v| v);
    assert_eq!(
        mapped
            .into_elements()
            .map(|v| v.into_inner())
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(CALLS.load(Ordering::SeqCst), 4);
    let sorted = rust_gdp::transform::sort(vec![3, 1, 2]);
    assert_eq!(sorted.into_inner(), vec![1, 2, 3]);
}

#[test]
fn existential_relation_transport() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    enum Ordered {}
    impl Relation<i32, i32> for Ordered {
        fn test(a: &i32, b: &i32) -> bool {
            CALLS.fetch_add(1, Ordering::SeqCst);
            a < b
        }
    }
    let pair = with_name(1, |a| {
        with_name(2, |b| {
            let p = a.relate::<_, Ordered>(&b).unwrap();
            a.pack_with(b, p)
        })
    });
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
    let values = pair.with_names(|a, b, p| a.pack_with(b, p).into_inner());
    assert_eq!(values, (1, 2));
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

#[test]
fn successful_transformation_and_decision() {
    let p = Refined::<_, Positive>::new(1).unwrap();
    let q = p.try_map::<_, Even>(|n| n + 1).unwrap();
    assert_eq!(q.into_inner(), 2);
    with_name(1, |n| {
        let p = n.decide::<Positive>().unwrap();
        assert_eq!(n.refine(p).into_inner(), 1);
    });
    let max = U13::new(U13::MAX).unwrap();
    assert!(max.checked_mul(max).is_none());
    assert!(UInt::<127>::new(1u128 << 127).is_none());
    assert!(UInt::<127>::new((1u128 << 127) - 1).is_some());
    assert!(!<Positive as Predicate<i128>>::test(&i128::MIN));
    assert!(!<InRange<0, 1> as Predicate<i128>>::test(&i128::MIN));
    assert!(<Positive as Predicate<usize>>::test(&usize::MAX));
    assert!(!<NonZero as Predicate<f32>>::test(&f32::NAN));
    assert!(!<Finite as Predicate<f32>>::test(&f32::INFINITY));
}
