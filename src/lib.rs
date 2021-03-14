use std::fs;
use std::io::prelude::*;
use std::error::Error;

pub fn create_project(name: &str)-> Result<(), Box<dyn Error>>  {
    create_project_tree(&name)?;
    create_readme_md_template(&name)?;
    create_cmakelists_template(&name)?;
    create_example_template(&name)?;
    create_config_file_template(&name)?;

    Ok(())
}

pub fn check_args(args: &[String])-> Result<&str, &str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }
    Ok(&args[1])
}

fn create_project_tree(name: &str) -> Result<(), Box<dyn Error>> {
    fs::create_dir(name)?;
    fs::create_dir(format!("{}/src", name))?;
    fs::create_dir(format!("{}/src/examples", name))?;
    fs::create_dir(format!("{}/include", name))?;
    fs::create_dir(format!("{}/tests", name))?;
    fs::create_dir(format!("{}/build", name))?;
    
    Ok(())
}

fn create_readme_md_template(name: &str) -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::create(format!("{}/README.md", name))?;
    let text = format!("# {}\nWrite something abount your project.", name);
    file.write(text.as_bytes())?;

    Ok(())
}

fn create_cmakelists_template(name: &str) -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::create(format!("{}/CMakeLists.txt", name))?;
    let text = format!("cmake_minimum_required(VERSION 3.15)\nproject({})\nset(CMAKE_CXX_STANDARD 17)\nif(WIN32)
set(COMPILE_OPTIONS)
set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS)
elseif(UNIX)
set(COMPILE_OPTIONS -Wall -Wextra -pendatic -Werror -fstack-protector)
set(CMAKE_BUILD_TYPE \"Debug\")
endif()

set(VERSION_MAJOR \"0\")
set(VERSION_MINOR \"1\")
set(VERSION_PATCH \"0\")
set(PROJECT_VERSION ${{VERSION_MAJOR}}.${{VERSION_MINOR}}.${{VERSION_PATCH}})
configure_file(\"config_version.h.in\"
    ${{CMAKE_SOURCE_DIR}}/include/config_version.h)

option(testing \"Execute tests\" OFF)

set(TESTS_INCLUDE_DIR ${{CMAKE_SOURCE_DIR}}/tests)

add_executable(example src/examples/example.cpp)
target_include_directories(example PRIVATE include)

if(testing)
add_subdirectory(tests)
endif()", name);
    file.write(text.as_bytes())?;

    Ok(())
}

fn create_example_template(name: &str) -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::create(format!("{}/src/examples/example.cpp", name))?;
    let text = format!("#include <iostream>

int main() {{
    std::cout << \"Hello World\";

    return 0;
}}");

    file.write(text.as_bytes())?;

    Ok(())
}

fn create_config_file_template(name: &str) -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::create(format!("{}/config_version.h.in", name))?;
    let text = format!("#ifndef _CONFIG_VERSION_H
#define _CONFIG_VERSION_H

#define VERSION_MAJOR \"@PROJECT_VERSION_MAJOR@\"
#define VERSION_MINOR \"@PROJECT_VERSION_MINOR@\"
#define VERSION_PATCH \"@PROJECT_VERSION_PATCH@\"
#define VERSION \"@PROJECT_VERSION@\"

const char* GetVersion() {{
    return VERSION;
}}

#endif");
    file.write(text.as_bytes())?;
    Ok(())
}