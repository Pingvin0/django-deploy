pub struct Template {
    base: &'static str,
    prefetch: &'static str,
    staticfiles: &'static str,
    media: &'static str,   
}



pub static mut APACHETEMPLATE: Template = Template {
    base: include_str!("../templates/apache/main.conf"),
    staticfiles: include_str!("../templates/apache/static.conf"),
    prefetch: include_str!("../templates/apache/prefetch.conf"),
    media: include_str!("../templates/apache/media.conf"),
    
};