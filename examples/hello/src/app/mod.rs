use nidrs::default_uses;
use nidrs::macros::module;

pub mod controller;
pub mod dto;
pub mod exception;
pub mod service;

use crate::modules::conf::ConfModule;
use crate::modules::conf::ConfOptions;
use crate::modules::log::LogModule;
use crate::modules::user::UserModule;
use controller::AppController;
use service::AppService;

#[default_uses(LogInterceptor)]
#[module({
    imports: [
        ConfModule::for_root(ConfOptions{
            log_level: "info".to_string(),
        }),
        LogModule,
        UserModule,
    ],
    // interceptors: [LogInterceptor],
    controllers: [AppController],
    services: [AppService],
    exports: [AppService],
})]
pub struct AppModule;
