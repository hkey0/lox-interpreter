#[macro_export]
macro_rules! define_visitor {
    ($base:ident, $( $type:ident ),* $(,)?) => {
        paste::paste! {
            pub trait Visitor<R> {
                $(
                    fn [<visitor_ $type:lower _ $base:lower>](&mut self, expr: &$type) -> R;
                )*
            }
        }
    };
}

#[macro_export]
macro_rules! generate_ast {
    (
        $enum_name:ident {
            $(
                $variant:ident ( $( $field_name:ident : $field_ty:ty ),* )
            ),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        pub enum $enum_name {
            $(
                $variant(Box<$variant>),
            )*
        }

        $(
            #[derive(Debug)]
            pub struct $variant {
                $( pub $field_name: $field_ty, )*
            }
        )*
    };
}
