macro_rules! solutions {
    ($days:literal) => {
        seq_macro::seq! { d in 1..=$days{
            paste::paste! {
                pub mod [<day d>];
            }
        }}
    };
}

solutions!(10);
