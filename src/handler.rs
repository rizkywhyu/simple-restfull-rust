pub fn config(conf: &mut actix_web::web::ServiceConfig){
    let scope = actix_web::web::scope("/v1")
        .service(api::services::sample::send_msg);
    conf.service(scope);
} 
//2022-03-11 21:25:43