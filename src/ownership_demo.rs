#[cfg(test)]
mod tests {
    #[test]
    fn move_out_from_vec() {
        let mut v = Vec::new();
        for i in 101..106 {
            v.push(i.to_string());
        }

        // Doesn't work
        //        let _third = v[2];
        //        let _fifth = v[4];

        // Works
        let _third = &v[2];
        let _fifth = &v[4];
    }

    #[test]
    // error[E0597]: `parabola` does not live long enough
    fn return_ref() {
        fn smallest(v: &[i64]) -> &i64 {
            let mut s = &v[0];
            for r in &v[1..] {
                if *r < *s {
                    s = r;
                }
            }
            s
        }

        // Doesn't work
        //        let s;
        //        {
        //            let parabola = [9,4,1,0,1,4,9];
        //            s = smallest(&parabola);
        //        }
        //        assert_eq!(*s, 0);

        // Works
        {
            let parabola = [9, 4, 1, 0, 1, 4, 9];
            let s = smallest(&parabola);
            assert_eq!(*s, 0);
        }
    }

    #[test]
    fn struct_containing_ref() {
        //        struct S {
        //            r: &i32,
        //        }
        //
        //        let s;
        //        {
        //            let x = 10;
        //            s = S { r: &x };
        //        }
        //        assert_eq!(*s.r, 10);

        struct S<'a> {
            r: &'a i32,
        }

        {
            let x = 10;
            let s = S { r: &x };
            assert_eq!(*s.r, 10);
        }
    }

    #[test]
    fn distinct_lifttime() {
        fn f1<'a>(a: &'a i64, b: &'a i64) -> &'a i64 {
            if *a > *b {
                a
            } else {
                b
            }
        }

        fn f2<'a: 'c, 'b: 'c, 'c>(a: &'a i64, b: &'b i64) -> &'c i64 {
            if *a > *b {
                a
            } else {
                b
            }
        }

        fn f3<'a: 'b, 'b>(a: &'a i64, _b: &'b i64) -> &'a i64 {
            a
        }

        let a = 10;
        let d;
        {
            let b = 5;
            let c = f1(&a, &b);

            assert_eq!(*c, 10);

            let c = f2(&a, &b);

            assert_eq!(*c, 10);

            let c = f3(&a, &b);

            assert_eq!(*c, 10);

            d = f3(&a, &b);
            assert_eq!(*d, 10);
        }
    }
}
