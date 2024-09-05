#![feature(print_internals)]
#![feature(panic_internals)]
#![feature(alloc)]
#![feature(fmt_helpers_for_derive)]
#![allow(warnings, unused)]
#![feature(hint_must_use)]
#![feature(liballoc_internals)]
// >>Push: Global("app") -- [None]
//  CMETA: []
// >>Push: Module("AppModule") -- [None]
// >>Push: Service("AppController") -- [Some(String("AppModule"))]
//  CMETA: ["version"]
//  CMETA: ["auth", "role"]
// controller AppController []
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
//  CMETA: ["ControllerPath"]
// service_derive "AppController"
// >>Push: Handler("get_hello_world") -- [Some(String("AppModule"))]
//  CMETA: ["method_uses"]
//  CMETA: ["arr"]
//  CMETA: ["DisableDefaultPrefix"]
//  CMETA: ["version"]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "get_hello_world"
// route_derive is_tuple true
// << Pop: Some(Handler("get_hello_world")) ["method_uses", "version", "handler", "RouterPath", "arr", "DisableDefaultPrefix", "RouterName", "RouterMethod", "service", "auth", "ServiceType", "ServiceName", "ControllerPath", "role", "module", "global"]

// >>Push: Handler("post_hello_world") -- [Some(String("AppModule"))]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "post_hello_world"
// route_derive is_tuple false
// << Pop: Some(Handler("post_hello_world")) ["handler", "RouterName", "RouterPath", "RouterMethod", "version", "service", "auth", "ServiceType", "ServiceName", "ControllerPath", "role", "module", "global"]

// << Pop: Some(Service("AppController")) ["version", "service", "auth", "ServiceType", "ServiceName", "ControllerPath", "role", "module", "global"]

// >>Push: Service("AppInterceptor") -- [Some(String("AppModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "AppInterceptor"
// << Pop: Some(Service("AppInterceptor")) ["service", "ServiceType", "ServiceName", "module", "global"]

// >>Push: Service("AppService") -- [Some(String("AppModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "AppService"
// module "AppModule"
// << Pop: Some(Service("AppService")) ["service", "ServiceName", "ServiceType", "module", "global"]

// << Pop: Some(Module("AppModule")) ["module", "global"]

// >>Push: Module("ConfModule") -- [None]
// >>Push: Service("ConfOptions") -- [Some(String("ConfModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "ConfOptions"
// << Pop: Some(Service("ConfOptions")) ["service", "ServiceName", "ServiceType", "module", "global"]

// >>Push: Service("ConfService") -- [Some(String("ConfModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "ConfService"
// module "ConfModule"
// controller UserController []
// << Pop: Some(Service("ConfService")) ["ServiceName", "ServiceType", "service", "module", "global"]

// << Pop: Some(Module("ConfModule")) ["module", "global"]

// >>Push: Module("UserModule") -- [None]
// >>Push: Service("UserController") -- [Some(String("UserModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
//  CMETA: ["ControllerPath"]
// service_derive "UserController"
// >>Push: Handler("get_hello_world") -- [Some(String("UserModule"))]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "get_hello_world"
// route_derive is_tuple false
// << Pop: Some(Handler("get_hello_world")) ["RouterPath", "handler", "RouterMethod", "RouterName", "ServiceType", "service", "ServiceName", "ControllerPath", "module", "global"]

// >>Push: Handler("create_user") -- [Some(String("UserModule"))]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "create_user"
// route_derive is_tuple false
// << Pop: Some(Handler("create_user")) ["handler", "RouterPath", "RouterName", "RouterMethod", "ServiceType", "service", "ServiceName", "ControllerPath", "module", "global"]

// << Pop: Some(Service("UserController")) ["ServiceType", "service", "ServiceName", "ControllerPath", "module", "global"]

