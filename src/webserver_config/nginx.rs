pub struct Template {
    base: &'static str,
    prefetch: &'static str,
    staticfiles: &'static str,
    media: &'static str,   
}


pub static mut NGINXTEMPLATE: Template = Template {
    base: include_str!("../templates/nginx/main.conf"),
    staticfiles: include_str!("../templates/nginx/static.conf"),
    prefetch: include_str!("../templates/nginx/prefetch.conf"),
    media: include_str!("../templates/nginx/media.conf"),
    
};