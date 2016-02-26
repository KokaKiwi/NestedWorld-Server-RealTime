
macro_rules! config_struct {
    (
        pub struct $name:ident {
            $(pub $field_name:ident: $field_ty:ty = $field_default:expr,)+
        }
    ) => {
        #[derive(Debug, RustcEncodable, RustcDecodable)]
        pub struct $name {
            $(pub $field_name: $field_ty,)+
        }

        impl ::std::default::Default for $name {
            fn default() -> Self {
                $name {
                    $($field_name: $field_default,)+
                }
            }
        }
    };
    (
        pub struct $name:ident;
    ) => {
        #[derive(Debug, Default, RustcEncodable, RustcDecodable)]
        pub struct $name;
    }
}
