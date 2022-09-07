use twpb::*;

#[test]
fn test_macro(){
    // make an array to iterate over
    let dummydata = [1 as u8,2,3,4,5,6,7,8,9,10];
    // create a regular iterator
    let mut iter = dummydata.into_iter();
    // create a limiter iterator that only yields the first 4 items
    {
        let mut iter2 = LimitedIterator::new(&mut iter, 4);
        assert_eq!(Some(1), iter2.next());
        assert_eq!(Some(2), iter2.next());
        assert_eq!(Some(3), iter2.next());
        assert_eq!(Some(4), iter2.next());
        assert_eq!(None, iter2.next());
    }

    // confirm that we can still read the original iterator
    assert_eq!(Some(5), iter.next());
    {
        let mut iter2 = LimitedIterator::new(&mut iter, 0);
        assert_eq!(None, iter2.next());
    }
    {
        let mut iter2 = LimitedIterator::new(&mut iter, 2);
        assert_eq!(Some(6), iter2.next());
        assert_eq!(Some(7), iter2.next());
        assert_eq!(None, iter2.next());
    }
    {
        let mut iter2 = LimitedIterator::new(&mut iter, 100);
        assert_eq!(Some(8), iter2.next());
        assert_eq!(Some(9), iter2.next());
        assert_eq!(Some(10), iter2.next());
        assert_eq!(None, iter2.next());
    }
    assert_eq!(None, iter.next());
}