#![no_std]

pub static PUB_STATIC_IN_GLOBAL: i32 = 0;
pub static PUB_STATIC_IN_GLOBAL_WILL_REMOVE: i32 = 0;
pub static PUB_STATIC_IN_GLOBAL_WILL_RENAME: i32 = 0;
pub static PUB_STATIC_IN_GLOBAL_WILL_BE_CONST: i32 = 0;
pub static PUB_STATIC_IN_GLOBAL_WILL_BE_PRIVATE_CONST: i32 = 0;
pub static PUB_STATIC_IN_GLOBAL_WILL_BE_PRIVATE_STATIC: i32 = 0;

pub mod my_module {
    pub static PUB_STATIC_IN_MODULE: i32 = 0;
    pub static PUB_STATIC_IN_MODULE_WILL_REMOVE: i32 = 0;
    pub static PUB_STATIC_IN_MODULE_WILL_RENAME: i32 = 0;
    pub static PUB_STATIC_IN_MODULE_WILL_BE_CONST: i32 = 0;
    pub static PUB_STATIC_IN_MODULE_WILL_BE_PRIVATE_CONST: i32 = 0;
    pub static PUB_STATIC_IN_MODULE_WILL_BE_PRIVATE_STATIC: i32 = 0;
    pub static PUB_STATIC_IN_MODULE_WILL_RE_EXPORT: i32 = 0;
    pub static PUB_STATIC_IN_MODULE_WILL_RE_EXPORT_CONST: i32 = 0;

    pub mod my_module_nested {
        pub static PUB_STATIC_IN_NESTED_MODULE: i32 = 0;
        pub static PUB_STATIC_IN_NESTED_MODULE_WILL_REMOVE: i32 = 0;
        pub static PUB_STATIC_IN_NESTED_MODULE_WILL_RENAME: i32 = 0;
        pub static PUB_STATIC_IN_NESTED_MODULE_WILL_BE_CONST: i32 = 0;
        pub static PUB_STATIC_IN_NESTED_MODULE_WILL_BE_PRIVATE_CONST: i32 = 0;
        pub static PUB_STATIC_IN_NESTED_MODULE_WILL_BE_PRIVATE_STATIC: i32 = 0;
        pub static PUB_STATIC_IN_NESTED_MODULE_WILL_RE_EXPORT: i32 = 0;
        pub static PUB_STATIC_IN_NESTED_MODULE_WILL_RE_EXPORT_CONST: i32 = 0;
    }
}