// >>Push: Service("UserService") -- [Some(String("UserModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "UserService"
// module "UserModule"
// << Pop: Some(Service("UserService")) ["service", "ServiceName", "ServiceType", "module", "global"]

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod app {
    use nidrs::macros::module;
    pub mod controller {
        use std::collections::HashMap;
        use nidrs::externs::axum::{extract::Query, response::AppendHeaders, Json};
        use nidrs::macros::{controller, get, meta, post, uses};
        use nidrs::{version, Inject, Meta};
        use crate::AppResult;
        use super::{
            dto::{ArgDto, Status},
            interceptor::AppInterceptor, service::AppService,
        };
        pub struct AppController {
            app_service: Inject<AppService>,
        }
        #[automatically_derived]
        impl ::core::default::Default for AppController {
            #[inline]
            fn default() -> AppController {
                AppController {
                    app_service: ::core::default::Default::default(),
                }
            }
        }
        impl nidrs::Controller for AppController {}
        impl nidrs::Service for AppController {
            fn inject(
                &self,
                ctx: nidrs::ModuleCtx,
                module_name: &str,
            ) -> nidrs::ModuleCtx {
                let service = ctx.get_service::<AppService>(&module_name, "AppService");
                self.app_service.inject(service.clone());
                ctx
            }
        }
        impl nidrs::ImplMeta for AppController {
            fn __meta() -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set("version", "v1");
                meta.set("service", "AppController");
                meta.set("auth", "true");
                meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                meta.set_data(nidrs::datasets::ServiceName::from("AppController"));
                meta.set_data(nidrs::datasets::ControllerPath::from(""));
                meta.set("role", "admin");
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
        }
        impl AppController {
            pub async fn get_hello_world(
                &self,
                meta: Meta,
                Query(q): Query<HashMap<String, String>>,
            ) -> AppResult<(AppendHeaders<[(String, String); 2]>, Status)> {
                {
                    ::std::io::_print(format_args!("Query {0:?}\n", q));
                };
                {
                    ::std::io::_print(format_args!("Meta Keys {0:?}\n", meta.keys()));
                };
                {
                    ::std::io::_print(
                        format_args!("Meta {0:?}\n", meta.get::<&str>("role")),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Meta {0:?}\n",
                            meta.get_data::<nidrs::datasets::DisableDefaultPrefix>(),
                        ),
                    );
                };
                Ok((
                    AppendHeaders([
                        ("X-Custom-Header".to_string(), "hello".to_string()),
                        ("X-Custom-Header".to_string(), "world".to_string()),
                    ]),
                    Status {
                        db: "ok".to_string(),
                        redis: "ok".to_string(),
                    },
                ))
            }
            pub fn __meta_get_hello_world(&self) -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set("method_uses", ["AppInterceptor"]);
                meta.set("version", "v2");
                meta.set("handler", "get_hello_world");
                meta.set_data(nidrs::datasets::RouterPath::from("/hello"));
                meta.set("arr", ["user"]);
                meta.set_data(nidrs::datasets::DisableDefaultPrefix(false));
                meta.set_data(nidrs::datasets::RouterName::from("get_hello_world"));
                meta.set_data(nidrs::datasets::RouterMethod::from("get"));
                meta.set("service", "AppController");
                meta.set("auth", "true");
                meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                meta.set_data(nidrs::datasets::ServiceName::from("AppController"));
                meta.set_data(nidrs::datasets::ControllerPath::from(""));
                meta.set("role", "admin");
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
            pub fn __route_get_hello_world(
                &self,
                mut ctx: nidrs::ModuleCtx,
            ) -> nidrs::ModuleCtx {
                use nidrs::externs::axum;
                use axum::response::IntoResponse;
                use nidrs::externs::axum::{extract::Query, Json};
                use nidrs::externs::meta::{InnerMeta, Meta};
                use nidrs::Interceptor;
                use serde_json::Value;
                let mut meta = self.__meta_get_hello_world();
                let router_info = ctx.get_router_full(&meta);
                if let Err(e) = router_info {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("[{0}] {1:?}", "__route_get_hello_world", e),
                        );
                    };
                }
                let full_path = router_info.unwrap();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Registering router \'{0} {1}\'.\n",
                            "get".to_uppercase(),
                            full_path,
                        ),
                    );
                };
                meta.set_data(nidrs::datasets::RouterFullPath(full_path.clone()));
                let meta = Meta::new(meta);
                let module_name = meta.get::<&str>("module").unwrap();
                let controller_name = meta
                    .get_data::<nidrs::datasets::ServiceName>()
                    .unwrap()
                    .value();
                let t_controller = ctx
                    .get_controller::<Self>(module_name, controller_name);
                let router = nidrs::externs::axum::Router::new()
                    .route(
                        &full_path,
                        nidrs::externs::axum::routing::get(|p0, p1| async move {
                            let r = t_controller.get_hello_world(p0, p1).await;
                            r
                        }),
                    )
                    .layer(nidrs::externs::axum::Extension(meta.clone()))
                    .layer(
                        axum::middleware::from_fn({
                            let inter = ctx
                                .get_interceptor::<
                                    AppInterceptor,
                                >(module_name, "AppInterceptor");
                            move |
                                req: axum::extract::Request,
                                next: axum::middleware::Next|
                            {
                                let inter = std::sync::Arc::clone(&inter);
                                async move {
                                    let res = inter.intercept(req, next).await;
                                    if let Ok(res) = res {
                                        Ok(res.into_response())
                                    } else {
                                        Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                                    }
                                }
                            }
                        }),
                    );
                ctx.routers.push(nidrs::MetaRouter::new(router, meta));
                ctx
            }
            pub async fn post_hello_world(
                &self,
                Query(q): Query<HashMap<String, String>>,
                Json(j): Json<ArgDto>,
            ) -> AppResult<String> {
                {
                    ::std::io::_print(format_args!("Query {0:?}\n", q));
                };
                {
                    ::std::io::_print(format_args!("Json {0:?}\n", j));
                };
                Ok("Hello, World2!".to_string())
            }
            pub fn __meta_post_hello_world(&self) -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set("handler", "post_hello_world");
                meta.set_data(nidrs::datasets::RouterName::from("post_hello_world"));
                meta.set_data(nidrs::datasets::RouterPath::from("/hello"));
                meta.set_data(nidrs::datasets::RouterMethod::from("post"));
                meta.set("version", "v1");
                meta.set("service", "AppController");
                meta.set("auth", "true");
                meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                meta.set_data(nidrs::datasets::ServiceName::from("AppController"));
                meta.set_data(nidrs::datasets::ControllerPath::from(""));
                meta.set("role", "admin");
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
            pub fn __route_post_hello_world(
                &self,
                mut ctx: nidrs::ModuleCtx,
            ) -> nidrs::ModuleCtx {
                use nidrs::externs::axum;
                use axum::response::IntoResponse;
                use nidrs::externs::axum::{extract::Query, Json};
                use nidrs::externs::meta::{InnerMeta, Meta};
                use nidrs::Interceptor;
                use serde_json::Value;
                let mut meta = self.__meta_post_hello_world();
                let router_info = ctx.get_router_full(&meta);
                if let Err(e) = router_info {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("[{0}] {1:?}", "__route_post_hello_world", e),
                        );
                    };
                }
                let full_path = router_info.unwrap();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Registering router \'{0} {1}\'.\n",
                            "post".to_uppercase(),
                            full_path,
                        ),
                    );
                };
                meta.set_data(nidrs::datasets::RouterFullPath(full_path.clone()));
                let meta = Meta::new(meta);
                let module_name = meta.get::<&str>("module").unwrap();
                let controller_name = meta
                    .get_data::<nidrs::datasets::ServiceName>()
                    .unwrap()
                    .value();
                let t_controller = ctx
                    .get_controller::<Self>(module_name, controller_name);
                let router = nidrs::externs::axum::Router::new()
                    .route(
                        &full_path,
                        nidrs::externs::axum::routing::get(|p0, p1| async move {
                            let r = t_controller.post_hello_world(p0, p1).await;
                            match r {
                                Ok(r) => Json(r).into_response(),
                                Err(e) => e.into_response(),
                            }
                        }),
                    )
                    .layer(nidrs::externs::axum::Extension(meta.clone()));
                ctx.routers.push(nidrs::MetaRouter::new(router, meta));
                ctx
            }
        }
    }
    pub mod dto {
        use nidrs::externs::serde::{Deserialize, Serialize};
        use nidrs::externs::serde_json;
        use nidrs::{
            externs::axum::{
                body::Body, http::{header, StatusCode},
                response::{IntoResponse, Response},
            },
            valid_macro::dto,
        };
        pub struct Status {
            pub db: String,
            pub redis: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Status {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Status",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "db",
                        &self.db,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "redis",
                        &self.redis,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Status {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "db" => _serde::__private::Ok(__Field::__field0),
                                "redis" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"db" => _serde::__private::Ok(__Field::__field0),
                                b"redis" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Status>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Status;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Status",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Status with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Status with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Status {
                                db: __field0,
                                redis: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("db"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("redis"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("db")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("redis")?
                                }
                            };
                            _serde::__private::Ok(Status {
                                db: __field0,
                                redis: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["db", "redis"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Status",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Status>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Status {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Status",
                    "db",
                    &self.db,
                    "redis",
                    &&self.redis,
                )
            }
        }
        impl IntoResponse for Status {
            fn into_response(self) -> Response {
                let json_body = match serde_json::to_string(&self) {
                    Ok(json) => json,
                    Err(_) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body("Internal server error".into())
                            .unwrap();
                    }
                };
                let res: Response<Body> = Response::builder()
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(json_body.into())
                    .unwrap();
                res
            }
        }
        pub struct A {
            #[rule(Email)]
            pub hello: String,
            #[rule(Valid(v))]
            pub hello2: B,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for A {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "A",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "hello",
                        &self.hello,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "hello2",
                        &self.hello2,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for A {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "hello" => _serde::__private::Ok(__Field::__field0),
                                "hello2" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"hello" => _serde::__private::Ok(__Field::__field0),
                                b"hello2" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<A>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = A;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct A",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct A with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                B,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct A with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(A {
                                hello: __field0,
                                hello2: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<B> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("hello"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("hello2"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<B>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("hello")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("hello2")?
                                }
                            };
                            _serde::__private::Ok(A {
                                hello: __field0,
                                hello2: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["hello", "hello2"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "A",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<A>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for A {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "A",
                    "hello",
                    &self.hello,
                    "hello2",
                    &&self.hello2,
                )
            }
        }
        impl nidrs::valid::validator::Validator for A {
            fn valid(&self) -> nidrs::valid::validator::ValidResult {
                use nidrs::valid::validator::Rule;
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                let v = &self.hello;
                Email.valid(v, "hello", None)?;
                let v = &self.hello2;
                Valid(v).valid(v, "hello2", None)?;
                return Ok(());
            }
            fn example(&self) -> Vec<serde_json::Value> {
                ::alloc::vec::Vec::new()
            }
        }
        pub struct B {
            pub hello2: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for B {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "B",
                        false as usize + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "hello2",
                        &self.hello2,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for B {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "hello2" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"hello2" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<B>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = B;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct B",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct B with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(B { hello2: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("hello2"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("hello2")?
                                }
                            };
                            _serde::__private::Ok(B { hello2: __field0 })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["hello2"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "B",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<B>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for B {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "B",
                    "hello2",
                    &&self.hello2,
                )
            }
        }
        impl nidrs::valid::validator::Validator for B {
            fn valid(&self) -> nidrs::valid::validator::ValidResult {
                use nidrs::valid::validator::Rule;
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                return Ok(());
            }
            fn example(&self) -> Vec<serde_json::Value> {
                ::alloc::vec::Vec::new()
            }
        }
        pub enum ArgDto {
            A(A),
            B(B),
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ArgDto {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        ArgDto::A(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "ArgDto",
                                0u32,
                                "A",
                                __field0,
                            )
                        }
                        ArgDto::B(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "ArgDto",
                                1u32,
                                "B",
                                __field0,
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ArgDto {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 2",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "A" => _serde::__private::Ok(__Field::__field0),
                                "B" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"A" => _serde::__private::Ok(__Field::__field0),
                                b"B" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ArgDto>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ArgDto;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum ArgDto",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<A>(__variant),
                                        ArgDto::A,
                                    )
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<B>(__variant),
                                        ArgDto::B,
                                    )
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &["A", "B"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "ArgDto",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ArgDto>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for ArgDto {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ArgDto::A(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "A",
                            &__self_0,
                        )
                    }
                    ArgDto::B(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "B",
                            &__self_0,
                        )
                    }
                }
            }
        }
        impl nidrs::valid::validator::Validator for ArgDto {
            fn valid(&self) -> nidrs::valid::validator::ValidResult {
                use nidrs::valid::validator::Rule;
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                match self {
                    ArgDto::A(v) => v.valid()?,
                    ArgDto::B(v) => v.valid()?,
                }
                return Ok(());
            }
            fn example(&self) -> Vec<serde_json::Value> {
                ::alloc::vec::Vec::new()
            }
        }
        pub struct ArgWrapDto(pub ArgDto);
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ArgWrapDto {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    _serde::Serializer::serialize_newtype_struct(
                        __serializer,
                        "ArgWrapDto",
                        &self.0,
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ArgWrapDto {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ArgWrapDto>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ArgWrapDto;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "tuple struct ArgWrapDto",
                            )
                        }
                        #[inline]
                        fn visit_newtype_struct<__E>(
                            self,
                            __e: __E,
                        ) -> _serde::__private::Result<Self::Value, __E::Error>
                        where
                            __E: _serde::Deserializer<'de>,
                        {
                            let __field0: ArgDto = <ArgDto as _serde::Deserialize>::deserialize(
                                __e,
                            )?;
                            _serde::__private::Ok(ArgWrapDto(__field0))
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                ArgDto,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"tuple struct ArgWrapDto with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ArgWrapDto(__field0))
                        }
                    }
                    _serde::Deserializer::deserialize_newtype_struct(
                        __deserializer,
                        "ArgWrapDto",
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ArgWrapDto>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for ArgWrapDto {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "ArgWrapDto",
                    &&self.0,
                )
            }
        }
        impl nidrs::valid::validator::Validator for ArgWrapDto {
            fn valid(&self) -> nidrs::valid::validator::ValidResult {
                use nidrs::valid::validator::Rule;
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                return Ok(());
            }
            fn example(&self) -> Vec<serde_json::Value> {
                ::alloc::vec::Vec::new()
            }
        }
    }
    pub mod exception {
        pub enum AppException {
            ServiceException(String),
        }
    }
    pub mod interceptor {
        use nidrs::{injectable, AppResult, Interceptor};
        use nidrs_extern::axum::{
            extract::Request, middleware::Next, response::IntoResponse,
        };
        pub struct AppInterceptor;
        #[automatically_derived]
        impl ::core::default::Default for AppInterceptor {
            #[inline]
            fn default() -> AppInterceptor {
                AppInterceptor {}
            }
        }
        impl nidrs::Service for AppInterceptor {
            fn inject(
                &self,
                ctx: nidrs::ModuleCtx,
                module_name: &str,
            ) -> nidrs::ModuleCtx {
                ctx
            }
        }
        impl nidrs::ImplMeta for AppInterceptor {
            fn __meta() -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set("service", "AppInterceptor");
                meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                meta.set_data(nidrs::datasets::ServiceName::from("AppInterceptor"));
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
        }
        impl Interceptor for AppInterceptor {
            async fn intercept(
                &self,
                req: Request,
                next: Next,
            ) -> AppResult<impl IntoResponse> {
                {
                    ::std::io::_print(
                        format_args!("Intercepting request: {0:?}\n", req),
                    );
                };
                let res = next.run(req).await;
                {
                    ::std::io::_print(
                        format_args!("Intercepting response: {0:?}\n", res),
                    );
                };
                Ok(res)
            }
        }
    }
    pub mod service {
        use nidrs::macros::injectable;
        pub struct AppService {}
        #[automatically_derived]
        impl ::core::default::Default for AppService {
            #[inline]
            fn default() -> AppService {
                AppService {}
            }
        }
        impl nidrs::Service for AppService {
            fn inject(
                &self,
                ctx: nidrs::ModuleCtx,
                module_name: &str,
            ) -> nidrs::ModuleCtx {
                ctx
            }
        }
        impl nidrs::ImplMeta for AppService {
            fn __meta() -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set("service", "AppService");
                meta.set_data(nidrs::datasets::ServiceName::from("AppService"));
                meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
        }
        impl AppService {
            pub fn get_hello_world2(&self) -> String {
                "Hello, nidrs2xx333!".to_string()
            }
        }
    }
    use crate::modules::conf::ConfModule;
    use crate::modules::conf::ConfOptions;
    use crate::modules::user::UserModule;
    use controller::AppController;
    use interceptor::AppInterceptor;
    use service::AppService;
    pub struct AppModule;
    #[automatically_derived]
    impl ::core::default::Default for AppModule {
        #[inline]
        fn default() -> AppModule {
            AppModule {}
        }
    }
    impl nidrs::Module for AppModule {
        fn init(self, mut ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
            use nidrs::{
                Service, Controller, Interceptor, InterCtx, InterceptorHandler,
                ModuleCtx, StateCtx, ImplMeta,
            };
            if ctx.modules.contains_key("AppModule") {
                return ctx;
            }
            {
                ::std::io::_print(
                    format_args!(
                        "{0} ",
                        nidrs_extern::colored::Colorize::green("[nidrs]"),
                    ),
                );
            };
            {
                ::std::io::_print(
                    format_args!("Registering module {0}.\n", "AppModule"),
                );
            };
            ctx.modules.insert("AppModule".to_string(), Box::new(self));
            ctx.imports
                .insert(
                    "AppModule".to_string(),
                    Vec::from(["ConfModule".to_string(), "UserModule".to_string()]),
                );
            ctx.append_exports("AppModule", Vec::from(["AppService"]), false);
            ctx.register_interceptor(
                "AppModule",
                "AppInterceptor",
                Box::new(std::sync::Arc::new(crate::import::AppInterceptor::default())),
            );
            if ctx
                .register_controller(
                    "AppModule",
                    "AppController",
                    Box::new(std::sync::Arc::new(controller::AppController::default())),
                )
            {
                let t_controller = ctx
                    .get_controller::<
                        controller::AppController,
                    >("AppModule", "AppController");
                ctx = t_controller.__route_get_hello_world(ctx);
                let t_controller = ctx
                    .get_controller::<
                        controller::AppController,
                    >("AppModule", "AppController");
                ctx = t_controller.__route_post_hello_world(ctx);
            }
            let svc = std::sync::Arc::new(AppService::default());
            ctx.register_service("AppModule", "AppService", Box::new(svc));
            let dyn_module = ConfModule::for_root(ConfOptions {
                log_level: "info".to_string(),
            });
            let mut dyn_module_services = dyn_module.services;
            dyn_module_services
                .drain()
                .for_each(|(k, v)| {
                    ctx.register_service("ConfModule", &k, v);
                });
            let mut dyn_module_exports = dyn_module.exports;
            ctx.append_exports(
                "ConfModule",
                dyn_module_exports,
                nidrs::get_meta_by_type::<ConfModule>()
                    .get_data::<nidrs::datasets::Global>()
                    .unwrap_or(&nidrs::datasets::Global(false))
                    .value(),
            );
            let mut ctx = ConfModule::default().init(ctx);
            let mut ctx = UserModule::default().init(ctx);
            let t = ctx.get_service::<AppService>("AppModule", "AppService");
            {
                ::std::io::_print(
                    format_args!(
                        "{0} ",
                        nidrs_extern::colored::Colorize::green("[nidrs]"),
                    ),
                );
            };
            {
                ::std::io::_print(
                    format_args!("Injecting {0}::{1}.\n", "AppModule", "AppService"),
                );
            };
            let ctx = t.inject(ctx, &"AppModule");
            let t = ctx.get_controller::<AppController>("AppModule", "AppController");
            {
                ::std::io::_print(
                    format_args!(
                        "{0} ",
                        nidrs_extern::colored::Colorize::green("[nidrs]"),
                    ),
                );
            };
            {
                ::std::io::_print(
                    format_args!("Injecting {0}::{1}.\n", "AppModule", "AppController"),
                );
            };
            let ctx = t.inject(ctx, &"AppModule");
            let t = ctx.get_interceptor::<AppInterceptor>("AppModule", "AppInterceptor");
            {
                ::std::io::_print(
                    format_args!(
                        "{0} ",
                        nidrs_extern::colored::Colorize::green("[nidrs]"),
                    ),
                );
            };
            {
                ::std::io::_print(
                    format_args!("Injecting {0}::{1}.\n", "AppModule", "AppInterceptor"),
                );
            };
            let ctx = t.inject(ctx, &"AppModule");
            ctx
        }
        fn destroy(&self, ctx: &nidrs::ModuleCtx) {
            {
                ::std::io::_print(
                    format_args!(
                        "{0} ",
                        nidrs_extern::colored::Colorize::green("[nidrs]"),
                    ),
                );
            };
            {
                ::std::io::_print(format_args!("Destroying module {0}.\n", "AppModule"));
            };
        }
    }
    impl nidrs::ImplMeta for AppModule {
        fn __meta() -> nidrs::InnerMeta {
            let mut meta = nidrs::InnerMeta::new();
            meta.set("service", "AppService");
            meta.set_data(nidrs::datasets::ServiceName::from("AppService"));
            meta.set_data(nidrs::datasets::ServiceType::from("Service"));
            meta.set("module", "AppModule");
            meta.set("global", "app");
            meta
        }
    }
}
mod modules {
    pub mod conf {
        pub mod options {
            use nidrs::macros::injectable;
            pub struct ConfOptions {
                pub log_level: String,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ConfOptions {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "ConfOptions",
                        "log_level",
                        &&self.log_level,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::default::Default for ConfOptions {
                #[inline]
                fn default() -> ConfOptions {
                    ConfOptions {
                        log_level: ::core::default::Default::default(),
                    }
                }
            }
            impl nidrs::Service for ConfOptions {
                fn inject(
                    &self,
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    ctx
                }
            }
            impl nidrs::ImplMeta for ConfOptions {
                fn __meta() -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set("service", "ConfOptions");
                    meta.set_data(nidrs::datasets::ServiceName::from("ConfOptions"));
                    meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                    meta.set("module", "ConfModule");
                    meta.set("global", "app");
                    meta
                }
            }
        }
        pub mod service {
            use nidrs::macros::{injectable, on_module_init};
            use nidrs::{on_module_destroy, Inject};
            use super::options::ConfOptions;
            pub struct ConfService {
                pub options: Inject<ConfOptions>,
                pub log_level: String,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ConfService {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ConfService",
                        "options",
                        &self.options,
                        "log_level",
                        &&self.log_level,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::default::Default for ConfService {
                #[inline]
                fn default() -> ConfService {
                    ConfService {
                        options: ::core::default::Default::default(),
                        log_level: ::core::default::Default::default(),
                    }
                }
            }
            impl nidrs::Service for ConfService {
                fn inject(
                    &self,
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    let service = ctx
                        .get_service::<ConfOptions>(&module_name, "ConfOptions");
                    self.options.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for ConfService {
                fn __meta() -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::datasets::ServiceName::from("ConfService"));
                    meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                    meta.set("service", "ConfService");
                    meta.set("module", "ConfModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl ConfService {
                pub fn on_module_init(&self) {
                    let options = self.options.extract();
                    {
                        ::std::io::_print(
                            format_args!(
                                "ConfService initialized with log_level: {0:?}\n",
                                options,
                            ),
                        );
                    };
                }
                pub fn on_module_destroy(&self) {
                    {
                        ::std::io::_print(format_args!("ConfService destroyed\n"));
                    };
                }
            }
        }
        use nidrs::macros::module;
        use nidrs::{DynamicModule, Service};
        pub use options::ConfOptions;
        use service::ConfService;
        pub struct ConfModule;
        #[automatically_derived]
        impl ::core::default::Default for ConfModule {
            #[inline]
            fn default() -> ConfModule {
                ConfModule {}
            }
        }
        impl nidrs::Module for ConfModule {
            fn init(self, mut ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
                use nidrs::{
                    Service, Controller, Interceptor, InterCtx, InterceptorHandler,
                    ModuleCtx, StateCtx, ImplMeta,
                };
                if ctx.modules.contains_key("ConfModule") {
                    return ctx;
                }
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Registering module {0}.\n", "ConfModule"),
                    );
                };
                ctx.modules.insert("ConfModule".to_string(), Box::new(self));
                ctx.imports.insert("ConfModule".to_string(), Vec::from([]));
                ctx.append_exports("ConfModule", Vec::from(["ConfService"]), false);
                let svc = std::sync::Arc::new(ConfService::default());
                ctx.register_service("ConfModule", "ConfService", Box::new(svc));
                let t = ctx.get_service::<ConfService>("ConfModule", "ConfService");
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Injecting {0}::{1}.\n",
                            "ConfModule",
                            "ConfService",
                        ),
                    );
                };
                let ctx = t.inject(ctx, &"ConfModule");
                let service = ctx
                    .get_service::<ConfService>("ConfModule", "ConfService");
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Triggering event {0} for {1}::{2}.\n",
                            "on_module_init",
                            "ConfModule",
                            "ConfService",
                        ),
                    );
                };
                service.on_module_init();
                ctx
            }
            fn destroy(&self, ctx: &nidrs::ModuleCtx) {
                let service = ctx
                    .get_service::<ConfService>("ConfModule", "ConfService");
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Triggering event {0} for {1}::{2}.\n",
                            "on_module_destroy",
                            "ConfModule",
                            "ConfService",
                        ),
                    );
                };
                service.on_module_destroy();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Destroying module {0}.\n", "ConfModule"),
                    );
                };
            }
        }
        impl nidrs::ImplMeta for ConfModule {
            fn __meta() -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set_data(nidrs::datasets::ServiceName::from("ConfService"));
                meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                meta.set("service", "ConfService");
                meta.set("module", "ConfModule");
                meta.set("global", "app");
                meta
            }
        }
        impl ConfModule {
            pub fn for_root(options: ConfOptions) -> DynamicModule {
                DynamicModule::new().service(options)
            }
        }
    }
    pub mod user {
        use nidrs::macros::module;
        pub mod controller {
            use std::collections::HashMap;
            use nidrs::macros::{controller, get};
            use nidrs::{externs::axum::extract::Query, post};
            use nidrs::{AppResult, Inject};
            use nidrs_extern::axum::Json;
            use super::{dto::CreateUserDto, service::UserService};
            pub struct UserController {
                user_service: Inject<UserService>,
            }
            #[automatically_derived]
            impl ::core::default::Default for UserController {
                #[inline]
                fn default() -> UserController {
                    UserController {
                        user_service: ::core::default::Default::default(),
                    }
                }
            }
            impl nidrs::Controller for UserController {}
            impl nidrs::Service for UserController {
                fn inject(
                    &self,
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    let service = ctx
                        .get_service::<UserService>(&module_name, "UserService");
                    self.user_service.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for UserController {
                fn __meta() -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                    meta.set("service", "UserController");
                    meta.set_data(nidrs::datasets::ServiceName::from("UserController"));
                    meta.set_data(nidrs::datasets::ControllerPath::from("/user"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl UserController {
                pub async fn get_hello_world(
                    &self,
                    Query(q): Query<HashMap<String, String>>,
                ) -> AppResult<String> {
                    {
                        ::std::io::_print(format_args!("Query {0:?}\n", q));
                    };
                    Ok(self.user_service.extract().get_hello_world2())
                }
                pub fn __meta_get_hello_world(&self) -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::datasets::RouterPath::from("/hello"));
                    meta.set("handler", "get_hello_world");
                    meta.set_data(nidrs::datasets::RouterMethod::from("get"));
                    meta.set_data(nidrs::datasets::RouterName::from("get_hello_world"));
                    meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                    meta.set("service", "UserController");
                    meta.set_data(nidrs::datasets::ServiceName::from("UserController"));
                    meta.set_data(nidrs::datasets::ControllerPath::from("/user"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
                pub fn __route_get_hello_world(
                    &self,
                    mut ctx: nidrs::ModuleCtx,
                ) -> nidrs::ModuleCtx {
                    use nidrs::externs::axum;
                    use axum::response::IntoResponse;
                    use nidrs::externs::axum::{extract::Query, Json};
                    use nidrs::externs::meta::{InnerMeta, Meta};
                    use nidrs::Interceptor;
                    use serde_json::Value;
                    let mut meta = self.__meta_get_hello_world();
                    let router_info = ctx.get_router_full(&meta);
                    if let Err(e) = router_info {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!("[{0}] {1:?}", "__route_get_hello_world", e),
                            );
                        };
                    }
                    let full_path = router_info.unwrap();
                    {
                        ::std::io::_print(
                            format_args!(
                                "{0} ",
                                nidrs_extern::colored::Colorize::green("[nidrs]"),
                            ),
                        );
                    };
                    {
                        ::std::io::_print(
                            format_args!(
                                "Registering router \'{0} {1}\'.\n",
                                "get".to_uppercase(),
                                full_path,
                            ),
                        );
                    };
                    meta.set_data(nidrs::datasets::RouterFullPath(full_path.clone()));
                    let meta = Meta::new(meta);
                    let module_name = meta.get::<&str>("module").unwrap();
                    let controller_name = meta
                        .get_data::<nidrs::datasets::ServiceName>()
                        .unwrap()
                        .value();
                    let t_controller = ctx
                        .get_controller::<Self>(module_name, controller_name);
                    let router = nidrs::externs::axum::Router::new()
                        .route(
                            &full_path,
                            nidrs::externs::axum::routing::get(|p0| async move {
                                let r = t_controller.get_hello_world(p0).await;
                                match r {
                                    Ok(r) => Json(r).into_response(),
                                    Err(e) => e.into_response(),
                                }
                            }),
                        )
                        .layer(nidrs::externs::axum::Extension(meta.clone()));
                    ctx.routers.push(nidrs::MetaRouter::new(router, meta));
                    ctx
                }
                pub async fn create_user(
                    &self,
                    dto: Json<CreateUserDto>,
                ) -> AppResult<String> {
                    Ok(self.user_service.extract().get_hello_world2())
                }
                pub fn __meta_create_user(&self) -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set("handler", "create_user");
                    meta.set_data(nidrs::datasets::RouterPath::from("/"));
                    meta.set_data(nidrs::datasets::RouterName::from("create_user"));
                    meta.set_data(nidrs::datasets::RouterMethod::from("post"));
                    meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                    meta.set("service", "UserController");
                    meta.set_data(nidrs::datasets::ServiceName::from("UserController"));
                    meta.set_data(nidrs::datasets::ControllerPath::from("/user"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
                pub fn __route_create_user(
                    &self,
                    mut ctx: nidrs::ModuleCtx,
                ) -> nidrs::ModuleCtx {
                    use nidrs::externs::axum;
                    use axum::response::IntoResponse;
                    use nidrs::externs::axum::{extract::Query, Json};
                    use nidrs::externs::meta::{InnerMeta, Meta};
                    use nidrs::Interceptor;
                    use serde_json::Value;
                    let mut meta = self.__meta_create_user();
                    let router_info = ctx.get_router_full(&meta);
                    if let Err(e) = router_info {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!("[{0}] {1:?}", "__route_create_user", e),
                            );
                        };
                    }
                    let full_path = router_info.unwrap();
                    {
                        ::std::io::_print(
                            format_args!(
                                "{0} ",
                                nidrs_extern::colored::Colorize::green("[nidrs]"),
                            ),
                        );
                    };
                    {
                        ::std::io::_print(
                            format_args!(
                                "Registering router \'{0} {1}\'.\n",
                                "post".to_uppercase(),
                                full_path,
                            ),
                        );
                    };
                    meta.set_data(nidrs::datasets::RouterFullPath(full_path.clone()));
                    let meta = Meta::new(meta);
                    let module_name = meta.get::<&str>("module").unwrap();
                    let controller_name = meta
                        .get_data::<nidrs::datasets::ServiceName>()
                        .unwrap()
                        .value();
                    let t_controller = ctx
                        .get_controller::<Self>(module_name, controller_name);
                    let router = nidrs::externs::axum::Router::new()
                        .route(
                            &full_path,
                            nidrs::externs::axum::routing::get(|p0| async move {
                                let r = t_controller.create_user(p0).await;
                                match r {
                                    Ok(r) => Json(r).into_response(),
                                    Err(e) => e.into_response(),
                                }
                            }),
                        )
                        .layer(nidrs::externs::axum::Extension(meta.clone()));
                    ctx.routers.push(nidrs::MetaRouter::new(router, meta));
                    ctx
                }
            }
        }
        pub mod dto {
            use nidrs::valid_macro::dto;
            use nidrs_extern::utoipa;
            pub struct CreateUserDto {
                #[rule(Email, "age must be greater than 0")]
                pub name: String,
                #[rule(Number::default().max(12).min(0))]
                pub age: i32,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CreateUserDto {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CreateUserDto",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "name",
                            &self.name,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "age",
                            &self.age,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for CreateUserDto {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "name" => _serde::__private::Ok(__Field::__field0),
                                    "age" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"name" => _serde::__private::Ok(__Field::__field0),
                                    b"age" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<CreateUserDto>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CreateUserDto;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct CreateUserDto",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct CreateUserDto with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    i32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct CreateUserDto with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(CreateUserDto {
                                    name: __field0,
                                    age: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<i32> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("age"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("name")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("age")?
                                    }
                                };
                                _serde::__private::Ok(CreateUserDto {
                                    name: __field0,
                                    age: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["name", "age"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CreateUserDto",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<CreateUserDto>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::fmt::Debug for CreateUserDto {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "CreateUserDto",
                        "name",
                        &self.name,
                        "age",
                        &&self.age,
                    )
                }
            }
            impl nidrs::valid::validator::Validator for CreateUserDto {
                fn valid(&self) -> nidrs::valid::validator::ValidResult {
                    use nidrs::valid::validator::Rule;
                    use nidrs::valid::ruleset;
                    use nidrs::valid::ruleset::*;
                    let v = &self.name;
                    Email
                        .valid(
                            v,
                            "name",
                            Some("age must be greater than 0".to_string()),
                        )?;
                    let v = &self.age;
                    Number::default().max(12).min(0).valid(v, "age", None)?;
                    return Ok(());
                }
                fn example(&self) -> Vec<serde_json::Value> {
                    ::alloc::vec::Vec::new()
                }
            }
            impl<'__s> utoipa::ToSchema<'__s> for CreateUserDto {
                fn schema() -> (
                    &'__s str,
                    utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
                ) {
                    (
                        "CreateUserDto",
                        utoipa::openapi::ObjectBuilder::new()
                            .property(
                                "name",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::String),
                            )
                            .required("name")
                            .property(
                                "age",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("age")
                            .into(),
                    )
                }
            }
        }
        pub mod service {
            use std::sync::{Arc, Mutex};
            use nidrs::macros::injectable;
            use nidrs::Inject;
            use crate::app::service::AppService;
            pub struct UserService {
                app_service: Inject<AppService>,
                count: Arc<Mutex<i32>>,
            }
            #[automatically_derived]
            impl ::core::default::Default for UserService {
                #[inline]
                fn default() -> UserService {
                    UserService {
                        app_service: ::core::default::Default::default(),
                        count: ::core::default::Default::default(),
                    }
                }
            }
            impl nidrs::Service for UserService {
                fn inject(
                    &self,
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    let service = ctx
                        .get_service::<AppService>(&module_name, "AppService");
                    self.app_service.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for UserService {
                fn __meta() -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set("service", "UserService");
                    meta.set_data(nidrs::datasets::ServiceName::from("UserService"));
                    meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl UserService {
                pub fn get_hello_world(&self) -> String {
                    self.app_service.extract().get_hello_world2()
                }
                pub fn get_hello_world2(&self) -> String {
                    let mut count = self.count.lock().unwrap();
                    *count += 1;
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("Hello, World! {0}", count),
                        );
                        res
                    })
                }
            }
        }
        use crate::app::AppModule;
        use controller::UserController;
        use service::UserService;
        pub struct UserModule;
        #[automatically_derived]
        impl ::core::default::Default for UserModule {
            #[inline]
            fn default() -> UserModule {
                UserModule {}
            }
        }
        impl nidrs::Module for UserModule {
            fn init(self, mut ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
                use nidrs::{
                    Service, Controller, Interceptor, InterCtx, InterceptorHandler,
                    ModuleCtx, StateCtx, ImplMeta,
                };
                if ctx.modules.contains_key("UserModule") {
                    return ctx;
                }
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Registering module {0}.\n", "UserModule"),
                    );
                };
                ctx.modules.insert("UserModule".to_string(), Box::new(self));
                ctx.imports
                    .insert(
                        "UserModule".to_string(),
                        Vec::from(["AppModule".to_string()]),
                    );
                ctx.append_exports("UserModule", Vec::from(["UserService"]), false);
                if ctx
                    .register_controller(
                        "UserModule",
                        "UserController",
                        Box::new(
                            std::sync::Arc::new(controller::UserController::default()),
                        ),
                    )
                {
                    let t_controller = ctx
                        .get_controller::<
                            controller::UserController,
                        >("UserModule", "UserController");
                    ctx = t_controller.__route_get_hello_world(ctx);
                    let t_controller = ctx
                        .get_controller::<
                            controller::UserController,
                        >("UserModule", "UserController");
                    ctx = t_controller.__route_create_user(ctx);
                }
                let svc = std::sync::Arc::new(UserService::default());
                ctx.register_service("UserModule", "UserService", Box::new(svc));
                let mut ctx = AppModule::default().init(ctx);
                let t = ctx.get_service::<UserService>("UserModule", "UserService");
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Injecting {0}::{1}.\n",
                            "UserModule",
                            "UserService",
                        ),
                    );
                };
                let ctx = t.inject(ctx, &"UserModule");
                let t = ctx
                    .get_controller::<UserController>("UserModule", "UserController");
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Injecting {0}::{1}.\n",
                            "UserModule",
                            "UserController",
                        ),
                    );
                };
                let ctx = t.inject(ctx, &"UserModule");
                ctx
            }
            fn destroy(&self, ctx: &nidrs::ModuleCtx) {
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Destroying module {0}.\n", "UserModule"),
                    );
                };
            }
        }
        impl nidrs::ImplMeta for UserModule {
            fn __meta() -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set("service", "UserService");
                meta.set_data(nidrs::datasets::ServiceName::from("UserService"));
                meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                meta.set("module", "UserModule");
                meta.set("global", "app");
                meta
            }
        }
    }
}
mod shared {
    pub mod fn_test {
        use nidrs::externs::anyhow;
        use nidrs::externs::axum::http::StatusCode;
        use nidrs::{throw, Exception};
        use crate::AppResult;
        pub fn fn_test() -> AppResult {
            nidrs::__throw(
                Exception::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    anyhow::Error::msg("Error"),
                ),
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "from {0} line {1}",
                            "examples/hello/src/shared/fn_test.rs",
                            8usize,
                        ),
                    );
                    res
                }),
            )?;
            Ok(())
        }
    }
}
use std::time::Duration;
use nidrs::externs::axum::{
    error_handling::HandleErrorLayer, extract::Request, http::StatusCode,
    middleware::{self, Next},
    response::Response, BoxError,
};
use nidrs::externs::tower::timeout::TimeoutLayer;
pub use nidrs::AppError;
pub use nidrs::AppResult;
fn main() {
    let app = nidrs::NidrsFactory::create(app::AppModule);
    let app = app.default_prefix("/api/{version}");
    let app = app.default_version("v1");
    let app = app
        .default_layer(
            nidrs::externs::tower::ServiceBuilder::new()
                .layer(
                    HandleErrorLayer::new(|error: BoxError| async move {
                        if error.is::<nidrs::externs::tower::timeout::error::Elapsed>() {
                            Ok(StatusCode::REQUEST_TIMEOUT)
                        } else {
                            Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Unhandled internal error: {0}", error),
                                    );
                                    res
                                }),
                            ))
                        }
                    }),
                )
                .layer(TimeoutLayer::new(Duration::from_secs(5)))
                .layer(middleware::from_fn(auth)),
        );
    let app = app.listen(3000);
    app.block();
}
pub mod import {
    pub use crate::app::controller::AppController;
    pub use crate::app::interceptor::AppInterceptor;
    pub use crate::app::service::AppService;
    pub use crate::modules::user::service::UserService;
    pub use crate::modules::conf::options::ConfOptions;
    pub use crate::modules::user::controller::UserController;
    pub use crate::modules::conf::service::ConfService;
}
struct CurrentUser {
    pub id: u64,
    pub username: String,
}
#[automatically_derived]
impl ::core::clone::Clone for CurrentUser {
    #[inline]
    fn clone(&self) -> CurrentUser {
        CurrentUser {
            id: ::core::clone::Clone::clone(&self.id),
            username: ::core::clone::Clone::clone(&self.username),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CurrentUser {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "CurrentUser",
            "id",
            &self.id,
            "username",
            &&self.username,
        )
    }
}
async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    {
        ::std::io::_print(format_args!("auth {0:?}\n", req));
    };
    req.extensions_mut()
        .insert(CurrentUser {
            id: 1,
            username: "foo".to_string(),
        });
    Ok(next.run(req).await)
}
extern crate alloc;