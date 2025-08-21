//! Assertions and error messages.

/// Assert elements are a multiple of the slice length.
macro_rules! assert_element_len {
    (
        $axis:ident,
        $el_len:ident,
        $slice_len:ident
    ) => {
        assert!(
            $el_len % $slice_len == 0,
            "operation on axis {} expected a multiple of {} elements (recieved {})",
            $axis,
            $slice_len,
            $el_len
        );
    };
}

/// Assert elements are less than or equal to the slice length.
macro_rules! assert_slice_len {
    (
        $array:ident,
        $axis:ident,
        $el_n:ident
    ) => {
        assert!(
            $el_n <= $array.shape[$axis],
            "operation on axis {} expected a slice length <= axis length {} (recieved {})",
            $axis,
            $array.shape[$axis],
            $el_n
        )
    };
}

/// Assert an index is within the array dimensionality.
macro_rules! assert_shape_index {
    (
        $axis:ident,
        $N:ident
    ) => {
        assert!(
            $axis < $N,
            "axis {} is out of bounds for dimensionality {}",
            $axis,
            $N
        );
    };
}

/// Assert an axis index is in bounds.
macro_rules! assert_slice_index {
    (
        $array:ident,
        $axis:ident,
        $index:expr
    ) => {
        assert!(
            $index < $array.shape[$axis],
            "slice index {} is out of bounds axis {} of length {}",
            $index,
            $axis,
            $array.shape[$axis]
        );
    };
}

/// Assert an axis index is in bounds.
macro_rules! assert_slice_range {
    (
        $array:ident,
        $axis:ident,
        $range:ident
    ) => {
        assert!(
            $range.len() <= $array.shape[$axis],
            "range {:?} is out of bounds for axis {} of length {}",
            $range,
            $axis,
            $array.shape[$axis]
        );
    };
}

/// Assert translation is in bounds.
macro_rules! assert_origin_bounds {
    (
        $axis:ident,
        $origin:ident,
        $n:ident
    ) => {
        assert!(
            $origin[$axis].checked_add($n).is_some(),
            "translation {} is out of bounds for axis {}, origin {:?}",
            $origin,
            $n,
            $axis,
        );
    };
    (
        $axis:ident,
        $origin:ident,
        -$n:ident
    ) => {
        assert!(
            $origin[$axis].checked_sub($n).is_some(),
            "translation -{} is out of bounds for axis {}, origin {:?}",
            $n,
            $axis,
            $origin
        );
    };
}
