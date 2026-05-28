#[macro_export]
macro_rules! set_or_clear_bit {
    ($target:expr, $bit_num:expr, $enabled:expr) => {
        if $enabled {
            $target |= (1 << $bit_num);
        } else {
            $target &= !(1 << $bit_num);
        }
    };
}

#[macro_export]
macro_rules! set_or_clear_val {
    ($target:expr, $mask_val:expr, $enabled:expr) => {
        if $enabled {
            $target |= $mask_val;
        } else {
            $target &= !$mask_val;
        }
    };
}

#[macro_export]
macro_rules! get_bit {
    ($target:expr, $bit_num:expr) => {
        ($target >> $bit_num) & 1
    };
}
