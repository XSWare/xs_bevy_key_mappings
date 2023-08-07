#![feature(macro_metavar_expr)]

macro_rules! declare_mappings {
    ($($input_call:ident,)*) => {
        $(
            ::paste::paste!{
                #[macro_export]
                macro_rules! [<map_ $input_call>] {
                    ($$($func_name:ident => $key:ident),* $$(,)?) => {
                        $$(pub fn $func_name(input: ::bevy::prelude::Res<::bevy::prelude::Input<::bevy::prelude::KeyCode>>) -> bool {
                            input.$input_call(::bevy::prelude::KeyCode::$key)
                        })*
                    };
                }
            }
        )*
    }
}

declare_mappings!(clear_just_pressed, clear_just_released, just_pressed, just_released, pressed,);
