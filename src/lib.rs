use std::error::Error;
pub mod cxx_template;

pub struct Config {
    name: String,
    template_type: String,
}

pub trait Template {
    fn create_template(&self) -> Result<(), Box<dyn Error>>;
}

pub fn create_project(config: &Config)-> Result<(), Box<dyn Error>>  {
    if config.template_type.to_ascii_lowercase() == "cxx" {
        cxx_template::CXX::new(&config.name)?.
        create_template()?;
    }else {
        return Err("invalid project type.\nValid types are <cxx>.")?;
    }

    Ok(())
}

pub fn check_args(args: &[String])-> Result<Config, &str> {
    if args.len() < 3 {
        return Err("not enough arguments");
    }
    let name = &args[2];
    let template_type = &args[1];
    Ok(Config{
        name: name.to_string(),
        template_type: template_type.to_string()
    })
}

